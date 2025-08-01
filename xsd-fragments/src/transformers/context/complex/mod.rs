mod expand_restriction_fragments;
pub use expand_restriction_fragments::{
    Error as ExpandRestrictionFragmentsError, ExpandRestrictionFragments,
};
mod expand_extension_fragments;
pub use expand_extension_fragments::{
    Error as ExpandExtensionFragmentsError, ExpandExtensionFragments,
};
mod expand_groups;
pub use expand_groups::{Error as ExpandGroupsError, ExpandGroups};
mod expand_attribute_declarations;
pub use expand_attribute_declarations::{
    Error as ExpandAttributeDeclarationsError, ExpandAttributeDeclarations,
};
mod remove_prohibited_attributes;
pub use remove_prohibited_attributes::{
    Error as RemoveProhibitedAttributesError, RemoveProhibitedAttributes,
};