pub const FAN_OUT_OFFSET_V2: usize = 8;

pub const HASH_LEN_SHA1: usize = 20;

pub const HASHES_OFFSET_V2: usize = FAN_OUT_OFFSET_V2 + FAN_OUT_SIZE;

pub const FAN_OUT_SIZE: usize = 256 * 4;

pub const CRC_LEN: usize = 4;

pub const OFFSET_LEN: usize = 4;

pub const PACK_SIGN_LEN: usize = 4;
pub const PACK_VERSION_LEN: usize = 4;
pub const PACK_COUNT_LEN: usize = 4;
pub const PACK_OBJECTS_OFFSET: usize = PACK_SIGN_LEN + PACK_COUNT_LEN + PACK_VERSION_LEN;

pub const PACK_CHECKSUM_LEN: usize = 20;
