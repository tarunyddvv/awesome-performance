use std::{io::ErrorKind, os::unix::fs::OpenOptionsExt, process::ExitCode};

const FILENAME: &str = "my_file";

fn main() -> ExitCode {
    unsafe {
        libc::umask(0);
    }

    let result = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o644)
        .open(FILENAME);

    match result {
        Ok(file) => {
            // Equivalent to close(file_descriptor)
            drop(file);
            ExitCode::SUCCESS
        }
        Err(error) => {
            println!("There was a problem creating {FILENAME}");

            if error.kind() == ErrorKind::AlreadyExists {
                println!("The file already exists");
            } else {
                println!("Unknown error: {}", error.raw_os_error().unwrap_or(-1));
            };
            ExitCode::from(255)
        }
    }
}
