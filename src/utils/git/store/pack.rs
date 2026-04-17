use iced::futures::StreamExt;
use crate::utils::git::consts::{CRC_LEN, FAN_OUT_OFFSET_V2, FAN_OUT_SIZE, HASHES_OFFSET_V2, HASH_LEN_SHA1, OFFSET_LEN};

pub struct PackFiles {
    pub hash: String,
    pub idx: PackFileType,
    pub pack: PackFileType,
    pub rev: Option<PackFileType>,
}

impl PackFiles {
    pub fn get_fanout_as_bytes(&self) -> Option<&[u8]> {
        self.idx.get_fanout_as_bytes()
    }
}

#[derive(Debug, Clone)]
pub enum PackFileType {
    Idx(Vec<u8>),
    Pack(Vec<u8>),
    Rev(Vec<u8>),
    Crash,
}

impl PackFileType {
    pub fn as_slice(&self) -> Option<&[u8]> {
        match self {
            PackFileType::Idx(v) | PackFileType::Pack(v) | PackFileType::Rev(v) => Some(v),
            PackFileType::Crash => None,
        }
    }

    pub fn get_fanout_as_bytes(&self) -> Option<&[u8]> {
        match self {
            PackFileType::Idx(v) => {
                let idx = &v[FAN_OUT_OFFSET_V2..];
                Some(&idx[..FAN_OUT_SIZE])
            }
            _ => None,
        }
    }

    pub fn get_part_of_hashes_table(&self, start: usize, end: usize) -> Option<Vec<String>> {
        match self {
            PackFileType::Idx(v) => {
                let idx = &v[HASHES_OFFSET_V2 + start * HASH_LEN_SHA1
                    ..HASHES_OFFSET_V2 + end * HASH_LEN_SHA1];
                let hashes: Vec<String> = idx
                    .chunks(20)
                    .map(|chunk| chunk.iter().map(|b| format!("{:02x}", b)).collect())
                    .collect();
                Some(hashes)
            }
            _ => None,
        }
    }

    pub fn get_offsets_table(&self, size: usize) -> Option<Vec<u32>> {
        match self {
            PackFileType::Idx(v) => {
                let idx = &v[HASHES_OFFSET_V2 + size * (CRC_LEN + HASH_LEN_SHA1)
                    ..HASHES_OFFSET_V2 + size * (CRC_LEN + HASH_LEN_SHA1 + OFFSET_LEN)];
                let offsets: Vec<u32> = idx
                    .chunks(4)
                    .map(|chunk|{
                        let arr:[u8;4] = chunk.try_into().unwrap();
                        u32::from_be_bytes(arr)
                    }).collect();
                Some(offsets)
            }
            _ => None,
        }
    }
}
