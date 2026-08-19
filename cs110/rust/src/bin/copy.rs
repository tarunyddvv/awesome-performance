use std::{
    env,
    fs::{File, OpenOptions},
    io::{Error, ErrorKind, Read, Write},
    os::unix::fs::OpenOptionsExt,
    process::ExitCode,
};

const WRONG_ARGUMENT_COUNT: u8 = 4;
const READ_FAILURE: u8 = 8;
const WRITE_FAILURE: u8 = 16;

fn main() -> ExitCode {
    let args: Vec<_> = env::args_os().collect();

    if args.len() != 3 {
        eprintln!(
            "{} <source-file> <destination-file>",
            args[0].to_string_lossy()
        );
        return ExitCode::from(WRONG_ARGUMENT_COUNT);
    }

    let mut input = match File::open(&args[1]) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("{}: {error}", args[1].to_string_lossy());
            return ExitCode::from(READ_FAILURE);
        }
    };

    let mut output = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o644)
        .open(&args[2])
    {
        Ok(file) => file,
        Err(error) => {
            eprintln!("{}: {error}", args[2].to_string_lossy());
            return ExitCode::from(WRITE_FAILURE);
        }
    };

    loop {
        let mut buffer = [0u8; 1024];

        let bytes_read = match input.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes) => bytes,
            Err(error) => {
                eprintln!("read: {error}");
                return ExitCode::from(READ_FAILURE);
            }
        };

        let mut bytes_written = 0;

        while bytes_written < bytes_read {
            match output.write(&buffer[bytes_written..bytes_read]) {
                Ok(0) => {
                    eprintln!(
                        "{}",
                        Error::new(ErrorKind::WriteZero, "write made no progress")
                    );
                    return ExitCode::from(WRITE_FAILURE);
                }
                Ok(bytes) => {
                    bytes_written += bytes;
                }
                Err(error) => {
                    eprintln!("write: {error}");
                    return ExitCode::from(WRITE_FAILURE);
                }
            }
        }
    }

    // Files are automatically closed when input and output go out of scope.
    ExitCode::SUCCESS
}
