use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use crate::headers::{FILE_INFO_HEADER_BYTES_SIZE, SESSION_INFO_HEADER_BYTES_SIZE, SessionInfo, FileInfo};

pub trait ReadSeek: Read + Seek {}
impl<T: Read + Seek> ReadSeek for T {}

pub struct Parser<'ibt> {
    pub ibt_file_path: &'ibt String,
    box_ibt_file: Box<dyn ReadSeek>,

}

impl<'ibt> Parser<'ibt> {
    pub fn new(ibt: &'ibt String) -> Self {
        Parser {
            ibt_file_path: ibt,
            box_ibt_file: create_box_ibt_file(ibt),
        }
    }

    pub fn file_info(&mut self) -> FileInfo {
        // Read General info bytes and convert to GeneralInfo type
        let file_info_bytes = read_file_bytes(
            &mut self.box_ibt_file,
            0,
            FILE_INFO_HEADER_BYTES_SIZE
        ).unwrap();

        FileInfo::from(file_info_bytes)
    }

    pub fn session_info(&mut self) -> SessionInfo {
        // Read Timinginfo bytes and convert to TimingInfo Type
        let session_info_bytes = read_file_bytes(
            &mut self.box_ibt_file,
            FILE_INFO_HEADER_BYTES_SIZE,
            SESSION_INFO_HEADER_BYTES_SIZE
        ).unwrap();

        SessionInfo::from(session_info_bytes)
    }
}

fn create_box_ibt_file(ibt_file_path: &String) -> Box<dyn ReadSeek> {
    let file = File::open(&ibt_file_path).unwrap();
    // Create Box type for entire file (writes to heap)
    Box::new(file)
}

fn read_file_bytes(whole_file: &mut Box<dyn ReadSeek>, from: usize, header_size: usize) -> Result<Vec<u8>, ()> {
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