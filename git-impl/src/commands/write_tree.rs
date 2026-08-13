use anyhow::Context;
use sha1::{Digest, Sha1};
use std::{fs, io::Write, os::unix::fs::PermissionsExt, path::Path};

use crate::objects::Object;

fn write_tree_for(path: &Path) -> anyhow::Result<Option<[u8; 20]>> {
    let mut dir =
        fs::read_dir(path).with_context(|| format!("open directory {}", path.display()))?;

    let mut tree_object = Vec::new();
    while let Some(entry) = dir.next() {
        let entry = entry.with_context(|| format!("bad directory entry in {}", path.display()))?;
        let file_name = entry.file_name();
        let meta = entry.metadata().context("metadata for directory entry")?;
        let mode = if meta.is_dir() {
            "40000"
        } else if meta.is_symlink() {
            "120000"
        } else if (meta.permissions().mode() & 0o111) != 0 {
            "100755"
        } else {
            "100644"
        };
        let path = entry.path();
        let hash = if meta.is_dir() {
            let Some(hash) = write_tree_for(&entry.path())? else {
                continue;
            };
            hash
        } else {
            let tmp = "temporary";
            let hash = Object::blob_from_file(&path)
                .context("open blob input file")?
                .write(std::fs::File::create(tmp).context("construct temporary file for blob")?)
                .context("stream file into blob")?;
            let hash_hex = hex::encode(hash);
            std::fs::create_dir_all(format!(".git/objects/{}/", &hash_hex[..2]))
                .context("create subdir of .git/objects")?;
            std::fs::rename(
                tmp,
                format!(".git/objects/{}/{}", &hash_hex[..2], &hash_hex[2..]),
            )?;

            hash
        };

        tree_object.extend(mode.as_bytes());
        tree_object.push(b' ');
        tree_object.extend(file_name.as_encoded_bytes());
        tree_object.push(0);
        tree_object.extend(hash);
    }
    Ok(None)
}

pub fn invoke() -> anyhow::Result<()> {
    // fn write_blob(writer: impl Write, file: PathBuf) -> anyhow::Result<String> {
    //     let stat = std::fs::metadata(&file)
    //         .with_context(|| format!("file metadata: {}", file.display()))?;

    //     let e = ZlibEncoder::new(writer, Compression::default());

    //     let mut writer = HashWriter {
    //         hasher: Sha1::new(),
    //         writer: e,
    //     };
    //     let content = std::fs::read(&file).context("reading the contents of the file")?;
    //     write!(writer, "blob ")?;
    //     write!(writer, "{}\0", stat.len())?;
    //     writer.write_all(&content)?;

    //     writer.writer.finish()?;
    //     let hash = writer.hasher.finalize();

    //     Ok(hex::encode(hash))
    // }

    // let hash = if write {
    //     let temp = "temporary";
    //     let hash = write_blob(std::fs::File::create(temp)?, file)?;

    //     std::fs::create_dir_all(format!("../.git/objects/{}/", &hash[..2]))?;
    //     std::fs::rename(
    //         temp,
    //         format!("../.git/objects/{}/{}", &hash[..2], &hash[2..]),
    //     )?;

    //     hash
    // } else {
    //     write_blob(std::io::sink(), file)?
    // };

    // println!("{hash}");

    Ok(())
}

struct HashWriter<W> {
    hasher: Sha1,
    writer: W,
}

impl<W> Write for HashWriter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = self.writer.write(buf)?;
        self.hasher.update(&buf[..n]);

        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
