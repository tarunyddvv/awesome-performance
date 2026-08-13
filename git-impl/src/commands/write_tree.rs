use crate::objects::{Kind, Object};
use anyhow::{Context, bail, ensure};
use std::{collections::BTreeMap, fs, io::Cursor};

struct IndexEntry {
    path: Vec<u8>,
    mode: u32,
    hash: [u8; 20],
}

fn read_u32(bytes: &[u8]) -> anyhow::Result<u32> {
    let value = bytes.get(..4).context("truncated index")?;
    Ok(u32::from_be_bytes(value.try_into().unwrap()))
}

fn read_index() -> anyhow::Result<Vec<IndexEntry>> {
    let index = Object::git_dir()?.join("index");
    let bytes = fs::read(&index).with_context(|| format!("read {}", index.display()))?;
    ensure!(bytes.len() >= 12, "git index is truncated");
    ensure!(&bytes[..4] == b"DIRC", "invalid git index signature");
    ensure!(read_u32(&bytes[4..])? == 2, "unsupported git index version");

    let count = read_u32(&bytes[8..])? as usize;
    let mut offset = 12;
    let mut entries = Vec::with_capacity(count);
    for _ in 0..count {
        ensure!(offset + 62 <= bytes.len(), "truncated git index entry");
        let mode = read_u32(&bytes[offset + 24..])?;
        let mut hash = [0; 20];
        hash.copy_from_slice(&bytes[offset + 40..offset + 60]);
        let flags = u16::from_be_bytes(bytes[offset + 60..offset + 62].try_into().unwrap());
        ensure!(
            flags & 0x3000 == 0,
            "cannot write an index with unmerged entries"
        );

        let name_start = offset + 62;
        let name_end = bytes[name_start..]
            .iter()
            .position(|&byte| byte == 0)
            .map(|length| name_start + length)
            .context("unterminated git index path")?;
        let path = bytes[name_start..name_end].to_vec();
        let entry_size = (name_end + 1 - offset + 7) & !7;
        offset += entry_size;
        entries.push(IndexEntry { path, mode, hash });
    }
    Ok(entries)
}

fn write_tree(entries: &[&IndexEntry], prefix: &[u8]) -> anyhow::Result<[u8; 20]> {
    let mut files = Vec::new();
    let mut directories: BTreeMap<Vec<u8>, Vec<&IndexEntry>> = BTreeMap::new();

    for entry in entries {
        let path = entry
            .path
            .strip_prefix(prefix)
            .context("index path is outside tree prefix")?;
        if let Some(separator) = path.iter().position(|&byte| byte == b'/') {
            let name = path[..separator].to_vec();
            let child_prefix = [prefix, &name, b"/"].concat();
            directories.entry(child_prefix).or_default().push(entry);
        } else {
            files.push((path.to_vec(), entry));
        }
    }

    let mut tree_entries: Vec<(Vec<u8>, u32, [u8; 20])> = files
        .into_iter()
        .map(|(name, entry)| (name, entry.mode, entry.hash))
        .collect();
    for (child_prefix, child_entries) in directories {
        let name = child_prefix[prefix.len()..child_prefix.len() - 1].to_vec();
        tree_entries.push((name, 0o040000, write_tree(&child_entries, &child_prefix)?));
    }

    tree_entries.sort_by(|(a_name, a_mode, _), (b_name, b_mode, _)| {
        let mut a_key = a_name.clone();
        if a_mode == &0o040000 {
            a_key.push(b'/');
        }
        let mut b_key = b_name.clone();
        if b_mode == &0o040000 {
            b_key.push(b'/');
        }
        a_key.cmp(&b_key)
    });

    let mut content = Vec::new();
    for (name, mode, hash) in tree_entries {
        content.extend(format!("{:o} ", mode).as_bytes());
        content.extend(name);
        content.push(0);
        content.extend(hash);
    }

    Object {
        kind: Kind::Tree,
        expected_size: content.len() as u64,
        reader: Cursor::new(content),
    }
    .write_to_objects()
    .context("write tree object")
}

pub fn invoke() -> anyhow::Result<()> {
    let entries = read_index()?;
    if entries.is_empty() {
        bail!("asked to make tree object for empty index");
    }
    let entries: Vec<_> = entries.iter().collect();
    println!("{}", hex::encode(write_tree(&entries, &[])?));
    Ok(())
}
