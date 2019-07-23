extern crate cast_rs;
use cast_rs::vec_t;

fn main() {
    let v: Vec<u8> = vec![72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100];
    let s: &str = vec_t::to_str(&v).unwrap();
    assert_eq!("Hello world", s);

    let string: String = vec_t::to_string(&v);
    assert_eq!("Hello world".to_string(), string);

    let array: &[u8] = vec_t::to_array(&v);
    assert_eq!([72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100], array);
}
