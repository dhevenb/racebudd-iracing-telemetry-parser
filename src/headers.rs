pub const FILE_INFO_HEADER_BYTES_SIZE: usize = 112;
pub const SESSION_INFO_HEADER_BYTES_SIZE: usize = 32;
pub const VAR_HEADER_BYTES_SIZE: usize = 144;

// For data types in the different iRacing Telemetry files ref: https://sajax.github.io/irsdkdocs/telemetry/

// File specific information 
pub struct  FileInfo {
    pub version: i32,
    pub status: i32,
    pub tick_rate: i32,
    pub sesion_info_update: i32,
    pub sesion_info_offset: i32,
    pub sesion_info_length: i32,
    pub num_vars: i32,
    pub var_header_offset: i32,
    pub num_buf: i32,
    pub buf_len: i32,
    pub buf_offset: i32,
}

// Session specific information
pub struct SessionInfo {
    pub start_date: f32,
    pub start_time: f64,
    pub end_time: f64,
    pub lap_count: i32,
    pub record_count: i32,
}

// Additional metadata about variables recorded during the session
pub struct VarHeader {
    pub r#type: i32,
    pub offset: i32,
    pub count: i32,
    pub count_as_time: i8,
    pub name: String,
    pub description: String,
    pub unit: String,
}

impl From<Vec<u8>> for FileInfo {
    fn from(buffer: Vec<u8>) -> Self {
        FileInfo {
            version: i32::from_le_bytes(buffer[0..4].try_into().unwrap()),
            status: i32::from_le_bytes(buffer[4..8].try_into().unwrap()),
            tick_rate: i32::from_le_bytes(buffer[8..12].try_into().unwrap()),
            sesion_info_update: i32::from_le_bytes(buffer[12..16].try_into().unwrap()),
            sesion_info_length: i32::from_le_bytes(buffer[16..20].try_into().unwrap()),
            sesion_info_offset: i32::from_le_bytes(buffer[20..24].try_into().unwrap()),
            num_vars: i32::from_le_bytes(buffer[24..28].try_into().unwrap()),
            var_header_offset: i32::from_le_bytes(buffer[28..32].try_into().unwrap()),
            num_buf: i32::from_le_bytes(buffer[32..36].try_into().unwrap()),
            buf_len: i32::from_le_bytes(buffer[36..40].try_into().unwrap()),
            buf_offset: i32::from_le_bytes(buffer[52..56].try_into().unwrap()),
        }
    }
}

impl From<Vec<u8>> for SessionInfo {
    fn from(buffer: Vec<u8>) -> Self {
        SessionInfo {
            start_date: f32::from_le_bytes(buffer[0..4].try_into().unwrap()),
            start_time: f64::from_le_bytes(buffer[8..16].try_into().unwrap()),
            end_time: f64::from_le_bytes(buffer[16..24].try_into().unwrap()),
            lap_count: i32::from_le_bytes(buffer[24..28].try_into().unwrap()),
            record_count: i32::from_le_bytes(buffer[28..32].try_into().unwrap()),
        }
    }
}

