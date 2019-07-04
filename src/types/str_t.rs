pub fn to_string(s: &str) -> String {
    String::from(s)
}

pub fn to_array(s: &str) -> &[u8] {
    s.as_bytes()
}

pub fn to_vec(s: &str) -> Vec<u8> {
    s.as_bytes().to_owned()
}

