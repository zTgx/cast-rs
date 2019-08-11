pub mod types;

pub use crate::types::array_t   as array_t;
pub use crate::types::str_t     as str_t;
pub use crate::types::string_t  as string_t;
pub use crate::types::vec_t     as vec_t;

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
