extern crate hex;
extern crate downcast_rs;
extern crate slice_cast;

pub mod hexcast {
    pub use hex::*;
}

pub mod downcast {
     pub use downcast_rs::*;
}

pub mod slicecast {
    pub use slice_cast::*;
}
