use std::fs::File;

use crate::headers::{FILE_INFO_HEADER_BYTES_SIZE, SESSION_INFO_HEADER_BYTES_SIZE, VAR_INFO_BYTES_SIZE, FileInfo, SessionInfo, VarInfo};
use crate::telemetry_data::Ticks;
use crate::utils::{ReadSeek, read_file_bytes};

pub struct Parser {
    box_ibt_file: Box<dyn ReadSeek>,
}

impl<'ibt> Parser {
    pub fn new(ibt: &'ibt str) -> Self {
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

    pub fn channels(&mut self) -> Vec<VarInfo> {
        let num_vars = self.file_info().num_vars;
        let var_header_buf_size = num_vars as usize * VAR_INFO_BYTES_SIZE;
        let var_header_offset = self.file_info().var_header_offset as usize;
        
        let channel_data = read_file_bytes(
            &mut self.box_ibt_file,
            var_header_offset,
            var_header_buf_size,
        ).unwrap();

        let channels: Vec<VarInfo> = (0..num_vars)
            .map(|n| {
                let start = n as usize * VAR_INFO_BYTES_SIZE;
                let end = start + VAR_INFO_BYTES_SIZE;
                VarInfo::from(channel_data[start..end].to_vec())
            })
            .collect();

        channels
    }

    pub fn telemetry_data(&mut self) -> Ticks {
        let tick_length = self.file_info().buf_len;
        let buf_offset = self.file_info().buf_offset;
        let channels = self.channels();

        Ticks {
            file: &mut self.box_ibt_file,
            channels: channels,
            tick_length: tick_length,
            buf_offset: buf_offset,
            tick_number: 0,
        }
    }
}

fn create_box_ibt_file(ibt_file_path: &str) -> Box<dyn ReadSeek> {
    let file = File::open(&ibt_file_path).unwrap();
    // Create Box type for entire file (writes to heap)
    Box::new(file)
}