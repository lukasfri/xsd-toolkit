mod xs_custom;
mod xs_generated;

pub use xs_custom::elements::Facet;
pub use xs_generated::*;

pub mod types {
    pub use super::xs_custom::types::{List, QName, TargetNamespace};
    pub use super::xs_generated::types::*;
}
