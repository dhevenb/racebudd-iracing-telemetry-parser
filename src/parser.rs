use std::fs::File;

use crate::headers::{FILE_INFO_HEADER_BYTES_SIZE, SESSION_INFO_HEADER_BYTES_SIZE, SessionInfo, FileInfo};
use crate::utils::{ReadSeek, read_file_bytes};

pub struct Parser {
    box_ibt_file: Box<dyn ReadSeek>,

}

impl<'ibt> Parser {
    pub fn new(ibt: &'ibt String) -> Self {
        Parser {
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