use std::str;
use std::str::Utf8Error;

pub fn to_str<'a> (v: &'a Vec<u8>) -> Result<&'a str, Utf8Error> {
    str::from_utf8(&v)
}

pub fn to_string<'a> (v: &'a Vec<u8>) -> String {
    v.iter().map(|c| { *c as char }).collect::<String>()
}

pub fn to_array<'a> (v: &'a Vec<u8>) -> &'a [u8] {
    v.as_slice()
}

pub fn to_1_array<'a> (v: &'a Vec<u8>) -> Option<[u8; 1]> {
    if v.len() != 1 { return None; }
    let mut x: [u8; 1] = [0u8; 1];
    let mut index = 0;
    for &item in v.iter() {
        x[index] = item;

        index += 1;
    }

    Some(x)
}

pub fn to_8_array<'a> (v: &'a Vec<u8>) -> Option<[u8; 8]> {
    if v.len() != 8 { return None; }
    let mut x: [u8; 8] = [0u8; 8];
    let mut index = 0;
    for &item in v.iter() {
        x[index] = item;

        index += 1;
    }

    Some(x)
}


