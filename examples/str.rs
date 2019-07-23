extern crate cast_rs;
use cast_rs::str_t;

fn main() {
    let x: &str = "Hello world";
    let s: String = str_t::to_string(x);
    assert_eq!("Hello world".to_string(), s);
}
