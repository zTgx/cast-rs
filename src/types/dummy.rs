use std::any::Any;

trait A {
    fn as_any(&self) -> &dyn Any;
}

struct B;

impl A for B {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/*
fn main() {
    let a: Box<dyn A> = Box::new(B);
    // The indirection through `as_any` is because using `downcast_ref`
    // on `Box<A>` *directly* only lets us downcast back to `&A` again.
    // The method ensures we get an `Any` vtable that lets us downcast
    // back to the original, concrete type.
    let b: &B = match a.as_any().downcast_ref::<B>() {
        Some(b) => b,
            None => panic!("&a isn't a B!"),
    };
}
*/
