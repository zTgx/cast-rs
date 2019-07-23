extern crate cast_rs;
use cast_rs::string_t;

fn main() {
    let x: String = "Hello world".to_string();
    let s: &str = string_t::to_str(&x);
    assert_eq!("Hello world", s);


}
