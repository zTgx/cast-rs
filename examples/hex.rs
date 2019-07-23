extern crate cast_rs;
use cast_rs::hex_t;

fn main() {
    assert_eq!(
            hex_t::decode("48656c6c6f20776f726c6421"),
            Ok("Hello world!".to_owned().into_bytes())
            );

   assert_eq!(hex_t::encode(vec![1, 2, 3, 15, 16]), "0102030f10");

   assert_eq!(hex_t::encode("Hello world!"), "48656c6c6f20776f726c6421");
}
