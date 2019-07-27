extern crate cast_rs;
use cast_rs::str_t;

fn main() {
    let x: &str = "Hello world";
    let s: String = str_t::to_string(x);
    assert_eq!("Hello world".to_string(), s);

    assert_eq!(Ok("1.122e1".to_string()),  str_t::to_expo("11.22"));
    assert_eq!(Ok("1.1e1".to_string()),    str_t::to_expo("11."));
    assert_eq!(Ok("2.2e-1".to_string()),   str_t::to_expo(".22"));
    assert_eq!(Ok("2.2e1".to_string()),    str_t::to_expo("22"));
    assert_eq!(Err("invalid input!"),   str_t::to_expo("ff"));


    assert_eq!(-1isize, str_t::get_exponent("2.2e-1"));
}
