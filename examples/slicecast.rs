extern crate cast_rs;

fn main() {
    let foo: [u8; 4] = [1, 0, 0, 0];
    let bar: &u32 = unsafe { slice_cast::cast_to(&foo) };
    println!("{}", bar);
}
