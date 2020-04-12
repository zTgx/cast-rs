// Import macro via `macro_use` pre-1.30.

use cast_rs::downcast_rs::*;

// To create a trait with downcasting methods, extend `Downcast` and run
// `impl_downcast!()` on the trait.
trait Base: Downcast {}
impl_downcast!(Base);

// Concrete types implementing Base.
#[derive(Debug)]
struct Foo(u32);
impl Base for Foo {}
#[derive(Debug)]
struct Bar(f64);
impl Base for Bar {}

fn main() {
    // Create a trait object.
    let base: Box<dyn Base> = Box::new(Foo(42));

    // Try sequential downcasts.
    if let Some(foo) = base.downcast_ref::<Foo>() {
        assert_eq!(foo.0, 42);
    } else if let Some(bar) = base.downcast_ref::<Bar>() {
        assert_eq!(bar.0, 42.0);
    }

    assert!(base.is::<Foo>());

    // Fail to convert `Box<Base>` into `Box<Bar>`.
    let res = base.downcast::<Bar>();
    assert!(res.is_err());
    let base = res.unwrap_err();
    // Convert `Box<Base>` into `Box<Foo>`.
    assert_eq!(42, base.downcast::<Foo>().map_err(|_| "Shouldn't happen.").unwrap().0);
}
