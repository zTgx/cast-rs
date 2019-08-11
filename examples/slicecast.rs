use cast_rs::slicecast;

fn main() {
    let foo: [u8; 4] = [1, 0, 0, 0];
    let bar: &u32 = unsafe { slicecast::cast_to(&foo) };
    println!("{}", bar);
}
