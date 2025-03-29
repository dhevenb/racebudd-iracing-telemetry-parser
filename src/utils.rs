use std::io::{Read, Seek, SeekFrom};

pub trait ReadSeek: Read + Seek {}
impl<T: Read + Seek> ReadSeek for T {}

pub fn read_file_bytes(whole_file: &mut Box<dyn ReadSeek>, from: usize, header_size: usize) -> Result<Vec<u8>, ()> {
    // Create new buffer preloaded with 0 at the specified header size
    let mut buffer: Vec<u8> = vec![0; header_size];

    // Attempt to read whole_file from to header_size and assign it to buffer
    &whole_file.seek(SeekFrom::Start(from as u64)).unwrap();

    // Read from current file pointer
    match whole_file.read_exact(&mut buffer).map_err(|_e| ()) {
        Ok(_) => Ok(buffer),
        Err(_) => Err(()) 
    }
}
