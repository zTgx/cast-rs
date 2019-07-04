extern crate hex;
use hex::FromHexError;

pub fn encode<T: AsRef<[u8]>>(data: T) -> String {
    hex::encode(data)
}

pub fn decode<T: AsRef<[u8]>>(data: T) -> Result<Vec<u8>, FromHexError> {
    hex::decode(data)
}
