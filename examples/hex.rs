use cast_rs::hexcast;

fn main() {
    assert_eq!(
            hexcast::decode("48656c6c6f20776f726c6421"),
            Ok("Hello world!".to_owned().into_bytes())
            );

   assert_eq!(hexcast::encode(vec![1, 2, 3, 15, 16]), "0102030f10");

   assert_eq!(hexcast::encode("Hello world!"), "48656c6c6f20776f726c6421");
}
