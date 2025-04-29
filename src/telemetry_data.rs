use crate::utils::{read_file_bytes, ReadSeek};
use crate::headers::VarInfo;

#[derive(Debug)]
pub struct Ticks<'a> {
    pub file: &'a mut Box<dyn ReadSeek>,
    pub channels: Vec<VarInfo>,
    pub tick_length: i32,
    pub buf_offset: i32,
    pub tick_number: i32,
}

impl<'a> Iterator for Ticks<'a> {
    type Item = Tick;

    fn next(&mut self) -> Option<Self::Item> {
        let tick_start_pos = self.buf_offset + (self.tick_number * self.tick_length);

        match read_file_bytes(self.file, tick_start_pos as usize, self.tick_length as usize) {
            Ok(bytes) => {
                self.tick_number += 1;
                let channels = self.channels.clone();

                Some(Tick {
                    bytes,
                    channels,
                })
            },
            Err(_) => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tick {
    bytes: Vec<u8>,
    channels: Vec<VarInfo>,
}

impl Tick {
    pub fn get_value(&self, var: &VarInfo) -> Option<ChannelValue> {
        let offset = var.offset as usize;
        match var.var_type {
            0 => {
                let size = 1;
                Some(ChannelValue::Char(self.bytes[offset + size] as char))
            }
            1 => {
                let size = 1;
                Some(ChannelValue::Bool(self.bytes[offset + size] != 0))
            }
            2 => {
                let size = 4;
                let value =
                    i32::from_le_bytes(self.bytes[offset..(offset + size)].try_into().unwrap());
                Some(ChannelValue::Int(value))
            }
            3 => {
                let size = 4;
                let value =
                    u32::from_le_bytes(self.bytes[offset..(offset + size)].try_into().unwrap());
                Some(ChannelValue::BitField(value))
            }
            4 => {
                let size = 4;
                let value =
                    f32::from_le_bytes(self.bytes[offset..(offset + size)].try_into().unwrap());
                Some(ChannelValue::Float32(value))
            }
            5 => {
                let size = 8;
                let value =
                    f64::from_le_bytes(self.bytes[offset..(offset + size)].try_into().unwrap());
                Some(ChannelValue::Float64(value))
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub enum ChannelValue {
    Char(char),
    Bool(bool),
    Int(i32),
    BitField(u32),
    Float32(f32),
    Float64(f64),
}

impl ChannelValue {
    pub fn char(&self) -> char {
        if let ChannelValue::Char(x) = self {
            *x
        } else {
            panic!()
        }
    }

    pub fn bool(&self) -> bool {
        if let ChannelValue::Bool(x) = self {
            *x
        } else {
            panic!()
        }
    }

    pub fn float_32(&self) -> f32 {
        if let ChannelValue::Float32(x) = self {
            *x
        } else {
            panic!()
        }
    }

    pub fn float_64(&self) -> f64 {
        if let ChannelValue::Float64(x) = self {
            *x
        } else {
            panic!()
        }
    }

    pub fn int(&self) -> i32 {
        if let ChannelValue::Int(x) = self {
            *x
        } else {
            panic!()
        }
    }

    pub fn bitfield(&self) -> u32 {
        if let ChannelValue::BitField(x) = self {
            *x
        } else {
            panic!()
        }
    }
}