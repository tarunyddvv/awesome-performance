fn main() {
    let old_mask = unsafe { libc::umask(0) };

    unsafe { libc::umask(old_mask) };

    println!("umask is set to {:03o}", old_mask);
}
