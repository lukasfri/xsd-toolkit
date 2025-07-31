mod xs_custom;
mod xs_generated;

pub use xs_generated::*;

pub mod types {
    pub use super::xs_custom::types::QName;
    pub use super::xs_generated::types::*;
}
