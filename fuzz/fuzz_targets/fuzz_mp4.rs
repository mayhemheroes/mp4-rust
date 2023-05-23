use honggfuzz::fuzz;
use std::io::{Cursor};
use mp4::{Result};

fn process_data(data: &[u8]) -> Result<()> {
    let size = data.len() as u64;
    let reader = Cursor::new(data);

    let _mp4 = match mp4::Mp4Reader::read_header(reader, size) {
        Ok(_mp4) => _mp4,
        Err(_) => return Ok(()), // Ignore parsing errors, as we're fuzzing
    };

    Ok(())
}

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(s) = std::str::from_utf8(data) {
                let _ = process_data(data);
            }
        });
    }
}