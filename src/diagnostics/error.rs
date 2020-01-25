use std::fs;
use std::io;
use std::io::Write; // for the trait implementation

/// The destination file of error messages.
static mut DESTINATION : Option<&'static str> = None;

/// Whether to clear the error log.
static mut CLEAR : bool = true;

/// Reports an error to the destination file.
pub fn report(message : &str) -> io::Result<()> {
    unsafe {
        if let Some(path) = DESTINATION {
            if CLEAR {
                let _ = fs::remove_file(path);
                CLEAR = false;
            }
            let mut out = fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(true)
                    .open(path)?;
            out.write(format!("{}\n", message).as_bytes())?;
        }
    }
    Ok(())
}

// Sets the file destination.
pub fn set_destination(destination : Option<&'static str>) {
    unsafe {
        DESTINATION = destination;
    }
}

// Tells the logger to clear the file next time it is written to.
pub fn clear() {
    unsafe {
        CLEAR = true;
    }
}