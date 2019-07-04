use std::str;

pub fn to_str(arr: &[u8]) -> &str {
    str::from_utf8(&arr).unwrap()
}

pub fn to_string(arr: &[u8]) -> String {
    String::from_utf8(arr.to_vec()).unwrap()
}

pub fn to_vec(arr: &[u8]) -> Vec<u8> {
    arr.to_vec()
}

