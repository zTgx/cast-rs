pub fn to_str<'a> (s: &'a String) -> &'a str {
    s.as_str()
}

pub fn to_array<'a> (s: &'a String) -> &'a [u8] {
    s.as_bytes()
}

pub fn to_vec<'a> (s: &'a String) -> Vec<u8> {
    s.as_bytes().to_vec()
}

