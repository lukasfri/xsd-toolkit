mod xs_custom;
mod xs_generated;

pub mod xs {
    pub use super::xs_custom::elements::Facet;
    pub use super::xs_generated::*;

    pub mod types {
        pub use super::super::xs_custom::types::{List, QName, TargetNamespace};
        pub use super::super::xs_generated::types::*;
    }
}
