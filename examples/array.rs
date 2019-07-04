extern crate cast_rs;
use cast_rs::types::array_t;

fn main() {
    let arr: &[u8] = &[72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]; //Hello world
    let s: &str = array_t::to_str(&arr);
    assert_eq!("Hello world", s);

    let string: String = array_t::to_string(&arr);
    assert_eq!("Hello world".to_string(), string);

    let v: Vec<u8> = array_t::to_vec(&arr);
    assert_eq!(vec![72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100], v);
}