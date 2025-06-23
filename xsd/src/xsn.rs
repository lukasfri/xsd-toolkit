use std::sync::LazyLock;

use xmlity::{ExpandedName, LocalName, XmlNamespace};

macro_rules! xs_name {
    ($local_name:expr) => {
        ExpandedName::new(
            LocalName::new_dangerous($local_name),
            Some(XmlNamespace::XS),
        )
    };
}

pub static ANY_TYPE: LazyLock<ExpandedName<'static>> = LazyLock::new(|| xs_name!("anyType"));
pub static SIMPLE_ANY_TYPE: LazyLock<ExpandedName<'static>> =
    LazyLock::new(|| xs_name!("anySimpleType"));

pub static INTEGER: LazyLock<ExpandedName<'static>> = LazyLock::new(|| xs_name!("integer"));

pub static STRING: LazyLock<ExpandedName<'static>> = LazyLock::new(|| xs_name!("string"));

pub static NMTOKEN: LazyLock<ExpandedName<'static>> = LazyLock::new(|| xs_name!("NMTOKEN"));

pub static ID: LazyLock<ExpandedName<'static>> = LazyLock::new(|| xs_name!("ID"));

pub static NON_NEGATIVE_INTEGER: LazyLock<ExpandedName<'static>> =
    LazyLock::new(|| xs_name!("nonNegativeInteger"));
