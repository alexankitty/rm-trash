use std::path::Path;
use urlencoding::{decode, encode};

pub fn encode_path(path: &Path) -> String {
    encode(path.to_string_lossy().as_ref()).into_owned()
}

pub fn decode_path(path: &str) -> String {
    decode(path).expect("UTF-8").to_string()
}
