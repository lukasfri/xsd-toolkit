//! This module contains the logic for compiling complex types into fragments.
//!
//! It is naturally dependent on the simple type compiler, as complex types can contain simple types.

use std::{
    any::type_name,
    collections::{BTreeMap, VecDeque},
    ops::Deref,
};

use crate::{
    fragments::{
        simple::{self, SimpleFragmentEquivalent, SimpleTypeFragmentCompiler},
        Context, FragmentAccess, FragmentCollection, FragmentIdx, FragmentedXsdDocumentIdx,
        HasFragmentCollection,
    },
    NamedOrAnonymous,
};
use xmlity::{ExpandedName, LocalName, XmlNamespace};

use xsd::{ns, xs};

/// Extension trait for [`ExpandedName`] to handle default namespaces.
pub trait XmlNamespaceExt<'a> {
    /// Sets a default namespace if none is present.
    fn with_default_namespace<F: FnOnce() -> Option<XmlNamespace<'a>>>(self, f: F) -> Self;
}

impl<'a> XmlNamespaceExt<'a> for ExpandedName<'a> {
    fn with_default_namespace<F: FnOnce() -> Option<XmlNamespace<'a>>>(self, f: F) -> Self {
        let (local_name, mut namespace) = self.into_parts();

        namespace = namespace.or_else(f);

        ExpandedName::new(local_name.into_owned(), namespace)
    }
}

pub trait ComplexOffsetable {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    );

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>);
}

pub trait ComplexOffsetableExt: ComplexOffsetable + Sized {
    fn with_offset(
        mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) -> Self {
        self.offset(target, new, offsets);
        self
    }

    fn with_remapped_namespace(
        mut self,
        old: &Option<XmlNamespace>,
        new: &Option<XmlNamespace<'static>>,
    ) -> Self {
        self.remap_namespace(old, new);
        self
    }
}

impl<T: ComplexOffsetable> ComplexOffsetableExt for T {}

pub trait HasOffset {
    fn get_offset(offsets: &IdOffsets) -> usize;
}

impl<T: HasOffset> ComplexOffsetable for FragmentIdx<T> {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        if self.0 == *target {
            self.0 = *new;
            self.1 += T::get_offset(offsets);
        }
    }

    fn remap_namespace(
        &mut self,
        _old: &Option<XmlNamespace>,
        _new: &Option<XmlNamespace<'static>>,
    ) {
        // FragmentIdx doesn't contain namespace information, so no remapping needed
    }
}

impl HasOffset for ComplexTypeRootFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.complex_type_roots_offset
    }
}

impl HasOffset for simple::SimpleTypeRootFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        <Self as simple::HasOffset>::get_offset(&offsets.simple)
    }
}

impl HasOffset for simple::FacetFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        <Self as simple::HasOffset>::get_offset(&offsets.simple)
    }
}

impl HasOffset for SimpleRestrictionFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.simple_restrictions_offset
    }
}

impl HasOffset for SimpleExtensionFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.simple_extensions_offset
    }
}

impl HasOffset for SimpleContentFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.simple_contents_offset
    }
}

impl HasOffset for RestrictionFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.restrictions_offset
    }
}

impl HasOffset for ExtensionFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.extensions_offset
    }
}

impl HasOffset for ComplexContentFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.complex_contents_offset
    }
}

impl HasOffset for GroupRefFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.group_ref_offset
    }
}

impl HasOffset for AllFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.all_offset
    }
}

impl HasOffset for ChoiceFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.choice_offset
    }
}

impl HasOffset for SequenceFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.sequence_offset
    }
}

impl HasOffset for AnyFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.any_offset
    }
}

impl HasOffset for LocalElementFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.element_offset
    }
}

impl HasOffset for TopLevelElementFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.top_level_element_offset
    }
}

impl HasOffset for LocalAttributeFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.local_attribute_offset
    }
}

impl HasOffset for TopLevelAttributeFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.top_level_attribute_offset
    }
}

impl HasOffset for AttributeGroupRefFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.attribute_group_ref_offset
    }
}

impl HasOffset for TopLevelGroupFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.top_level_group_offset
    }
}

impl HasOffset for TopLevelAttributeGroupFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.top_level_attribute_group_offset
    }
}

impl HasOffset for AttributeDeclarationsFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.attribute_declaration_offset
    }
}

impl HasOffset for AnyAttributeFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.any_attribute_offset
    }
}

impl HasOffset for AssertionFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.assertion_offset
    }
}

impl HasOffset for AssertionsFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.assertion_group_offset
    }
}

impl HasOffset for IncludeFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.includes
    }
}

impl HasOffset for RedefineFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.redefines
    }
}

impl HasOffset for ImportFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.imports
    }
}

impl HasOffset for OverrideFragment {
    fn get_offset(offsets: &IdOffsets) -> usize {
        offsets.overrides
    }
}

impl ComplexOffsetable for NamedOrAnonymous<FragmentIdx<ComplexTypeRootFragment>> {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        match self {
            NamedOrAnonymous::Named(_idx) => {}
            NamedOrAnonymous::Anonymous(idx) => idx.offset(target, new, offsets),
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        match self {
            NamedOrAnonymous::Named(expanded_name) => {
                if expanded_name.namespace() == old {
                    *expanded_name =
                        ExpandedName::new(expanded_name.local_name().clone(), new.clone());
                }
            }
            NamedOrAnonymous::Anonymous(_idx) => {}
        }
    }
}

impl ComplexOffsetable for ExtensionFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        if let Some(ref mut content) = self.content_fragment {
            content.offset(target, new, offsets);
        }
        self.attribute_declarations.offset(target, new, offsets);
        self.assertions.offset(target, new, offsets);
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        if self.base.namespace() == old {
            self.base = ExpandedName::new(self.base.local_name().clone(), new.clone());
        }
        if let Some(ref mut content) = self.content_fragment {
            content.remap_namespace(old, new);
        }
        self.attribute_declarations.remap_namespace(old, new);
        self.assertions.remap_namespace(old, new);
    }
}

impl ComplexOffsetable for ComplexContentFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        self.content_fragment.offset(target, new, offsets);
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        self.content_fragment.remap_namespace(old, new);
    }
}

impl ComplexOffsetable for ComplexContentChildId {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        match self {
            ComplexContentChildId::Extension(fragment_id) => {
                fragment_id.offset(target, new, offsets)
            }
            ComplexContentChildId::Restriction(fragment_id) => {
                fragment_id.offset(target, new, offsets)
            }
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        match self {
            ComplexContentChildId::Extension(fragment_id) => fragment_id.remap_namespace(old, new),
            ComplexContentChildId::Restriction(fragment_id) => {
                fragment_id.remap_namespace(old, new)
            }
        }
    }
}

impl ComplexOffsetable for GroupRefFragment {
    #[allow(unused_variables)]
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        // GroupRefFragment has no fragmentIdx fields that need offsetting
        // Only contains ref_, min_occurs, max_occurs which are not fragment indices
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        if self.ref_.namespace() == old {
            let local_name = self.ref_.local_name().clone();
            self.ref_ = ExpandedName::new(local_name, new.clone());
        }
    }
}

impl ComplexOffsetable for AllFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        for fragment in &mut self.fragments {
            fragment.offset(target, new, offsets);
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        for fragment in &mut self.fragments {
            fragment.remap_namespace(old, new);
        }
    }
}

impl ComplexOffsetable for ChoiceFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        for fragment in &mut self.fragments {
            fragment.offset(target, new, offsets);
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        for fragment in &mut self.fragments {
            fragment.remap_namespace(old, new);
        }
    }
}

impl ComplexOffsetable for SequenceFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        for fragment in &mut self.fragments {
            fragment.offset(target, new, offsets);
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        for fragment in &mut self.fragments {
            fragment.remap_namespace(old, new);
        }
    }
}

impl ComplexOffsetable for AnyFragment {
    #[allow(unused_variables)]
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        // AnyFragment has no fragmentIdx fields that need offsetting
        // Only contains id and process_contents which are not fragment indices
    }

    fn remap_namespace(
        &mut self,
        _old: &Option<XmlNamespace>,
        _new: &Option<XmlNamespace<'static>>,
    ) {
        // AnyFragment has no namespace-related fields that need remapping
    }
}

impl ComplexOffsetable for LocalElementFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        self.type_.offset(target, new, offsets);
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        self.type_.remap_namespace(old, new);
    }
}

impl ComplexOffsetable for LocalElementFragmentType {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        match self {
            LocalElementFragmentType::Local(declared) => declared.offset(target, new, offsets),
            LocalElementFragmentType::Reference(_) => {
                // Reference elements don't have fragment indices to offset
            }
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        match self {
            LocalElementFragmentType::Local(declared) => declared.remap_namespace(old, new),
            LocalElementFragmentType::Reference(reference) => {
                if reference.ref_.namespace() == old {
                    reference.ref_ =
                        ExpandedName::new(reference.ref_.local_name().clone(), new.clone());
                }
            }
        }
    }
}

impl ComplexOffsetable for DeclaredElementFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        self.type_.offset(target, new, offsets);
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        self.type_.remap_namespace(old, new);
    }
}

impl ComplexOffsetable for TopLevelElementFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        if let Some(ref mut type_) = self.type_ {
            type_.offset(target, new, offsets);
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        if let Some(ref mut type_) = self.type_ {
            type_.remap_namespace(old, new);
        }
        for substitution_group in &mut self.substitution_groups {
            if substitution_group.namespace() == old {
                let local_name = substitution_group.local_name().clone();
                *substitution_group = ExpandedName::new(local_name, new.clone());
            }
        }
    }
}

impl ComplexOffsetable for LocalAttributeFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        self.type_mode.offset(target, new, offsets);
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        self.type_mode.remap_namespace(old, new);
    }
}

impl ComplexOffsetable for LocalAttributeFragmentTypeMode {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        match self {
            LocalAttributeFragmentTypeMode::Declared(declared) => {
                declared.offset(target, new, offsets)
            }
            LocalAttributeFragmentTypeMode::Reference(_) => {
                // Reference attributes don't have fragment indices to offset
            }
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        match self {
            LocalAttributeFragmentTypeMode::Declared(declared) => {
                declared.remap_namespace(old, new)
            }
            LocalAttributeFragmentTypeMode::Reference(reference) => {
                if reference.ref_.namespace() == old {
                    let local_name = reference.ref_.local_name().clone();
                    reference.ref_ = ExpandedName::new(local_name, new.clone());
                }
            }
        }
    }
}

impl ComplexOffsetable for DeclaredAttributeFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        if let Some(ref mut type_) = self.type_ {
            simple::SimpleOffsetable::offset(type_, target, new, &offsets.simple);
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        if let Some(ref mut type_) = self.type_ {
            simple::SimpleOffsetable::remap_namespace(type_, old, new);
        }
    }
}

impl ComplexOffsetable for TopLevelAttributeFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        if let Some(ref mut type_) = self.type_ {
            simple::SimpleOffsetable::offset(type_, target, new, &offsets.simple);
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        if let Some(ref mut type_) = self.type_ {
            simple::SimpleOffsetable::remap_namespace(type_, old, new);
        }
    }
}

impl ComplexOffsetable for AttributeGroupRefFragment {
    #[allow(unused_variables)]
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        // AttributeGroupRefFragment has no fragmentIdx fields that need offsetting
        // Only contains ref_ which is not a fragment index
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        if self.ref_.namespace() == old {
            let local_name = self.ref_.local_name().clone();
            self.ref_ = ExpandedName::new(local_name, new.clone());
        }
    }
}

impl ComplexOffsetable for TopLevelGroupFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        self.content.offset(target, new, offsets);
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        self.content.remap_namespace(old, new);
    }
}

impl ComplexOffsetable for TopLevelAttributeGroupFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        self.attr_decls.offset(target, new, offsets);
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        self.attr_decls.remap_namespace(old, new);
    }
}

impl ComplexOffsetable for AttributeDeclarationsFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        for declaration in &mut self.declarations {
            declaration.offset(target, new, offsets);
        }
        if let Some(ref mut any_attribute) = self.any_attribute {
            any_attribute.offset(target, new, offsets);
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        for declaration in &mut self.declarations {
            declaration.remap_namespace(old, new);
        }
        if let Some(ref mut any_attribute) = self.any_attribute {
            any_attribute.remap_namespace(old, new);
        }
    }
}

impl ComplexOffsetable for AnyAttributeFragment {
    #[allow(unused_variables)]
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        // AnyAttributeFragment has no fragmentIdx fields that need offsetting
        // Only contains id and process_contents which are not fragment indices
    }

    fn remap_namespace(
        &mut self,
        _old: &Option<XmlNamespace>,
        _new: &Option<XmlNamespace<'static>>,
    ) {
        // AnyAttributeFragment has no namespace-related fields that need remapping
    }
}

impl ComplexOffsetable for AssertionFragment {
    #[allow(unused_variables)]
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        // AssertionFragment has no fragmentIdx fields that need offsetting
        // Only contains id and test which are not fragment indices
    }

    fn remap_namespace(
        &mut self,
        _old: &Option<XmlNamespace>,
        _new: &Option<XmlNamespace<'static>>,
    ) {
        // AssertionFragment has no namespace-related fields that need remapping
    }
}

impl ComplexOffsetable for AssertionsFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        for assertion in &mut self.assertions {
            assertion.offset(target, new, offsets);
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        for assertion in &mut self.assertions {
            assertion.remap_namespace(old, new);
        }
    }
}

impl ComplexOffsetable for NestedParticleId {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        match self {
            NestedParticleId::Element(fragment_id) => fragment_id.offset(target, new, offsets),
            NestedParticleId::Group(fragment_id) => fragment_id.offset(target, new, offsets),
            NestedParticleId::Choice(fragment_id) => fragment_id.offset(target, new, offsets),
            NestedParticleId::Sequence(fragment_id) => fragment_id.offset(target, new, offsets),
            NestedParticleId::Any(fragment_id) => fragment_id.offset(target, new, offsets),
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        match self {
            NestedParticleId::Element(fragment_id) => fragment_id.remap_namespace(old, new),
            NestedParticleId::Group(fragment_id) => fragment_id.remap_namespace(old, new),
            NestedParticleId::Choice(fragment_id) => fragment_id.remap_namespace(old, new),
            NestedParticleId::Sequence(fragment_id) => fragment_id.remap_namespace(old, new),
            NestedParticleId::Any(fragment_id) => fragment_id.remap_namespace(old, new),
        }
    }
}

impl ComplexOffsetable for NamedOrAnonymous<ElementTypeContentId> {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        match self {
            NamedOrAnonymous::Named(_) => {
                // Named types don't need offsetting as they reference names
            }
            NamedOrAnonymous::Anonymous(content_id) => content_id.offset(target, new, offsets),
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        match self {
            NamedOrAnonymous::Named(expanded_name) => {
                if expanded_name.namespace() == old {
                    let local_name = expanded_name.local_name().clone();
                    *expanded_name = ExpandedName::new(local_name, new.clone());
                }
            }
            NamedOrAnonymous::Anonymous(content_id) => content_id.remap_namespace(old, new),
        }
    }
}

impl ComplexOffsetable for ElementTypeContentId {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        match self {
            ElementTypeContentId::SimpleType(fragment_id) => {
                simple::SimpleOffsetable::offset(fragment_id, target, new, &offsets.simple)
            }
            ElementTypeContentId::ComplexType(fragment_id) => {
                fragment_id.offset(target, new, offsets)
            }
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        match self {
            ElementTypeContentId::SimpleType(fragment_id) => {
                simple::SimpleOffsetable::remap_namespace(fragment_id, old, new)
            }
            ElementTypeContentId::ComplexType(fragment_id) => fragment_id.remap_namespace(old, new),
        }
    }
}

impl ComplexOffsetable for NamedGroupTypeContentId {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        match self {
            NamedGroupTypeContentId::All(fragment_id) => fragment_id.offset(target, new, offsets),
            NamedGroupTypeContentId::Sequence(fragment_id) => {
                fragment_id.offset(target, new, offsets)
            }
            NamedGroupTypeContentId::Choice(fragment_id) => {
                fragment_id.offset(target, new, offsets)
            }
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        match self {
            NamedGroupTypeContentId::All(fragment_id) => fragment_id.remap_namespace(old, new),
            NamedGroupTypeContentId::Sequence(fragment_id) => fragment_id.remap_namespace(old, new),
            NamedGroupTypeContentId::Choice(fragment_id) => fragment_id.remap_namespace(old, new),
        }
    }
}

impl ComplexOffsetable for TopLevelTypeId {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        match self {
            TopLevelTypeId::ComplexType(fragment_id) => fragment_id.offset(target, new, offsets),
            TopLevelTypeId::SimpleType(fragment_id) => {
                simple::SimpleOffsetable::offset(fragment_id, target, new, &offsets.simple)
            }
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        match self {
            TopLevelTypeId::ComplexType(fragment_id) => fragment_id.remap_namespace(old, new),
            TopLevelTypeId::SimpleType(fragment_id) => {
                simple::SimpleOffsetable::remap_namespace(fragment_id, old, new)
            }
        }
    }
}

impl ComplexOffsetable for RedefinableId {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        match self {
            RedefinableId::ComplexType(fragment_id) => fragment_id.offset(target, new, offsets),
            RedefinableId::SimpleType(fragment_id) => {
                ComplexOffsetable::offset(fragment_id, target, new, offsets)
            }
            RedefinableId::AttributeGroup(fragment_id) => fragment_id.offset(target, new, offsets),
            RedefinableId::Group(fragment_id) => fragment_id.offset(target, new, offsets),
            RedefinableId::Notation => {
                //TODO
            }
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        match self {
            RedefinableId::ComplexType(fragment_id) => fragment_id.remap_namespace(old, new),
            RedefinableId::SimpleType(fragment_id) => {
                simple::SimpleOffsetable::remap_namespace(fragment_id, old, new)
            }
            RedefinableId::AttributeGroup(fragment_id) => fragment_id.remap_namespace(old, new),
            RedefinableId::Group(fragment_id) => fragment_id.remap_namespace(old, new),
            RedefinableId::Notation => {
                //TODO
            }
        }
    }
}

impl ComplexOffsetable for SchemaTopId {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        match self {
            SchemaTopId::Redefinable(redefinable_id) => {
                redefinable_id.offset(target, new, offsets);
            }
            SchemaTopId::Element(fragment_idx) => {
                fragment_idx.offset(target, new, offsets);
            }
            SchemaTopId::Attribute(fragment_idx) => {
                fragment_idx.offset(target, new, offsets);
            }
            SchemaTopId::Notation => {
                //TODO
            }
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        match self {
            SchemaTopId::Redefinable(redefinable_id) => {
                redefinable_id.remap_namespace(old, new);
            }
            SchemaTopId::Element(fragment_idx) => {
                fragment_idx.remap_namespace(old, new);
            }
            SchemaTopId::Attribute(fragment_idx) => {
                fragment_idx.remap_namespace(old, new);
            }
            SchemaTopId::Notation => {
                //TODO
            }
        }
    }
}

impl ComplexOffsetable for CompositionId {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        match self {
            CompositionId::Include(fragment_idx) => fragment_idx.offset(target, new, offsets),
            CompositionId::Import(fragment_idx) => fragment_idx.offset(target, new, offsets),
            CompositionId::Redefine(fragment_idx) => fragment_idx.offset(target, new, offsets),
            CompositionId::Override(fragment_idx) => fragment_idx.offset(target, new, offsets),
            CompositionId::AnnotationId => {
                //TODO
            }
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        match self {
            CompositionId::Include(fragment_idx) => fragment_idx.remap_namespace(old, new),
            CompositionId::Import(fragment_idx) => fragment_idx.remap_namespace(old, new),
            CompositionId::Redefine(fragment_idx) => fragment_idx.remap_namespace(old, new),
            CompositionId::Override(fragment_idx) => fragment_idx.remap_namespace(old, new),
            CompositionId::AnnotationId => {
                //TODO
            }
        }
    }
}

/// Identifier for type definition particles in complex types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeDefParticleId {
    /// Group reference particle.
    Group(FragmentIdx<GroupRefFragment>),
    /// All particle.
    All(FragmentIdx<AllFragment>),
    /// Sequence particle.
    Sequence(FragmentIdx<SequenceFragment>),
    /// Choice particle.
    Choice(FragmentIdx<ChoiceFragment>),
}

impl ComplexOffsetable for TypeDefParticleId {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        match self {
            TypeDefParticleId::Group(fragment_id) => fragment_id.offset(target, new, offsets),
            TypeDefParticleId::All(fragment_id) => fragment_id.offset(target, new, offsets),
            TypeDefParticleId::Sequence(fragment_id) => fragment_id.offset(target, new, offsets),
            TypeDefParticleId::Choice(fragment_id) => fragment_id.offset(target, new, offsets),
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        match self {
            TypeDefParticleId::Group(fragment_id) => fragment_id.remap_namespace(old, new),
            TypeDefParticleId::All(fragment_id) => fragment_id.remap_namespace(old, new),
            TypeDefParticleId::Sequence(fragment_id) => fragment_id.remap_namespace(old, new),
            TypeDefParticleId::Choice(fragment_id) => fragment_id.remap_namespace(old, new),
        }
    }
}

impl From<FragmentIdx<GroupRefFragment>> for TypeDefParticleId {
    fn from(value: FragmentIdx<GroupRefFragment>) -> Self {
        Self::Group(value)
    }
}
impl From<FragmentIdx<AllFragment>> for TypeDefParticleId {
    fn from(value: FragmentIdx<AllFragment>) -> Self {
        Self::All(value)
    }
}
impl From<FragmentIdx<SequenceFragment>> for TypeDefParticleId {
    fn from(value: FragmentIdx<SequenceFragment>) -> Self {
        Self::Sequence(value)
    }
}
impl From<FragmentIdx<ChoiceFragment>> for TypeDefParticleId {
    fn from(value: FragmentIdx<ChoiceFragment>) -> Self {
        Self::Choice(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TopLevelTypeId {
    /// A complex type.
    ComplexType(FragmentIdx<ComplexTypeRootFragment>),
    /// A simple type.
    SimpleType(FragmentIdx<simple::SimpleTypeRootFragment>),
}

#[derive(Debug, Clone)]
pub struct SchemaFragment {
    pub compiler: ComplexTypeFragmentCompiler,
    pub target_namespace: Option<XmlNamespace<'static>>,
    pub top_level_types: BTreeMap<LocalName<'static>, TopLevelTypeId>,
    pub top_level_elements: BTreeMap<LocalName<'static>, FragmentIdx<TopLevelElementFragment>>,
    pub top_level_attributes: BTreeMap<LocalName<'static>, FragmentIdx<TopLevelAttributeFragment>>,
    pub top_level_groups: BTreeMap<LocalName<'static>, FragmentIdx<TopLevelGroupFragment>>,
    pub top_level_attribute_groups:
        BTreeMap<LocalName<'static>, FragmentIdx<TopLevelAttributeGroupFragment>>,
    pub compositions: VecDeque<CompositionId>,
    pub schema_tops: VecDeque<SchemaTopId>,
}

impl SchemaFragment {
    pub fn register_refinable(
        &mut self,
        redefinable_id: RedefinableId,
        prefix: bool,
    ) -> Result<(), Error> {
        match redefinable_id {
            RedefinableId::ComplexType(fragment_idx) => {
                let fragment = self
                    .compiler
                    .get_fragment(&fragment_idx)
                    .expect("Just added.");

                self.top_level_types.insert(
                    fragment.name.clone().expect("Missing name"),
                    TopLevelTypeId::ComplexType(fragment_idx),
                );
            }
            RedefinableId::SimpleType(fragment_idx) => {
                let fragment = self
                    .compiler
                    .simple_type_compiler
                    .get_fragment(&fragment_idx)
                    .expect("Just added.");

                self.top_level_types.insert(
                    fragment.name.clone().expect("Missing name"),
                    TopLevelTypeId::SimpleType(fragment_idx),
                );
            }
            RedefinableId::AttributeGroup(fragment_idx) => {
                let fragment = self
                    .compiler
                    .get_fragment(&fragment_idx)
                    .expect("Just added.");

                self.top_level_attribute_groups
                    .insert(fragment.name.clone(), fragment_idx);
            }
            RedefinableId::Group(fragment_idx) => {
                let fragment = self
                    .compiler
                    .get_fragment(&fragment_idx)
                    .expect("Just added.");

                self.top_level_groups
                    .insert(fragment.name.clone(), fragment_idx);
            }
            RedefinableId::Notation => {
                //TODO
            }
        }

        if prefix {
            self.schema_tops
                .push_front(SchemaTopId::Redefinable(redefinable_id));
        } else {
            self.schema_tops
                .push_back(SchemaTopId::Redefinable(redefinable_id));
        }

        Ok(())
    }

    /// Converts this type to complex fragments.
    pub fn from_schema(
        schema: &xs::Schema,
        namespace_idx: FragmentedXsdDocumentIdx,
    ) -> Result<Self, Error> {
        let xs::Schema::Schema(schema) = schema else {
            panic!("Expected a schema, but found: {:?}", schema);
        };

        let compiler = ComplexTypeFragmentCompiler::new(namespace_idx);
        let target_namespace = schema
            .target_namespace
            .as_ref()
            .map(|ns| XmlNamespace::new(ns.to_owned()))
            .transpose()
            .unwrap();

        let top_level_types = BTreeMap::new();
        let top_level_elements = BTreeMap::new();
        let top_level_attributes = BTreeMap::new();
        let top_level_groups = BTreeMap::new();
        let top_level_attribute_groups = BTreeMap::new();

        let mut schema_fragment = SchemaFragment {
            compiler,
            target_namespace,
            top_level_types,
            top_level_elements,
            top_level_attributes,
            top_level_groups,
            top_level_attribute_groups,
            schema_tops: VecDeque::new(),
            compositions: VecDeque::new(),
        };

        let default_namespace = schema_fragment.target_namespace.clone();

        schema
            .child_2
            .iter()
            .map(|a: &xs::schema_items::Child2| &a.schema_top)
            .try_for_each(|schema_top| {
                let schema_top = schema_top.to_complex_fragments(
                    &mut schema_fragment.compiler,
                    &Context {
                        default_namespace: default_namespace.as_ref(),
                    },
                )?;

                match schema_top {
                    SchemaTopId::Redefinable(redefinable_id) => {
                        schema_fragment.register_refinable(redefinable_id, false)
                    }
                    SchemaTopId::Element(fragment_idx) => {
                        let fragment = schema_fragment
                            .compiler
                            .get_fragment(&fragment_idx)
                            .expect("Just added.");

                        schema_fragment
                            .top_level_elements
                            .insert(fragment.name.clone(), fragment_idx);
                        schema_fragment
                            .schema_tops
                            .push_back(SchemaTopId::Element(fragment_idx));

                        Ok(())
                    }
                    SchemaTopId::Attribute(fragment_idx) => {
                        let fragment = schema_fragment
                            .compiler
                            .get_fragment(&fragment_idx)
                            .expect("Just added.");

                        schema_fragment
                            .top_level_attributes
                            .insert(fragment.name.clone(), fragment_idx);
                        schema_fragment
                            .schema_tops
                            .push_back(SchemaTopId::Attribute(fragment_idx));

                        Ok(())
                    }
                    SchemaTopId::Notation => {
                        //TODO
                        Ok(())
                    }
                }
            })?;

        schema_fragment.compositions.extend(
            schema
                .composition
                .iter()
                .map(|composition| {
                    composition.to_complex_fragments(
                        &mut schema_fragment.compiler,
                        &Context {
                            default_namespace: default_namespace.as_ref(),
                        },
                    )
                })
                .collect::<Result<Vec<_>, Error>>()?,
        );

        Ok(schema_fragment)
    }

    /// Reconstructs this type from complex fragments.
    pub fn to_schema(&self) -> Result<xs::Schema, Error> {
        let schema_tops = self
            .schema_tops
            .iter()
            .map(|schema_top_id| {
                xs::groups::SchemaTop::from_complex_fragments(&self.compiler, schema_top_id)
            })
            .map(|schema_top| {
                schema_top.map(|schema_top| xs::schema_items::Child2 {
                    schema_top,
                    annotation: Vec::new(),
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;

        let compositions = self
            .compositions
            .iter()
            .map(|composition_id| {
                xs::groups::Composition::from_complex_fragments(&self.compiler, composition_id)
            })
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(xs::Schema::Schema(
            xs::schema_items::Schema::builder()
                .maybe_target_namespace(self.target_namespace.as_ref().map(|ns| ns.to_string()))
                .child_2(schema_tops)
                .composition(compositions)
                .build(),
        ))
    }

    /// This merges another schema fragment into this one.
    pub fn merge_with(&mut self, other: &Self) -> Result<(), Error> {
        if other
            .target_namespace
            .as_ref()
            .is_some_and(|ns| Some(ns) != self.target_namespace.as_ref())
        {
            todo!(
                "Handle error for merging namespaces with different namespaces: {:?} and {:?}",
                self.target_namespace,
                other.target_namespace
            );
        }

        let offsets = self.compiler.merge_with(
            &other.compiler,
            &other.target_namespace,
            &self.target_namespace,
        )?;

        for (name, top_level_type) in &other.top_level_types {
            self.top_level_types.entry(name.clone()).or_insert_with(|| {
                top_level_type
                    .clone()
                    .with_offset(
                        &other.compiler.namespace_idx,
                        &self.compiler.namespace_idx,
                        &offsets,
                    )
                    .with_remapped_namespace(&other.target_namespace, &self.target_namespace)
            });
        }

        for (name, top_level_element) in &other.top_level_elements {
            self.top_level_elements
                .entry(name.clone())
                .or_insert_with(|| {
                    top_level_element
                        .clone()
                        .with_offset(
                            &other.compiler.namespace_idx,
                            &self.compiler.namespace_idx,
                            &offsets,
                        )
                        .with_remapped_namespace(&other.target_namespace, &self.target_namespace)
                });
        }

        for (name, top_level_attribute) in &other.top_level_attributes {
            self.top_level_attributes
                .entry(name.clone())
                .or_insert_with(|| {
                    top_level_attribute
                        .clone()
                        .with_offset(
                            &other.compiler.namespace_idx,
                            &self.compiler.namespace_idx,
                            &offsets,
                        )
                        .with_remapped_namespace(&other.target_namespace, &self.target_namespace)
                });
        }

        for (name, top_level_group) in &other.top_level_groups {
            self.top_level_groups
                .entry(name.clone())
                .or_insert_with(|| {
                    top_level_group
                        .clone()
                        .with_offset(
                            &other.compiler.namespace_idx,
                            &self.compiler.namespace_idx,
                            &offsets,
                        )
                        .with_remapped_namespace(&other.target_namespace, &self.target_namespace)
                });
        }

        for (name, top_level_attribute_group) in &other.top_level_attribute_groups {
            self.top_level_attribute_groups
                .entry(name.clone())
                .or_insert_with(|| {
                    top_level_attribute_group
                        .clone()
                        .with_offset(
                            &other.compiler.namespace_idx,
                            &self.compiler.namespace_idx,
                            &offsets,
                        )
                        .with_remapped_namespace(&other.target_namespace, &self.target_namespace)
                });
        }

        other.schema_tops.iter().rev().for_each(|schema_top_id| {
            self.schema_tops.push_front(
                schema_top_id
                    .clone()
                    .with_offset(
                        &other.compiler.namespace_idx,
                        &self.compiler.namespace_idx,
                        &offsets,
                    )
                    .with_remapped_namespace(&other.target_namespace, &self.target_namespace),
            );
        });

        other.compositions.iter().for_each(|composition_id| {
            self.compositions.push_front(
                composition_id
                    .clone()
                    .with_offset(
                        &other.compiler.namespace_idx,
                        &self.compiler.namespace_idx,
                        &offsets,
                    )
                    .with_remapped_namespace(&other.target_namespace, &self.target_namespace),
            );
        });

        Ok(())
    }
}

/// Fragment representing a complex type extension.
#[derive(Debug, Clone, PartialEq)]
pub struct ExtensionFragment {
    /// The base type being extended.
    pub base: ExpandedName<'static>,
    /// Optional content particle.
    pub content_fragment: Option<TypeDefParticleId>,
    /// Attribute declarations for this extension.
    pub attribute_declarations: FragmentIdx<AttributeDeclarationsFragment>,
    /// Assertions for this extension.
    pub assertions: FragmentIdx<AssertionsFragment>,
}

/// Fragment representing a complex type restriction.
#[derive(Debug, Clone, PartialEq)]
pub struct RestrictionFragment {
    /// The base type being restricted.
    pub base: ExpandedName<'static>,
    /// Optional content particle.
    pub content_fragment: Option<TypeDefParticleId>,
    /// Attribute declarations for this restriction.
    pub attribute_declarations: FragmentIdx<AttributeDeclarationsFragment>,
    /// Assertions for this restriction.
    pub assertions: FragmentIdx<AssertionsFragment>,
}

impl ComplexOffsetable for RestrictionFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        self.content_fragment
            .as_mut()
            .map(|fragment| fragment.offset(target, new, offsets));
        self.attribute_declarations.offset(target, new, offsets);
        self.assertions.offset(target, new, offsets);
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        if self.base.namespace() == old {
            let local_name = self.base.local_name().clone();
            self.base = ExpandedName::new(local_name, new.clone());
        }
        if let Some(ref mut content_fragment) = self.content_fragment {
            content_fragment.remap_namespace(old, new);
        }
        self.attribute_declarations.remap_namespace(old, new);
        self.assertions.remap_namespace(old, new);
    }
}

/// Identifier for attribute declarations, either direct attributes or attribute group references.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttributeDeclarationId {
    /// Direct attribute declaration.
    Attribute(FragmentIdx<LocalAttributeFragment>),
    /// Attribute group reference.
    AttributeGroupRef(FragmentIdx<AttributeGroupRefFragment>),
}

impl ComplexOffsetable for AttributeDeclarationId {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        match self {
            AttributeDeclarationId::Attribute(fragment_id) => {
                fragment_id.offset(target, new, offsets)
            }
            AttributeDeclarationId::AttributeGroupRef(fragment_id) => {
                fragment_id.offset(target, new, offsets)
            }
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        match self {
            AttributeDeclarationId::Attribute(fragment_id) => fragment_id.remap_namespace(old, new),
            AttributeDeclarationId::AttributeGroupRef(fragment_id) => {
                fragment_id.remap_namespace(old, new)
            }
        }
    }
}

impl From<FragmentIdx<LocalAttributeFragment>> for AttributeDeclarationId {
    fn from(value: FragmentIdx<LocalAttributeFragment>) -> Self {
        Self::Attribute(value)
    }
}
impl From<FragmentIdx<AttributeGroupRefFragment>> for AttributeDeclarationId {
    fn from(value: FragmentIdx<AttributeGroupRefFragment>) -> Self {
        Self::AttributeGroupRef(value)
    }
}

/// How an attribute is used in a complex type.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AttributeUse {
    /// Attribute is required.
    Required,
    /// Attribute is optional.
    #[default]
    Optional,
    /// Attribute is prohibited.
    Prohibited,
}

impl From<xs::types::attribute_items::UseValue> for AttributeUse {
    fn from(value: xs::types::attribute_items::UseValue) -> Self {
        match value {
            xs::types::attribute_items::UseValue::Prohibited => Self::Prohibited,
            xs::types::attribute_items::UseValue::Optional => Self::Optional,
            xs::types::attribute_items::UseValue::Required => Self::Required,
        }
    }
}

impl From<AttributeUse> for xs::types::attribute_items::UseValue {
    fn from(value: AttributeUse) -> Self {
        match value {
            AttributeUse::Prohibited => xs::types::attribute_items::UseValue::Prohibited,
            AttributeUse::Optional => xs::types::attribute_items::UseValue::Optional,
            AttributeUse::Required => xs::types::attribute_items::UseValue::Required,
        }
    }
}

/// Fragment for a declared attribute with a local name.
#[derive(Debug, Clone, PartialEq)]
pub struct DeclaredAttributeFragment {
    /// Local name of the attribute.
    pub name: LocalName<'static>,
    /// Type of the attribute.
    pub type_: Option<NamedOrAnonymous<FragmentIdx<simple::SimpleTypeRootFragment>>>,
}

/// Fragment for a reference to a top-level attribute.
#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceAttributeFragment {
    /// Reference to the top-level attribute.
    pub ref_: ExpandedName<'static>,
}

/// Type mode for local attribute fragments.
#[derive(Debug, Clone, PartialEq)]
pub enum LocalAttributeFragmentTypeMode {
    /// Declared attribute with local name and type.
    Declared(DeclaredAttributeFragment),
    /// Reference to a top-level attribute.
    Reference(ReferenceAttributeFragment),
}

/// Fragment representing a local attribute declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalAttributeFragment {
    /// How the attribute is declared (local or reference).
    pub type_mode: LocalAttributeFragmentTypeMode,
    /// How the attribute is used.
    pub use_: Option<AttributeUse>,
    /// Default value for the attribute.
    pub default: Option<String>,
}

/// Fragment representing a top-level attribute declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct TopLevelAttributeFragment {
    /// Name of the attribute.
    pub name: LocalName<'static>,
    /// Type of the attribute.
    pub type_: Option<NamedOrAnonymous<FragmentIdx<simple::SimpleTypeRootFragment>>>,
    pub default_: Option<String>,
}

/// Fragment representing a top-level group declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct TopLevelGroupFragment {
    /// Name of the group.
    pub name: LocalName<'static>,
    /// Content of the group.
    pub content: NamedGroupTypeContentId,
}

/// Fragment representing a reference to an attribute group.
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeGroupRefFragment {
    /// Reference to the attribute group.
    pub ref_: ExpandedName<'static>,
}

/// Fragment representing a top-level attribute group declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct TopLevelAttributeGroupFragment {
    /// Name of the attribute group.
    pub name: LocalName<'static>,
    /// Attribute declarations in this group.
    pub attr_decls: FragmentIdx<AttributeDeclarationsFragment>,
}

/// Fragment containing attribute declarations and any-attribute.
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeDeclarationsFragment {
    /// List of attribute declarations.
    pub declarations: VecDeque<AttributeDeclarationId>,
    /// Optional any-attribute declaration.
    pub any_attribute: Option<FragmentIdx<AnyAttributeFragment>>,
}

/// Fragment representing simple content in a complex type.
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleContentFragment {
    /// The content fragment (extension or restriction).
    pub content_fragment: SimpleContentChildId,
}

impl ComplexOffsetable for SimpleContentFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        self.content_fragment.offset(target, new, offsets);
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        self.content_fragment.remap_namespace(old, new);
    }
}

/// Fragment representing a simple content extension.
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleExtensionFragment {
    /// The base type being extended.
    pub base: ExpandedName<'static>,
    /// Attribute declarations for this extension.
    pub attribute_declarations: FragmentIdx<AttributeDeclarationsFragment>,
    /// Assertions for this extension.
    pub assertions: FragmentIdx<AssertionsFragment>,
}

impl ComplexOffsetable for SimpleExtensionFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        self.attribute_declarations.offset(target, new, offsets);
        self.assertions.offset(target, new, offsets);
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        if self.base.namespace() == old {
            let local_name = self.base.local_name().clone();
            self.base = ExpandedName::new(local_name, new.clone());
        }
        self.attribute_declarations.remap_namespace(old, new);
        self.assertions.remap_namespace(old, new);
    }
}

/// Fragment representing a simple content restriction.
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleRestrictionFragment {
    /// The base type being restricted.
    pub base: ExpandedName<'static>,
    /// Facets applied in this restriction.
    pub facets: Vec<FragmentIdx<simple::FacetFragment>>,
    /// Inline simple type definition.
    pub simple_type: Option<FragmentIdx<simple::SimpleTypeRootFragment>>,
    /// ID attribute for the restriction.
    pub id: Option<String>,
    /// Attribute declarations for this restriction.
    pub attribute_declarations: FragmentIdx<AttributeDeclarationsFragment>,
    /// Assertions for this restriction.
    pub assertions: FragmentIdx<AssertionsFragment>,
}

impl ComplexOffsetable for SimpleRestrictionFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        for facet in &mut self.facets {
            simple::SimpleOffsetable::offset(facet, target, new, &offsets.simple);
        }
        if let Some(simple_type) = &mut self.simple_type {
            simple::SimpleOffsetable::offset(simple_type, target, new, &offsets.simple);
        }
        self.attribute_declarations.offset(target, new, offsets);
        self.assertions.offset(target, new, offsets);
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        if self.base.namespace() == old {
            let local_name = self.base.local_name().clone();
            self.base = ExpandedName::new(local_name, new.clone());
        }
        for facet in &mut self.facets {
            simple::SimpleOffsetable::remap_namespace(facet, old, new);
        }
        if let Some(ref mut simple_type) = self.simple_type {
            simple::SimpleOffsetable::remap_namespace(simple_type, old, new);
        }
        self.attribute_declarations.remap_namespace(old, new);
        self.assertions.remap_namespace(old, new);
    }
}

/// Identifier for simple content child fragments.
#[derive(Debug, Clone, PartialEq)]
pub enum SimpleContentChildId {
    /// Simple content extension.
    Extension(FragmentIdx<SimpleExtensionFragment>),
    /// Simple content restriction.
    Restriction(FragmentIdx<SimpleRestrictionFragment>),
}

impl ComplexOffsetable for SimpleContentChildId {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        match self {
            SimpleContentChildId::Extension(fragment_id) => {
                fragment_id.offset(target, new, offsets)
            }
            SimpleContentChildId::Restriction(fragment_id) => {
                fragment_id.offset(target, new, offsets)
            }
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        match self {
            SimpleContentChildId::Extension(fragment_id) => fragment_id.remap_namespace(old, new),
            SimpleContentChildId::Restriction(fragment_id) => fragment_id.remap_namespace(old, new),
        }
    }
}

/// Fragment representing complex content in a complex type.
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexContentFragment {
    /// Whether the content is mixed.
    pub mixed: Option<bool>,
    /// The content fragment (extension or restriction).
    pub content_fragment: ComplexContentChildId,
}

/// Identifier for complex content child fragments.
#[derive(Debug, Clone, PartialEq)]
pub enum ComplexContentChildId {
    /// Complex content extension.
    Extension(FragmentIdx<ExtensionFragment>),
    /// Complex content restriction.
    Restriction(FragmentIdx<RestrictionFragment>),
}

/// Fragment for a declared element with local name and type.
#[derive(Debug, Clone, PartialEq)]
pub struct DeclaredElementFragment {
    /// Local name of the element.
    pub name: LocalName<'static>,
    /// Type of the element.
    pub type_: NamedOrAnonymous<ElementTypeContentId>,
}

/// Fragment for a reference to a top-level element.
#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceElementFragment {
    /// Reference to the top-level element.
    pub ref_: ExpandedName<'static>,
}

/// Type of local element fragment.
#[derive(Debug, Clone, PartialEq)]
pub enum LocalElementFragmentType {
    /// Local element declaration.
    Local(DeclaredElementFragment),
    /// Reference to top-level element.
    Reference(ReferenceElementFragment),
}

/// Fragment representing a local element declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalElementFragment {
    /// Minimum number of occurrences.
    pub min_occurs: Option<usize>,
    /// Maximum number of occurrences.
    pub max_occurs: Option<AllNNI>,
    /// Type of the element (local or reference).
    pub type_: LocalElementFragmentType,
}

/// Fragment representing a top-level element declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct TopLevelElementFragment {
    /// Name of the element.
    pub name: LocalName<'static>,
    /// Type of the element (named or anonymous).
    pub type_: Option<NamedOrAnonymous<ElementTypeContentId>>,
    /// List of substitution groups this element belongs to.
    pub substitution_groups: Vec<ExpandedName<'static>>,
    /// Whether the element is abstract.
    pub abstract_: bool,
}

/// Fragment representing a reference to a group.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupRefFragment {
    /// Minimum number of occurrences.
    pub min_occurs: Option<usize>,
    /// Maximum number of occurrences.
    pub max_occurs: Option<AllNNI>,
    /// Reference to the group.
    pub ref_: ExpandedName<'static>,
}

/// Fragment representing an "all" compositor.
#[derive(Debug, Clone, PartialEq)]
pub struct AllFragment {
    /// Minimum number of occurrences.
    pub min_occurs: Option<usize>,
    /// Maximum number of occurrences.
    pub max_occurs: Option<AllNNI>,
    /// Child particles.
    pub fragments: VecDeque<NestedParticleId>,
}

/// Fragment representing a "choice" compositor.
#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceFragment {
    /// Minimum number of occurrences.
    pub min_occurs: Option<usize>,
    /// Maximum number of occurrences.
    pub max_occurs: Option<AllNNI>,
    /// Child particles.
    pub fragments: VecDeque<NestedParticleId>,
}

/// Represents the maximum occurrence of types or elements
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum AllNNI {
    /// The occurrence is unbounded.
    Unbounded,

    /// The occurrence is bound to the specified limit.
    Bounded(usize),
}

impl Default for AllNNI {
    fn default() -> Self {
        Self::Bounded(1)
    }
}

impl From<xs::types::AllNNI> for AllNNI {
    fn from(value: xs::types::AllNNI) -> Self {
        AllNNI::from(&value)
    }
}

impl<'a> From<&'a xs::types::AllNNI> for AllNNI {
    fn from(value: &'a xs::types::AllNNI) -> Self {
        match value {
            xs::types::all_nni_items::AllNNI::NonNegativeInteger(a) => Self::Bounded(**a),
            xs::types::all_nni_items::AllNNI::Variant0(variant0) => match **variant0 {
                xs::types::all_nni_items::all_nni_variants::Variant0::Unbounded => Self::Unbounded,
            },
        }
    }
}

impl From<AllNNI> for xs::types::AllNNI {
    fn from(value: AllNNI) -> Self {
        match value {
            AllNNI::Unbounded => xs::types::AllNNI::Variant0(Box::new(
                xs::types::all_nni_items::all_nni_variants::Variant0::Unbounded,
            )),
            AllNNI::Bounded(a) => xs::types::AllNNI::NonNegativeInteger(Box::new(a)),
        }
    }
}

impl From<AllNNI> for xs::types::all_items::MaxOccursValue {
    fn from(value: AllNNI) -> Self {
        match value {
            AllNNI::Bounded(n) => xs::types::all_items::MaxOccursValue::from(n),
            AllNNI::Unbounded => xs::types::all_items::MaxOccursValue::from(
                xs::types::all_items::max_occurs_value_variants::Variant0::Unbounded,
            ),
        }
    }
}

/// Fragment representing a "sequence" compositor.
#[derive(Debug, Clone, PartialEq)]
pub struct SequenceFragment {
    /// Optional identifier.
    pub id: Option<String>,
    /// Minimum number of occurrences.
    pub min_occurs: Option<usize>,
    /// Maximum number of occurrences.
    pub max_occurs: Option<AllNNI>,
    /// Child particles.
    pub fragments: VecDeque<NestedParticleId>,
}

/// Identifier for complex type content models.
#[derive(Debug, Clone, PartialEq)]
pub enum ComplexTypeModelId {
    /// Simple content model.
    SimpleContent(FragmentIdx<SimpleContentFragment>),
    /// Complex content model.
    ComplexContent(FragmentIdx<ComplexContentFragment>),
    /// Other content model with particles and attributes.
    Other {
        //TODO: Add open content
        // open_content: Option<OpenContentId>,
        /// Optional particle content.
        particle: Option<TypeDefParticleId>,
        /// Attribute declarations.
        attr_decls: FragmentIdx<AttributeDeclarationsFragment>,
        /// Assertions for the content model.
        assertions: FragmentIdx<AssertionsFragment>,
    },
}

impl ComplexOffsetable for ComplexTypeModelId {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        match self {
            ComplexTypeModelId::SimpleContent(fragment) => fragment.offset(target, new, offsets),
            ComplexTypeModelId::ComplexContent(fragment) => fragment.offset(target, new, offsets),
            ComplexTypeModelId::Other {
                particle,
                attr_decls,
                assertions,
            } => {
                if let Some(particle) = particle {
                    particle.offset(target, new, offsets);
                }
                attr_decls.offset(target, new, offsets);
                assertions.offset(target, new, offsets);
            }
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        match self {
            ComplexTypeModelId::SimpleContent(fragment) => fragment.remap_namespace(old, new),
            ComplexTypeModelId::ComplexContent(fragment) => fragment.remap_namespace(old, new),
            ComplexTypeModelId::Other {
                particle,
                attr_decls,
                assertions,
            } => {
                if let Some(particle) = particle {
                    particle.remap_namespace(old, new);
                }
                attr_decls.remap_namespace(old, new);
                assertions.remap_namespace(old, new);
            }
        }
    }
}

/// Fragment representing a complex type definition.
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexTypeRootFragment {
    /// Optional name for named types.
    pub name: Option<LocalName<'static>>,
    /// Content model of the complex type.
    pub content: ComplexTypeModelId,
    /// Whether content is mixed.
    pub mixed: Option<bool>,
    /// Whether the type is abstract.
    pub abstract_: Option<bool>,
}

impl ComplexOffsetable for ComplexTypeRootFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        self.content.offset(target, new, offsets);
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        self.content.remap_namespace(old, new);
    }
}

/// Process contents for any wildcard.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnyProcessContents {
    /// Skip processing of contents.
    Skip,
    /// Lax processing of contents.
    Lax,
    /// Strict processing of contents.
    Strict,
}

impl From<xs::any_items::ProcessContentsValue> for AnyProcessContents {
    fn from(value: xs::any_items::ProcessContentsValue) -> Self {
        match value {
            xs::any_items::ProcessContentsValue::Skip => Self::Skip,
            xs::any_items::ProcessContentsValue::Lax => Self::Lax,
            xs::any_items::ProcessContentsValue::Strict => Self::Strict,
        }
    }
}

impl From<AnyProcessContents> for xs::any_items::ProcessContentsValue {
    fn from(value: AnyProcessContents) -> Self {
        match value {
            AnyProcessContents::Skip => Self::Skip,
            AnyProcessContents::Lax => Self::Lax,
            AnyProcessContents::Strict => Self::Strict,
        }
    }
}

/// Fragment representing any element wildcard.
#[derive(Debug, Clone, PartialEq)]
pub struct AnyFragment {
    /// Optional identifier for the fragment.
    pub id: Option<String>,
    /// Process contents for the any element.
    pub process_contents: Option<AnyProcessContents>,
}

/// Process contents for any attribute wildcard.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnyAttributeProcessContents {
    /// Skip processing of attributes.
    Skip,
    /// Lax processing of attributes.
    Lax,
    /// Strict processing of attributes.
    Strict,
}

impl From<xs::any_attribute_items::ProcessContentsValue> for AnyAttributeProcessContents {
    fn from(value: xs::any_attribute_items::ProcessContentsValue) -> Self {
        match value {
            xs::any_attribute_items::ProcessContentsValue::Skip => Self::Skip,
            xs::any_attribute_items::ProcessContentsValue::Lax => Self::Lax,
            xs::any_attribute_items::ProcessContentsValue::Strict => Self::Strict,
        }
    }
}

impl From<AnyAttributeProcessContents> for xs::any_attribute_items::ProcessContentsValue {
    fn from(value: AnyAttributeProcessContents) -> Self {
        match value {
            AnyAttributeProcessContents::Skip => Self::Skip,
            AnyAttributeProcessContents::Lax => Self::Lax,
            AnyAttributeProcessContents::Strict => Self::Strict,
        }
    }
}

/// Fragment representing any attribute wildcard.
#[derive(Debug, Clone, PartialEq)]
pub struct AnyAttributeFragment {
    /// Optional identifier for the fragment.
    pub id: Option<String>,
    /// Process contents for the any attribute.
    pub process_contents: Option<AnyAttributeProcessContents>,
}

/// Fragment representing an assertion in a complex type.
#[derive(Debug, Clone, PartialEq)]
pub struct AssertionFragment {
    /// Optional identifier for the assertion.
    pub id: Option<String>,
    /// The assertion expression.
    pub test: Option<String>,
}

/// Fragment representing any attribute wildcard.
#[derive(Debug, Clone, PartialEq)]
pub struct AssertionsFragment {
    /// List of assertions in this fragment.
    pub assertions: VecDeque<FragmentIdx<AssertionFragment>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RedefineFragment {
    pub schema_location: String,
    pub redefineable: VecDeque<RedefinableId>,
}

impl ComplexOffsetable for RedefineFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        self.redefineable.iter_mut().for_each(|redefinable| {
            redefinable.offset(target, new, offsets);
        });
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        self.redefineable.iter_mut().for_each(|redefinable| {
            redefinable.remap_namespace(old, new);
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RedefinableId {
    /// Complex type to redefine.
    ComplexType(FragmentIdx<ComplexTypeRootFragment>),
    /// Simple type to redefine.
    SimpleType(FragmentIdx<simple::SimpleTypeRootFragment>),
    /// Attribute group to redefine.
    AttributeGroup(FragmentIdx<TopLevelAttributeGroupFragment>),
    /// Group to redefine.
    Group(FragmentIdx<TopLevelGroupFragment>),
    Notation,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SchemaTopId {
    Redefinable(RedefinableId),
    Element(FragmentIdx<TopLevelElementFragment>),
    Attribute(FragmentIdx<TopLevelAttributeFragment>),
    //TODO
    Notation,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompositionId {
    Include(FragmentIdx<IncludeFragment>),
    Import(FragmentIdx<ImportFragment>),
    Redefine(FragmentIdx<RedefineFragment>),
    Override(FragmentIdx<OverrideFragment>),
    AnnotationId,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IncludeFragment {
    pub schema_location: String,
}

impl ComplexOffsetable for IncludeFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        _offsets: &IdOffsets,
    ) {
        // No specific offsetting needed for IncludeFragment
    }

    fn remap_namespace(
        &mut self,
        _old: &Option<XmlNamespace>,
        _new: &Option<XmlNamespace<'static>>,
    ) {
        // No specific namespace remapping needed for IncludeFragment
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportFragment {
    pub namespace: Option<XmlNamespace<'static>>,
    pub schema_location: Option<String>,
}

impl ComplexOffsetable for ImportFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        _offsets: &IdOffsets,
    ) {
        // No specific offsetting needed for ImportFragment
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        if self.namespace == *old {
            self.namespace = new.clone();
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OverrideFragment {
    pub schema_location: String,
    pub schema_tops: VecDeque<SchemaTopId>,
}

impl ComplexOffsetable for OverrideFragment {
    fn offset(
        &mut self,
        target: &FragmentedXsdDocumentIdx,
        new: &FragmentedXsdDocumentIdx,
        offsets: &IdOffsets,
    ) {
        for schema_top in &mut self.schema_tops {
            match schema_top {
                SchemaTopId::Redefinable(redefinable_id) => {
                    redefinable_id.offset(target, new, offsets);
                }
                SchemaTopId::Element(fragment_id) => {
                    fragment_id.offset(target, new, offsets);
                }
                SchemaTopId::Attribute(fragment_id) => {
                    fragment_id.offset(target, new, offsets);
                }
                SchemaTopId::Notation => {}
            }
        }
    }

    fn remap_namespace(&mut self, old: &Option<XmlNamespace>, new: &Option<XmlNamespace<'static>>) {
        for schema_top in &mut self.schema_tops {
            match schema_top {
                SchemaTopId::Redefinable(redefinable_id) => {
                    redefinable_id.remap_namespace(old, new)
                }
                SchemaTopId::Element(fragment_id) => fragment_id.remap_namespace(old, new),
                SchemaTopId::Attribute(fragment_id) => fragment_id.remap_namespace(old, new),
                SchemaTopId::Notation => {}
            }
        }
    }
}

/// Complex type fragment compiler responsible for converting XSD complex types to fragment representations.
#[derive(Debug, Clone)]
pub struct ComplexTypeFragmentCompiler {
    pub namespace_idx: FragmentedXsdDocumentIdx,
    /// The [`SimpleTypeFragmentCompiler`] for this complex type compiler.
    pub simple_type_compiler: SimpleTypeFragmentCompiler,
    complex_types: FragmentCollection<ComplexTypeRootFragment>,
    simple_restrictions: FragmentCollection<SimpleRestrictionFragment>,
    simple_extensions: FragmentCollection<SimpleExtensionFragment>,
    simple_contents: FragmentCollection<SimpleContentFragment>,
    restrictions: FragmentCollection<RestrictionFragment>,
    extensions: FragmentCollection<ExtensionFragment>,
    complex_contents: FragmentCollection<ComplexContentFragment>,
    group_refs: FragmentCollection<GroupRefFragment>,
    alls: FragmentCollection<AllFragment>,
    choices: FragmentCollection<ChoiceFragment>,
    pub sequences: FragmentCollection<SequenceFragment>,
    anys: FragmentCollection<AnyFragment>,
    elements: FragmentCollection<LocalElementFragment>,
    top_level_elements: FragmentCollection<TopLevelElementFragment>,
    local_attributes: FragmentCollection<LocalAttributeFragment>,
    top_level_attributes: FragmentCollection<TopLevelAttributeFragment>,
    attribute_group_refs: FragmentCollection<AttributeGroupRefFragment>,
    groups: FragmentCollection<TopLevelGroupFragment>,
    attribute_groups: FragmentCollection<TopLevelAttributeGroupFragment>,
    attribute_declarations: FragmentCollection<AttributeDeclarationsFragment>,
    any_attributes: FragmentCollection<AnyAttributeFragment>,
    assertions: FragmentCollection<AssertionFragment>,
    assertion_groups: FragmentCollection<AssertionsFragment>,
    redefines: FragmentCollection<RedefineFragment>,
    includes: FragmentCollection<IncludeFragment>,
    imports: FragmentCollection<ImportFragment>,
    overrides: FragmentCollection<OverrideFragment>,
}

impl HasFragmentCollection<ComplexTypeRootFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<ComplexTypeRootFragment> {
        &self.complex_types
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<ComplexTypeRootFragment> {
        &mut self.complex_types
    }
}

impl HasFragmentCollection<SimpleRestrictionFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<SimpleRestrictionFragment> {
        &self.simple_restrictions
    }
    fn get_fragment_collection_mut(
        &mut self,
    ) -> &mut FragmentCollection<SimpleRestrictionFragment> {
        &mut self.simple_restrictions
    }
}

impl HasFragmentCollection<SimpleExtensionFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<SimpleExtensionFragment> {
        &self.simple_extensions
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<SimpleExtensionFragment> {
        &mut self.simple_extensions
    }
}

impl HasFragmentCollection<SimpleContentFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<SimpleContentFragment> {
        &self.simple_contents
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<SimpleContentFragment> {
        &mut self.simple_contents
    }
}

impl HasFragmentCollection<ExtensionFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<ExtensionFragment> {
        &self.extensions
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<ExtensionFragment> {
        &mut self.extensions
    }
}

impl HasFragmentCollection<RestrictionFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<RestrictionFragment> {
        &self.restrictions
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<RestrictionFragment> {
        &mut self.restrictions
    }
}

impl HasFragmentCollection<ComplexContentFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<ComplexContentFragment> {
        &self.complex_contents
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<ComplexContentFragment> {
        &mut self.complex_contents
    }
}

impl HasFragmentCollection<GroupRefFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<GroupRefFragment> {
        &self.group_refs
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<GroupRefFragment> {
        &mut self.group_refs
    }
}

impl HasFragmentCollection<AllFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<AllFragment> {
        &self.alls
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<AllFragment> {
        &mut self.alls
    }
}

impl HasFragmentCollection<ChoiceFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<ChoiceFragment> {
        &self.choices
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<ChoiceFragment> {
        &mut self.choices
    }
}

impl HasFragmentCollection<SequenceFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<SequenceFragment> {
        &self.sequences
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<SequenceFragment> {
        &mut self.sequences
    }
}

impl HasFragmentCollection<AnyFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<AnyFragment> {
        &self.anys
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<AnyFragment> {
        &mut self.anys
    }
}

impl HasFragmentCollection<LocalElementFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<LocalElementFragment> {
        &self.elements
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<LocalElementFragment> {
        &mut self.elements
    }
}

impl HasFragmentCollection<TopLevelElementFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<TopLevelElementFragment> {
        &self.top_level_elements
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<TopLevelElementFragment> {
        &mut self.top_level_elements
    }
}

impl HasFragmentCollection<LocalAttributeFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<LocalAttributeFragment> {
        &self.local_attributes
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<LocalAttributeFragment> {
        &mut self.local_attributes
    }
}

impl HasFragmentCollection<TopLevelAttributeFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<TopLevelAttributeFragment> {
        &self.top_level_attributes
    }
    fn get_fragment_collection_mut(
        &mut self,
    ) -> &mut FragmentCollection<TopLevelAttributeFragment> {
        &mut self.top_level_attributes
    }
}

impl HasFragmentCollection<AttributeGroupRefFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<AttributeGroupRefFragment> {
        &self.attribute_group_refs
    }
    fn get_fragment_collection_mut(
        &mut self,
    ) -> &mut FragmentCollection<AttributeGroupRefFragment> {
        &mut self.attribute_group_refs
    }
}

impl HasFragmentCollection<TopLevelGroupFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<TopLevelGroupFragment> {
        &self.groups
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<TopLevelGroupFragment> {
        &mut self.groups
    }
}

impl HasFragmentCollection<TopLevelAttributeGroupFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<TopLevelAttributeGroupFragment> {
        &self.attribute_groups
    }
    fn get_fragment_collection_mut(
        &mut self,
    ) -> &mut FragmentCollection<TopLevelAttributeGroupFragment> {
        &mut self.attribute_groups
    }
}

impl HasFragmentCollection<AttributeDeclarationsFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<AttributeDeclarationsFragment> {
        &self.attribute_declarations
    }
    fn get_fragment_collection_mut(
        &mut self,
    ) -> &mut FragmentCollection<AttributeDeclarationsFragment> {
        &mut self.attribute_declarations
    }
}

impl HasFragmentCollection<AnyAttributeFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<AnyAttributeFragment> {
        &self.any_attributes
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<AnyAttributeFragment> {
        &mut self.any_attributes
    }
}

impl HasFragmentCollection<AssertionsFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<AssertionsFragment> {
        &self.assertion_groups
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<AssertionsFragment> {
        &mut self.assertion_groups
    }
}

impl HasFragmentCollection<AssertionFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<AssertionFragment> {
        &self.assertions
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<AssertionFragment> {
        &mut self.assertions
    }
}

impl HasFragmentCollection<RedefineFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<RedefineFragment> {
        &self.redefines
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<RedefineFragment> {
        &mut self.redefines
    }
}

impl HasFragmentCollection<IncludeFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<IncludeFragment> {
        &self.includes
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<IncludeFragment> {
        &mut self.includes
    }
}

impl HasFragmentCollection<ImportFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<ImportFragment> {
        &self.imports
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<ImportFragment> {
        &mut self.imports
    }
}

impl HasFragmentCollection<OverrideFragment> for ComplexTypeFragmentCompiler {
    fn get_fragment_collection(&self) -> &FragmentCollection<OverrideFragment> {
        &self.overrides
    }
    fn get_fragment_collection_mut(&mut self) -> &mut FragmentCollection<OverrideFragment> {
        &mut self.overrides
    }
}

impl<T: 'static> FragmentAccess<T> for ComplexTypeFragmentCompiler
where
    ComplexTypeFragmentCompiler: HasFragmentCollection<T>,
{
    fn get_fragment(&self, fragment_id: &FragmentIdx<T>) -> Option<&T> {
        if fragment_id.namespace_idx() != self.namespace_idx {
            return None;
        }

        self.get_fragment_collection().get_fragment(&fragment_id.1)
    }

    fn get_fragment_mut(&mut self, fragment_id: &FragmentIdx<T>) -> Option<&mut T> {
        if fragment_id.namespace_idx() != self.namespace_idx {
            return None;
        }
        self.get_fragment_collection_mut()
            .get_fragment_mut(&fragment_id.1)
    }

    fn push_fragment(&mut self, fragment: T) -> FragmentIdx<T> {
        let idx = self.get_fragment_collection_mut().push_fragment(fragment);
        FragmentIdx::new(self.namespace_idx, idx)
    }

    fn iter_fragment_ids(&self) -> Vec<FragmentIdx<T>> {
        self.get_fragment_collection()
            .iter_fragment_ids()
            .into_iter()
            .map(|idx| FragmentIdx::new(self.namespace_idx, idx))
            .collect()
    }
}

pub struct IdOffsets {
    simple: simple::IdOffsets,
    complex_type_roots_offset: usize,
    simple_restrictions_offset: usize,
    simple_extensions_offset: usize,
    simple_contents_offset: usize,
    restrictions_offset: usize,
    extensions_offset: usize,
    complex_contents_offset: usize,
    group_ref_offset: usize,
    all_offset: usize,
    choice_offset: usize,
    sequence_offset: usize,
    any_offset: usize,
    element_offset: usize,
    top_level_element_offset: usize,
    local_attribute_offset: usize,
    top_level_attribute_offset: usize,
    attribute_group_ref_offset: usize,
    top_level_group_offset: usize,
    top_level_attribute_group_offset: usize,
    attribute_declaration_offset: usize,
    any_attribute_offset: usize,
    assertion_offset: usize,
    assertion_group_offset: usize,
    includes: usize,
    redefines: usize,
    imports: usize,
    overrides: usize,
}

impl ComplexTypeFragmentCompiler {
    /// Creates a new [`ComplexTypeFragmentCompiler`] with the given namespace and namespace index.
    pub fn new(namespace_idx: FragmentedXsdDocumentIdx) -> Self {
        Self::new_with_simple_compiler(
            namespace_idx,
            SimpleTypeFragmentCompiler::new(namespace_idx),
        )
    }

    /// Creates a new [`ComplexTypeFragmentCompiler`] with a given [`SimpleTypeFragmentCompiler`].
    pub fn new_with_simple_compiler(
        namespace_idx: FragmentedXsdDocumentIdx,
        simple_type_compiler: SimpleTypeFragmentCompiler,
    ) -> Self {
        Self {
            simple_type_compiler,
            namespace_idx,
            complex_types: FragmentCollection::new(),
            simple_restrictions: FragmentCollection::new(),
            simple_extensions: FragmentCollection::new(),
            simple_contents: FragmentCollection::new(),
            restrictions: FragmentCollection::new(),
            extensions: FragmentCollection::new(),
            complex_contents: FragmentCollection::new(),
            group_refs: FragmentCollection::new(),
            alls: FragmentCollection::new(),
            choices: FragmentCollection::new(),
            sequences: FragmentCollection::new(),
            anys: FragmentCollection::new(),
            elements: FragmentCollection::new(),
            top_level_elements: FragmentCollection::new(),
            local_attributes: FragmentCollection::new(),
            top_level_attributes: FragmentCollection::new(),
            attribute_group_refs: FragmentCollection::new(),
            groups: FragmentCollection::new(),
            attribute_groups: FragmentCollection::new(),
            attribute_declarations: FragmentCollection::new(),
            any_attributes: FragmentCollection::new(),
            assertions: FragmentCollection::new(),
            assertion_groups: FragmentCollection::new(),
            includes: FragmentCollection::new(),
            redefines: FragmentCollection::new(),
            imports: FragmentCollection::new(),
            overrides: FragmentCollection::new(),
        }
    }

    pub fn merge_with(
        &mut self,
        other: &Self,
        old_target_namespace: &Option<XmlNamespace<'_>>,
        new_target_namespace: &Option<XmlNamespace<'static>>,
    ) -> Result<IdOffsets, Error> {
        let simple_merge_result = self.simple_type_compiler.merge_with(
            &other.simple_type_compiler,
            old_target_namespace,
            new_target_namespace,
        )?;

        let merge_result = IdOffsets {
            simple: simple_merge_result,
            complex_type_roots_offset: self.complex_types.len(),
            simple_restrictions_offset: self.simple_restrictions.len(),
            simple_extensions_offset: self.simple_extensions.len(),
            simple_contents_offset: self.simple_contents.len(),
            restrictions_offset: self.restrictions.len(),
            extensions_offset: self.extensions.len(),
            complex_contents_offset: self.complex_contents.len(),
            group_ref_offset: self.group_refs.len(),
            all_offset: self.alls.len(),
            choice_offset: self.choices.len(),
            sequence_offset: self.sequences.len(),
            any_offset: self.anys.len(),
            element_offset: self.elements.len(),
            top_level_element_offset: self.top_level_elements.len(),
            local_attribute_offset: self.local_attributes.len(),
            top_level_attribute_offset: self.top_level_attributes.len(),
            attribute_group_ref_offset: self.attribute_group_refs.len(),
            top_level_group_offset: self.groups.len(),
            top_level_attribute_group_offset: self.attribute_groups.len(),
            attribute_declaration_offset: self.attribute_declarations.len(),
            any_attribute_offset: self.any_attributes.len(),
            assertion_offset: self.assertions.len(),
            assertion_group_offset: self.assertion_groups.len(),
            includes: self.includes.len(),
            redefines: self.redefines.len(),
            imports: self.imports.len(),
            overrides: self.overrides.len(),
        };

        self.complex_types.merge_with(
            &other.complex_types,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.simple_restrictions.merge_with(
            &other.simple_restrictions,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.simple_extensions.merge_with(
            &other.simple_extensions,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.simple_contents.merge_with(
            &other.simple_contents,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.restrictions.merge_with(
            &other.restrictions,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.extensions.merge_with(
            &other.extensions,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.complex_contents.merge_with(
            &other.complex_contents,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.group_refs.merge_with(
            &other.group_refs,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.alls.merge_with(
            &other.alls,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.choices.merge_with(
            &other.choices,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.sequences.merge_with(
            &other.sequences,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.anys.merge_with(
            &other.anys,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.elements.merge_with(
            &other.elements,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.top_level_elements.merge_with(
            &other.top_level_elements,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.local_attributes.merge_with(
            &other.local_attributes,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.top_level_attributes.merge_with(
            &other.top_level_attributes,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.attribute_group_refs.merge_with(
            &other.attribute_group_refs,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.groups.merge_with(
            &other.groups,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.attribute_groups.merge_with(
            &other.attribute_groups,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.attribute_declarations.merge_with(
            &other.attribute_declarations,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.any_attributes.merge_with(
            &other.any_attributes,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.assertions.merge_with(
            &other.assertions,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.assertion_groups.merge_with(
            &other.assertion_groups,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.includes.merge_with(
            &other.includes,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.redefines.merge_with(
            &other.redefines,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.imports.merge_with(
            &other.imports,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        self.overrides.merge_with(
            &other.overrides,
            &other.namespace_idx,
            &self.namespace_idx,
            &merge_result,
            old_target_namespace,
            new_target_namespace,
        );

        Ok(merge_result)
    }
}

impl AsMut<SimpleTypeFragmentCompiler> for ComplexTypeFragmentCompiler {
    fn as_mut(&mut self) -> &mut SimpleTypeFragmentCompiler {
        &mut self.simple_type_compiler
    }
}

impl AsMut<ComplexTypeFragmentCompiler> for ComplexTypeFragmentCompiler {
    fn as_mut(&mut self) -> &mut ComplexTypeFragmentCompiler {
        self
    }
}

impl AsRef<SimpleTypeFragmentCompiler> for ComplexTypeFragmentCompiler {
    fn as_ref(&self) -> &SimpleTypeFragmentCompiler {
        &self.simple_type_compiler
    }
}

impl AsRef<ComplexTypeFragmentCompiler> for ComplexTypeFragmentCompiler {
    fn as_ref(&self) -> &ComplexTypeFragmentCompiler {
        self
    }
}

/// Error types for complex fragment operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    /// Error from simple type processing.
    #[error("Error in simple type processing: {0}")]
    Simple(simple::Error),
    /// Name is missing in top-level declaration.
    #[error("Name is missing in top-level declaration")]
    NameMissingInTopLevel,
    /// Element type is invalid, both type and type_choice are present.
    #[error("Element type is invalid, both type and type_choice are present: {name}")]
    TypeAttributeAndTypeContentBothPresent {
        /// Name of the element with conflicting type representations.
        name: LocalName<'static>,
    },
    /// Element type is invalid, neither type nor type_choice are present.
    #[error("Element type is invalid, neither type nor type_choice are present: {name}")]
    NoTypePresent {
        /// Name of the element without a type.
        name: LocalName<'static>,
    },
    /// Name or reference is missing in a top-level element.
    #[error("Name or reference is missing in a top-level element")]
    NameOrRefMissingInTopLevelElement {},
    /// Substitution group is not supported.
    #[error("Substitution group not supported for element type: {fragment_type}")]
    SubstitutionGroupNotSupported {
        /// Name of the element with unsupported substitution group.
        fragment_type: &'static str,
    },
}

impl From<simple::Error> for Error {
    fn from(err: simple::Error) -> Self {
        Error::Simple(err)
    }
}

/// Trait for types that can be converted to and from [`ComplexTypeFragmentCompiler`] fragments.
pub trait ComplexFragmentEquivalent: Sized {
    /// The identifier type for the fragment.
    type FragmentId;

    /// Converts this type to complex fragments.
    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error>;

    /// Reconstructs this type from complex fragments.
    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error>;
}

/// Identifier for element type content (simple or complex type).
#[derive(Debug, Clone, PartialEq)]
pub enum ElementTypeContentId {
    /// Simple type content.
    SimpleType(FragmentIdx<simple::SimpleTypeRootFragment>),
    /// Complex type content.
    ComplexType(FragmentIdx<ComplexTypeRootFragment>),
}

impl ComplexFragmentEquivalent for xs::types::top_level_element_items::Type {
    type FragmentId = ElementTypeContentId;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        match self {
            xs::types::top_level_element_items::Type::SimpleType(local_simple_type) => {
                let simple_type_fragment =
                    local_simple_type.to_simple_fragments(compiler.as_mut(), context)?;

                Ok(ElementTypeContentId::SimpleType(simple_type_fragment))
            }
            xs::types::top_level_element_items::Type::ComplexType(local_complex_type) => {
                let complex_type_fragment =
                    local_complex_type.to_complex_fragments(compiler, context)?;

                Ok(ElementTypeContentId::ComplexType(complex_type_fragment))
            }
        }
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        match fragment_id {
            ElementTypeContentId::SimpleType(fragment_id) => {
                xs::types::LocalSimpleType::from_simple_fragments(compiler.as_ref(), fragment_id)
                    .map(Box::new)
                    .map(xs::types::top_level_element_items::Type::SimpleType)
                    .map_err(From::from)
            }
            ElementTypeContentId::ComplexType(fragment_idx) => {
                xs::types::LocalComplexType::from_complex_fragments(compiler, fragment_idx)
                    .map(Box::new)
                    .map(xs::types::top_level_element_items::Type::ComplexType)
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::types::local_element_items::Type {
    type FragmentId = ElementTypeContentId;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        match self {
            xs::types::local_element_items::Type::SimpleType(local_simple_type) => {
                let simple_type_fragment = local_simple_type
                    .to_simple_fragments(compiler.as_mut(), context)
                    .expect("Failed to convert simple type to fragments");

                Ok(ElementTypeContentId::SimpleType(simple_type_fragment))
            }
            xs::types::local_element_items::Type::ComplexType(local_complex_type) => {
                let complex_type_fragment =
                    local_complex_type.to_complex_fragments(compiler, context)?;

                Ok(ElementTypeContentId::ComplexType(complex_type_fragment))
            }
        }
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        match fragment_id {
            ElementTypeContentId::SimpleType(fragment_id) => {
                xs::types::LocalSimpleType::from_simple_fragments(compiler.as_ref(), fragment_id)
                    .map(Box::new)
                    .map(xs::types::local_element_items::Type::SimpleType)
                    .map_err(From::from)
            }
            ElementTypeContentId::ComplexType(fragment_idx) => {
                xs::types::LocalComplexType::from_complex_fragments(compiler, fragment_idx)
                    .map(Box::new)
                    .map(xs::types::local_element_items::Type::ComplexType)
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::types::LocalElement {
    type FragmentId = FragmentIdx<LocalElementFragment>;

    fn to_complex_fragments(
        &self,
        mut compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let max_occurs = self.max_occurs.clone().map(|a| AllNNI::from(*a));
        let min_occurs = self.min_occurs;

        let type_ = if let Some(ref_) = self.ref_.as_ref() {
            if self.name.is_some() {
                //TODO
            }

            LocalElementFragmentType::Reference(ReferenceElementFragment {
                ref_: ref_
                    .0
                    .clone()
                    .with_default_namespace(|| context.default_namespace.cloned()),
            })
        } else if let Some(name) = self.name.clone() {
            let type_ = if let Some(type_) = self.type_attribute.as_ref() {
                NamedOrAnonymous::Named(
                    type_
                        .0
                        .clone()
                        .with_default_namespace(|| context.default_namespace.cloned()),
                )
            } else if let Some(type_choice) = self.type_.as_ref() {
                let content_type = type_choice.to_complex_fragments(&mut compiler, context)?;

                NamedOrAnonymous::Anonymous(content_type)
            } else {
                return Err(Error::NoTypePresent { name: name.clone() });
            };

            LocalElementFragmentType::Local(DeclaredElementFragment { name, type_ })
        } else {
            return Err(Error::NameOrRefMissingInTopLevelElement {});
        };

        Ok(compiler.push_fragment(LocalElementFragment {
            type_,
            max_occurs,
            min_occurs,
        }))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let element_builder = xs::types::LocalElement::builder()
            .maybe_min_occurs(fragment.min_occurs)
            .maybe_max_occurs(fragment.max_occurs.map(From::from).map(Box::new));

        match &fragment.type_ {
            LocalElementFragmentType::Local(fragment) => Ok(element_builder
                .name(fragment.name.clone())
                .maybe_type_attribute(match &fragment.type_ {
                    NamedOrAnonymous::Named(expanded_name) => {
                        Some(xs::types::QName(expanded_name.clone()))
                    }
                    NamedOrAnonymous::Anonymous(_) => None,
                })
                .maybe_type_(
                    match &fragment.type_ {
                        NamedOrAnonymous::Anonymous(content_type) => Some(
                            xs::types::local_element_items::Type::from_complex_fragments(
                                compiler,
                                content_type,
                            ),
                        ),
                        NamedOrAnonymous::Named(_) => None,
                    }
                    .transpose()?,
                )
                .any_attributes(ns::AnyAttributes::default())
                .build()),
            LocalElementFragmentType::Reference(fragment) => Ok(element_builder
                .ref_(xs::types::QName(fragment.ref_.clone()))
                .any_attributes(ns::AnyAttributes::default())
                .build()),
        }
    }
}

impl ComplexFragmentEquivalent for xs::types::TopLevelElement {
    type FragmentId = FragmentIdx<TopLevelElementFragment>;

    fn to_complex_fragments(
        &self,
        mut compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let name = self.name.clone();

        let type_ = match (self.type_attribute.as_ref(), self.type_.as_ref()) {
            (Some(type_), None) => Some(NamedOrAnonymous::Named(
                type_
                    .0
                    .clone()
                    .with_default_namespace(|| context.default_namespace.cloned()),
            )),
            (None, Some(type_choice)) => {
                let content_type = type_choice.to_complex_fragments(&mut compiler, context)?;
                Some(NamedOrAnonymous::Anonymous(content_type))
            }
            (Some(_), Some(_)) => {
                return Err(Error::TypeAttributeAndTypeContentBothPresent { name })
            }
            (None, None) => None,
        };

        let substitution_groups = self
            .substitution_group
            .iter()
            .flat_map(|list| list.0.iter())
            .map(|expanded_name| expanded_name.0.clone())
            .collect();

        Ok(compiler.push_fragment(TopLevelElementFragment {
            name,
            type_,
            substitution_groups,
            abstract_: self.abstract_.unwrap_or_default(),
        }))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        Ok(xs::types::TopLevelElement::builder()
            .name(fragment.name.clone())
            .maybe_type_attribute(fragment.type_.as_ref().and_then(|f| match f {
                NamedOrAnonymous::Named(expanded_name) => {
                    Some(xs::types::QName(expanded_name.clone()))
                }
                NamedOrAnonymous::Anonymous(_) => None,
            }))
            .maybe_type_(
                fragment
                    .type_
                    .as_ref()
                    .and_then(|f| match f {
                        NamedOrAnonymous::Anonymous(content_type) => Some(
                            xs::types::top_level_element_items::Type::from_complex_fragments(
                                compiler,
                                content_type,
                            ),
                        ),
                        NamedOrAnonymous::Named(_) => None,
                    })
                    .transpose()?,
            )
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::types::GroupRef {
    type FragmentId = FragmentIdx<GroupRefFragment>;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let ref_ = self
            .ref_
            .0
            .clone()
            .with_default_namespace(|| context.default_namespace.cloned());

        Ok(compiler.push_fragment(GroupRefFragment {
            min_occurs: self.min_occurs,
            max_occurs: self.max_occurs.clone().map(|a| AllNNI::from(*a)),
            ref_,
        }))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        Ok(xs::types::GroupRef {
            id: None,
            min_occurs: fragment.min_occurs,
            max_occurs: fragment.max_occurs.map(From::from).map(Box::new),
            ref_: xs::types::QName(fragment.ref_.clone()),
            annotation: None,
            any_attributes: ns::AnyAttributes::default(),
        })
    }
}

impl ComplexFragmentEquivalent for xs::Any {
    type FragmentId = FragmentIdx<AnyFragment>;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        _context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let xs::Any::Any(any) = self else {
            return Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            });
        };

        Ok(compiler.push_fragment(AnyFragment {
            id: any.id.clone(),
            process_contents: any.process_contents.map(AnyProcessContents::from),
        }))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        Ok(xs::Any::from(
            xs::any_items::Any::builder()
                .maybe_id(fragment.id.clone())
                .maybe_process_contents(fragment.process_contents.map(AnyProcessContents::into))
                .build(),
        ))
    }
}

/// Identifier for nested particles within complex type content.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NestedParticleId {
    /// Element particle.
    Element(FragmentIdx<LocalElementFragment>),
    /// Group reference particle.
    Group(FragmentIdx<GroupRefFragment>),
    /// Choice particle.
    Choice(FragmentIdx<ChoiceFragment>),
    /// Sequence particle.
    Sequence(FragmentIdx<SequenceFragment>),
    /// Any element wildcard particle.
    Any(FragmentIdx<AnyFragment>),
}

impl TryFrom<TypeDefParticleId> for NestedParticleId {
    type Error = FragmentIdx<AllFragment>;

    fn try_from(value: TypeDefParticleId) -> Result<Self, Self::Error> {
        match value {
            TypeDefParticleId::Group(fragment_idx) => Ok(Self::Group(fragment_idx)),
            TypeDefParticleId::All(fragment_idx) => return Err(fragment_idx),
            TypeDefParticleId::Choice(fragment_idx) => Ok(Self::Choice(fragment_idx)),
            TypeDefParticleId::Sequence(fragment_idx) => Ok(Self::Sequence(fragment_idx)),
        }
    }
}

impl From<FragmentIdx<LocalElementFragment>> for NestedParticleId {
    fn from(value: FragmentIdx<LocalElementFragment>) -> Self {
        Self::Element(value)
    }
}
impl From<FragmentIdx<GroupRefFragment>> for NestedParticleId {
    fn from(value: FragmentIdx<GroupRefFragment>) -> Self {
        Self::Group(value)
    }
}
impl From<FragmentIdx<ChoiceFragment>> for NestedParticleId {
    fn from(value: FragmentIdx<ChoiceFragment>) -> Self {
        Self::Choice(value)
    }
}
impl From<FragmentIdx<SequenceFragment>> for NestedParticleId {
    fn from(value: FragmentIdx<SequenceFragment>) -> Self {
        Self::Sequence(value)
    }
}

impl From<FragmentIdx<AnyFragment>> for NestedParticleId {
    fn from(value: FragmentIdx<AnyFragment>) -> Self {
        Self::Any(value)
    }
}

impl ComplexFragmentEquivalent for xs::groups::NestedParticle {
    type FragmentId = NestedParticleId;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let compiler: &mut ComplexTypeFragmentCompiler = compiler.as_mut();

        use xs::groups::NestedParticle;

        match self {
            NestedParticle::Element(local_element) => local_element
                .to_complex_fragments(compiler, context)
                .map(NestedParticleId::Element),
            NestedParticle::Group(group_type) => group_type
                .to_complex_fragments(compiler, context)
                .map(NestedParticleId::Group),
            NestedParticle::Choice(choice_type) => choice_type
                .to_complex_fragments(compiler, context)
                .map(NestedParticleId::Choice),
            NestedParticle::Sequence(sequence_type) => sequence_type
                .to_complex_fragments(compiler, context)
                .map(NestedParticleId::Sequence),
            NestedParticle::Any(any) => any
                .to_complex_fragments(compiler, context)
                .map(NestedParticleId::Any),
        }
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        match fragment_id {
            NestedParticleId::Element(fragment_idx) => {
                xs::types::LocalElement::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::groups::NestedParticle::from)
            }
            NestedParticleId::Group(fragment_idx) => {
                xs::types::GroupRef::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::groups::NestedParticle::from)
            }
            NestedParticleId::Choice(fragment_idx) => {
                xs::Choice::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::groups::NestedParticle::from)
            }
            NestedParticleId::Sequence(fragment_idx) => {
                xs::Sequence::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::groups::NestedParticle::from)
            }
            NestedParticleId::Any(_) => {
                unreachable!()
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::groups::all_model_items::Child1 {
    type FragmentId = NestedParticleId;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let compiler: &mut ComplexTypeFragmentCompiler = compiler.as_mut();

        match self {
            xs::groups::all_model_items::Child1::Element(local_element) => local_element
                .to_complex_fragments(compiler, context)
                .map(NestedParticleId::Element),
            xs::groups::all_model_items::Child1::Any(any) => any
                .to_complex_fragments(compiler, context)
                .map(NestedParticleId::Any),
            xs::groups::all_model_items::Child1::Group {
                id: _,
                ref_,
                min_occurs,
                max_occurs,
                annotation: _,
            } => Ok(compiler
                .push_fragment(GroupRefFragment {
                    min_occurs: *min_occurs,
                    max_occurs: max_occurs.map(AllNNI::Bounded),
                    ref_: ref_
                        .0
                        .clone()
                        .with_default_namespace(|| context.default_namespace.cloned()),
                })
                .into()),
        }
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        match fragment_id {
            NestedParticleId::Element(fragment_idx) => {
                xs::types::LocalElement::from_complex_fragments(compiler, fragment_idx)
                    .map(Box::new)
                    .map(xs::groups::all_model_items::Child1::Element)
            }
            NestedParticleId::Any(fragment_idx) => {
                xs::Any::from_complex_fragments(compiler, fragment_idx)
                    .map(Box::new)
                    .map(xs::groups::all_model_items::Child1::Any)
            }
            NestedParticleId::Group(fragment_idx) => {
                let fragment = compiler
                    .get_fragment(fragment_idx)
                    .expect("Fragment not found in compiler.");

                Ok(xs::groups::all_model_items::Child1::Group {
                    id: None,
                    ref_: xs::types::QName(fragment.ref_.clone()),
                    min_occurs: fragment.min_occurs,
                    max_occurs: fragment.max_occurs.map(|a| match a {
                        AllNNI::Bounded(n) => n,
                        AllNNI::Unbounded => panic!("Unbounded not allowed in all groups"),
                    }),
                    annotation: None,
                })
            }
            _ => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: type_name::<Self>(),
                });
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::All {
    type FragmentId = FragmentIdx<AllFragment>;

    fn to_complex_fragments(
        &self,
        mut compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let xs::All::All(all) = self else {
            return Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            });
        };

        let fragments = all
            .all_model
            .child_1
            .iter()
            .map(|content| content.to_complex_fragments(&mut compiler, context))
            .collect::<Result<_, _>>()?;

        let all = AllFragment {
            min_occurs: None,
            max_occurs: None,
            fragments,
        };

        Ok(compiler.push_fragment(all))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let child_1 = fragment
            .fragments
            .iter()
            .map(|nested_id| {
                xs::groups::all_model_items::Child1::from_complex_fragments(compiler, nested_id)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(xs::All::All(Box::new(
            xs::types::All::builder()
                .maybe_min_occurs(
                    fragment
                        .min_occurs
                        .map(|min| min.try_into().expect("Invalid min occurs value")),
                )
                .maybe_max_occurs(fragment.max_occurs.map(Into::into))
                .all_model(
                    xs::groups::AllModel::builder()
                        .child_1(child_1)
                        .build()
                        .into(),
                )
                .any_attributes(ns::AnyAttributes::default())
                .build(),
        )))
    }
}

impl ComplexFragmentEquivalent for xs::Choice {
    type FragmentId = FragmentIdx<ChoiceFragment>;

    fn to_complex_fragments(
        &self,
        mut compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let xs::Choice::Choice(choice) = self else {
            return Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            });
        };

        let all = ChoiceFragment {
            min_occurs: choice.min_occurs,
            max_occurs: choice.max_occurs.clone().map(|a| AllNNI::from(*a)),
            fragments: choice
                .nested_particle
                .iter()
                .map(|content| content.to_complex_fragments(&mut compiler, context))
                .collect::<Result<_, _>>()?,
        };

        Ok(compiler.push_fragment(all))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        Ok(xs::Choice::from(
            xs::types::ExplicitGroup::builder()
                .maybe_min_occurs(fragment.min_occurs)
                .maybe_max_occurs(fragment.max_occurs.map(From::from).map(Box::new))
                .nested_particle(
                    fragment
                        .fragments
                        .iter()
                        .map(|fragment| {
                            xs::groups::NestedParticle::from_complex_fragments(compiler, fragment)
                        })
                        .collect::<Result<_, _>>()?,
                )
                .any_attributes(ns::AnyAttributes::default())
                .build(),
        ))
    }
}

impl ComplexFragmentEquivalent for xs::Sequence {
    type FragmentId = FragmentIdx<SequenceFragment>;

    fn to_complex_fragments(
        &self,
        mut compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let xs::Sequence::Sequence(sequence) = self else {
            return Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            });
        };

        let seq = SequenceFragment {
            id: sequence.id.clone(),
            min_occurs: sequence.min_occurs,
            max_occurs: sequence.max_occurs.clone().map(|a| AllNNI::from(*a)),
            fragments: sequence
                .nested_particle
                .iter()
                .map(|content| content.to_complex_fragments(&mut compiler, context))
                .collect::<Result<_, _>>()?,
        };

        Ok(compiler.push_fragment(seq))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        Ok(xs::Sequence::from(
            xs::types::ExplicitGroup::builder()
                .maybe_min_occurs(fragment.min_occurs)
                .maybe_max_occurs(fragment.max_occurs.map(From::from).map(Box::new))
                .nested_particle(
                    fragment
                        .fragments
                        .iter()
                        .map(|fragment| {
                            xs::groups::NestedParticle::from_complex_fragments(compiler, fragment)
                        })
                        .collect::<Result<_, _>>()?,
                )
                .any_attributes(ns::AnyAttributes::default())
                .build(),
        ))
    }
}

impl ComplexFragmentEquivalent for xs::groups::TypeDefParticle {
    type FragmentId = TypeDefParticleId;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        use xs::groups::TypeDefParticle;

        match self {
            TypeDefParticle::Group(group_ref) => group_ref
                .to_complex_fragments(compiler, context)
                .map(TypeDefParticleId::Group),
            TypeDefParticle::All(all) => all
                .to_complex_fragments(compiler, context)
                .map(TypeDefParticleId::All),
            TypeDefParticle::Choice(choice) => choice
                .to_complex_fragments(compiler, context)
                .map(TypeDefParticleId::Choice),
            TypeDefParticle::Sequence(sequence) => sequence
                .to_complex_fragments(compiler, context)
                .map(TypeDefParticleId::Sequence),
        }
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        match fragment_id {
            TypeDefParticleId::Group(group_ref) => {
                let group_ref = xs::types::GroupRef::from_complex_fragments(compiler, group_ref)?;
                Ok(xs::groups::TypeDefParticle::Group(Box::new(group_ref)))
            }
            TypeDefParticleId::All(all) => {
                let all = xs::All::from_complex_fragments(compiler, all)?;
                Ok(xs::groups::TypeDefParticle::All(Box::new(all)))
            }
            TypeDefParticleId::Choice(choice) => {
                let choice = xs::Choice::from_complex_fragments(compiler, choice)?;
                Ok(xs::groups::TypeDefParticle::Choice(Box::new(choice)))
            }
            TypeDefParticleId::Sequence(sequence) => {
                let sequence = xs::Sequence::from_complex_fragments(compiler, sequence)?;
                Ok(xs::groups::TypeDefParticle::Sequence(Box::new(sequence)))
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::types::Assertion {
    type FragmentId = FragmentIdx<AssertionFragment>;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        _context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let fragment = AssertionFragment {
            id: self.id.clone(),
            test: self.test.clone(),
        };

        Ok(compiler.push_fragment(fragment))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        Ok(xs::types::Assertion::builder()
            .maybe_id(fragment.id.clone())
            .maybe_test(fragment.test.clone())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::groups::Assertions {
    type FragmentId = FragmentIdx<AssertionsFragment>;

    fn to_complex_fragments(
        &self,
        mut compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let assertions = self
            .assert
            .iter()
            .map(|assertion| assertion.0.to_complex_fragments(&mut compiler, context))
            .collect::<Result<_, _>>()?;

        let root_fragment = AssertionsFragment { assertions };

        Ok(compiler.push_fragment(root_fragment))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let assertions = fragment
            .assertions
            .iter()
            .map(|assertion_id| {
                xs::types::Assertion::from_complex_fragments(compiler, assertion_id)
                    .map(xs::groups::assertions_items::Assert::from)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(xs::groups::Assertions::builder()
            .assert(assertions)
            .build()
            .into())
    }
}

impl ComplexFragmentEquivalent for xs::types::ExtensionType {
    type FragmentId = FragmentIdx<ExtensionFragment>;

    /// This method expects all references to already be defined.
    fn to_complex_fragments(
        &self,
        mut compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let base = self.base.0.clone();

        let content_fragment = self
            .type_def_particle
            .as_ref()
            .map(|content| content.to_complex_fragments(&mut compiler, context))
            .transpose()?;

        let attribute_declarations = self
            .attr_decls
            .to_complex_fragments(&mut compiler, context)?;

        let assertions = self
            .assertions
            .to_complex_fragments(&mut compiler, context)?;

        let root_fragment = ExtensionFragment {
            base,
            content_fragment,
            attribute_declarations,
            assertions,
        };

        Ok(compiler.push_fragment(root_fragment))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let extension = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let particle = extension
            .content_fragment
            .as_ref()
            .map(|fragment_id| {
                xs::groups::TypeDefParticle::from_complex_fragments(compiler, fragment_id)
                    .map(Box::new)
            })
            .transpose()?;

        let attr_decls = xs::groups::AttrDecls::from_complex_fragments(
            compiler,
            &extension.attribute_declarations,
        )?;

        let assertions =
            xs::groups::Assertions::from_complex_fragments(compiler, &extension.assertions)?;

        Ok(Self::builder()
            .base(xs::types::QName(extension.base.clone()))
            .maybe_type_def_particle(particle)
            .attr_decls(attr_decls.into())
            .assertions(assertions.into())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::types::ComplexRestrictionType {
    type FragmentId = FragmentIdx<RestrictionFragment>;

    fn to_complex_fragments(
        &self,
        mut compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let base = self.base.0.clone();

        let content_fragment = self
            .child_1
            .as_ref()
            .map(|particle| {
                particle
                    .type_def_particle
                    .to_complex_fragments(&mut compiler, context)
            })
            .transpose()?;

        let attribute_declarations = self
            .attr_decls
            .to_complex_fragments(&mut compiler, context)?;

        let assertions = self
            .assertions
            .to_complex_fragments(&mut compiler, context)?;

        let root_fragment = RestrictionFragment {
            base,
            content_fragment,
            attribute_declarations,
            assertions,
        };

        Ok(compiler.push_fragment(root_fragment))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let particle = fragment
            .content_fragment
            .map(|a| xs::groups::TypeDefParticle::from_complex_fragments(compiler, &a))
            .transpose()?;

        let attr_decls = xs::groups::AttrDecls::from_complex_fragments(
            compiler,
            &fragment.attribute_declarations,
        )?;

        let assertions =
            xs::groups::Assertions::from_complex_fragments(compiler, &fragment.assertions)?;

        Ok(xs::types::ComplexRestrictionType::builder()
            .base(xs::types::QName(fragment.base.clone()))
            .maybe_child_1(particle.map(|particle| {
                xs::types::complex_restriction_type_items::Child1 {
                    open_content: None,
                    type_def_particle: Box::new(particle),
                }
            }))
            .attr_decls(attr_decls.into())
            .assertions(assertions.into())
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::groups::attr_decls_items::Attribute {
    type FragmentId = AttributeDeclarationId;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        use xs::groups::attr_decls_items::Attribute;

        match self {
            Attribute::Attribute(local) => Ok(AttributeDeclarationId::Attribute(
                local.to_complex_fragments(compiler, context)?,
            )),
            Attribute::AttributeGroup(group) => Ok(AttributeDeclarationId::AttributeGroupRef(
                group.to_complex_fragments(compiler, context)?,
            )),
        }
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        use xs::groups::attr_decls_items::Attribute;

        match fragment_id {
            AttributeDeclarationId::Attribute(fragment_idx) => {
                xs::types::Attribute::from_complex_fragments(compiler, fragment_idx)
                    .map(Attribute::from)
            }
            AttributeDeclarationId::AttributeGroupRef(fragment_idx) => {
                xs::types::AttributeGroupRef::from_complex_fragments(compiler, fragment_idx)
                    .map(Attribute::from)
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::types::Attribute {
    type FragmentId = FragmentIdx<LocalAttributeFragment>;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let type_mode = if let Some(ref ref_) = self.ref_ {
            LocalAttributeFragmentTypeMode::Reference(ReferenceAttributeFragment {
                ref_: ref_
                    .0
                    .clone()
                    .with_default_namespace(|| context.default_namespace.cloned()),
            })
        } else {
            let name = self
                .name
                .as_ref()
                .expect("name is required if not a reference");

            let type_ = if let Some(type_) = self.type_.as_ref() {
                Some(NamedOrAnonymous::Named(type_.0.clone()))
            } else {
                self.simple_type
                    .as_ref()
                    .map(|simple_type| {
                        simple_type
                            .to_simple_fragments(compiler.as_mut(), context)
                            .map(NamedOrAnonymous::Anonymous)
                    })
                    .transpose()?
            };

            LocalAttributeFragmentTypeMode::Declared(DeclaredAttributeFragment {
                name: name.clone(),
                type_,
            })
        };
        Ok(compiler.push_fragment(LocalAttributeFragment {
            type_mode,
            use_: self.use_.map(Into::into),
            default: self.default.clone(),
        }))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let builder = xs::types::Attribute::builder()
            .maybe_use_(fragment.use_.map(Into::into))
            .maybe_default(fragment.default.clone());

        match &fragment.type_mode {
            LocalAttributeFragmentTypeMode::Declared(local) => {
                let name = local.name.clone();
                let type_ = local.type_.as_ref().and_then(|a| match a {
                    NamedOrAnonymous::Named(ref_) => Some(xs::types::QName(ref_.clone())),
                    NamedOrAnonymous::Anonymous(_) => None,
                });
                Ok(builder.name(name).maybe_type_(type_).build())
            }
            LocalAttributeFragmentTypeMode::Reference(ref_) => {
                Ok(builder.ref_(xs::types::QName(ref_.ref_.clone())).build())
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::types::TopLevelAttribute {
    type FragmentId = FragmentIdx<TopLevelAttributeFragment>;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let name = self.name.clone();

        let type_ = match (self.type_.as_ref(), self.simple_type.as_ref()) {
            (None, Some(s)) => Some(NamedOrAnonymous::Anonymous(
                s.to_simple_fragments(compiler.as_mut(), context)?,
            )),
            (Some(t), None) => Some(NamedOrAnonymous::Named(t.0.clone())),
            (Some(_), Some(_)) => {
                return Err(Error::SubstitutionGroupNotSupported {
                    fragment_type: "TopLevelAttribute with both type and simpleType",
                });
            }
            (None, None) => None,
        };

        Ok(compiler.push_fragment(TopLevelAttributeFragment {
            name,
            type_,
            default_: self.default.clone(),
        }))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let name = fragment.name.clone();
        let type_ = fragment.type_.as_ref().and_then(|a| match a {
            NamedOrAnonymous::Named(ref_) => Some(xs::types::QName(ref_.clone())),
            NamedOrAnonymous::Anonymous(_) => None,
        });
        Ok(xs::types::TopLevelAttribute::builder()
            .name(name)
            .maybe_type_(type_)
            .any_attributes(ns::AnyAttributes::default())
            .maybe_default(fragment.default_.clone())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::types::AttributeGroupRef {
    type FragmentId = FragmentIdx<AttributeGroupRefFragment>;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        Ok(compiler.push_fragment(AttributeGroupRefFragment {
            ref_: self
                .ref_
                .0
                .clone()
                .with_default_namespace(|| context.default_namespace.cloned()),
        }))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        Ok(xs::types::AttributeGroupRef::builder()
            .ref_(xs::types::QName(fragment.ref_.clone()))
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::types::SimpleExtensionType {
    type FragmentId = FragmentIdx<SimpleExtensionFragment>;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let attribute_declarations = self.attr_decls.to_complex_fragments(compiler, context)?;

        let assertions = self.assertions.to_complex_fragments(compiler, context)?;

        Ok(compiler.push_fragment(SimpleExtensionFragment {
            base: self
                .base
                .0
                .clone()
                .with_default_namespace(|| context.default_namespace.cloned()),
            attribute_declarations,
            assertions,
        }))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let attr_decls = xs::groups::AttrDecls::from_complex_fragments(
            compiler,
            &fragment.attribute_declarations,
        )?;

        let assertions =
            xs::groups::Assertions::from_complex_fragments(compiler, &fragment.assertions)?;

        Ok(Self::builder()
            .base(xs::types::QName(fragment.base.clone()))
            .attr_decls(attr_decls.into())
            .assertions(assertions.into())
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::types::SimpleRestrictionType {
    type FragmentId = FragmentIdx<SimpleRestrictionFragment>;

    fn to_complex_fragments(
        &self,
        mut compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let base = self
            .base
            .0
            .clone()
            .with_default_namespace(|| context.default_namespace.cloned());

        let simple_restriction_model = self
            .simple_restriction_model
            .as_ref()
            .map(|a| &a.simple_restriction_model);

        let simple_type = simple_restriction_model
            .and_then(|a| a.simple_type.as_ref())
            .map(|simple_type| simple_type.to_simple_fragments(compiler.as_mut(), context))
            .transpose()?;

        let facets = simple_restriction_model
            .map(|a| {
                a.child_1
                    .iter()
                    .filter_map(|a| match a {
                        xs::groups::simple_restriction_model_items::Child1::Facet(facet) => {
                            Some(facet)
                        }
                        _ => None,
                    })
                    .map(|facet| facet.to_simple_fragments(compiler.as_mut(), context))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?
            .unwrap_or_default();

        let attribute_declarations = self
            .attr_decls
            .to_complex_fragments(&mut compiler, context)?;

        let assertions = self
            .assertions
            .to_complex_fragments(&mut compiler, context)?;

        Ok(compiler.push_fragment(SimpleRestrictionFragment {
            base,
            simple_type,
            facets,
            attribute_declarations,
            assertions,
            id: self.id.clone(),
        }))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let simple_restriction_model = (fragment.simple_type.is_some()
            || !fragment.facets.is_empty())
        .then(|| {
            let simple_type = fragment
                .simple_type
                .as_ref()
                .map(|simple_type| {
                    xs::types::LocalSimpleType::from_simple_fragments(
                        compiler.as_ref(),
                        simple_type,
                    )
                })
                .transpose()?;

            let facets = fragment
                .facets
                .iter()
                .map(|facet| xs::Facet::from_simple_fragments(compiler.as_ref(), facet))
                .collect::<Result<Vec<_>, _>>()?;

            let child_1 = (!facets.is_empty()).then(|| {
                facets
                    .into_iter()
                    .map(xs::groups::simple_restriction_model_items::Child1::from)
                    .collect()
            });

            Result::<_, Error>::Ok(
                xs::types::simple_restriction_type_items::SimpleRestrictionModel::builder()
                    .simple_restriction_model(Box::new(
                        xs::groups::SimpleRestrictionModel::builder()
                            .maybe_simple_type(simple_type.map(Box::new))
                            .maybe_child_1(child_1)
                            .build(),
                    ))
                    .build(),
            )
        })
        .transpose()?;

        let attr_decls = xs::groups::AttrDecls::from_complex_fragments(
            compiler,
            &fragment.attribute_declarations,
        )?;

        let assertions =
            xs::groups::Assertions::from_complex_fragments(compiler, &fragment.assertions)?;

        Ok(Self::builder()
            .base(xs::types::QName(fragment.base.clone()))
            .maybe_id(fragment.id.clone())
            .maybe_simple_restriction_model(simple_restriction_model)
            .attr_decls(attr_decls.into())
            .assertions(assertions.into())
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::SimpleContent {
    type FragmentId = FragmentIdx<SimpleContentFragment>;

    /// This method expects all references to already be defined.
    fn to_complex_fragments(
        &self,
        mut compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let xs::SimpleContent::SimpleContent(simple_content) = self else {
            return Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            });
        };

        let content_fragment = match &simple_content.child_1 {
            xs::simple_content_items::Child1::Extension(extension) => {
                let fragment_id = extension.to_complex_fragments(&mut compiler, context)?;

                SimpleContentChildId::Extension(fragment_id)
            }
            xs::simple_content_items::Child1::Restriction(restriction) => {
                let fragment_id = restriction.to_complex_fragments(&mut compiler, context)?;

                SimpleContentChildId::Restriction(fragment_id)
            }
        };

        Ok(compiler.push_fragment(SimpleContentFragment { content_fragment }))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let child_1 = match &fragment.content_fragment {
            SimpleContentChildId::Extension(fragment_id) => {
                xs::types::SimpleExtensionType::from_complex_fragments(compiler, fragment_id)?
                    .into()
            }
            SimpleContentChildId::Restriction(fragment_id) => {
                xs::types::SimpleRestrictionType::from_complex_fragments(compiler, fragment_id)?
                    .into()
            }
        };

        Ok(xs::SimpleContent::from(
            xs::simple_content_items::SimpleContent {
                annotation: None,
                id: None,
                child_1,
            },
        ))
    }
}

impl ComplexFragmentEquivalent for xs::ComplexContent {
    type FragmentId = FragmentIdx<ComplexContentFragment>;

    /// This method expects all references to already be defined.
    fn to_complex_fragments(
        &self,
        mut compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let xs::ComplexContent::ComplexContent(complex_content) = self else {
            return Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            });
        };

        let content_fragment = match &complex_content.child_1 {
            xs::complex_content_items::Child1::Extension(extension) => {
                let fragment_id = extension.to_complex_fragments(&mut compiler, context)?;

                ComplexContentChildId::Extension(fragment_id)
            }
            xs::complex_content_items::Child1::Restriction(restriction) => {
                let fragment_id = restriction.to_complex_fragments(&mut compiler, context)?;

                ComplexContentChildId::Restriction(fragment_id)
            }
        };

        Ok(compiler.push_fragment(ComplexContentFragment {
            content_fragment,
            mixed: complex_content.mixed,
        }))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let child_1 = match &fragment.content_fragment {
            ComplexContentChildId::Extension(fragment_id) => {
                xs::types::ExtensionType::from_complex_fragments(compiler, fragment_id)?.into()
            }
            ComplexContentChildId::Restriction(fragment_id) => {
                xs::types::ComplexRestrictionType::from_complex_fragments(compiler, fragment_id)?
                    .into()
            }
        };

        Ok(xs::ComplexContent::from(
            xs::complex_content_items::ComplexContent {
                annotation: None,
                id: None,
                mixed: fragment.mixed,
                child_1,
            },
        ))
    }
}

impl ComplexFragmentEquivalent for xs::groups::ComplexTypeModel {
    type FragmentId = ComplexTypeModelId;

    /// This method expects all references to already be defined.
    fn to_complex_fragments(
        &self,
        mut compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        use xs::groups::ComplexTypeModel;

        match self {
            ComplexTypeModel::SimpleContent(simple_content) => simple_content
                .to_complex_fragments(compiler, context)
                .map(ComplexTypeModelId::SimpleContent),
            ComplexTypeModel::ComplexContent(complex_content) => complex_content
                .to_complex_fragments(compiler, context)
                .map(ComplexTypeModelId::ComplexContent),
            ComplexTypeModel::Variant2(variant_2) => {
                let xs::groups::complex_type_model_items::complex_type_model_variants::Variant2 {
                    type_def_particle,
                    attr_decls,
                    assertions,
                    ..
                } = variant_2.deref();

                //TODO: Review open content
                let particle = type_def_particle
                    .as_deref()
                    .map(|a| a.to_complex_fragments(&mut compiler, context))
                    .transpose()?;

                let attr_decls = attr_decls.to_complex_fragments(&mut compiler, context)?;

                let assertions = assertions.to_complex_fragments(&mut compiler, context)?;

                Ok(ComplexTypeModelId::Other {
                    particle,
                    attr_decls,
                    assertions,
                })
            }
        }
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        match fragment_id {
            ComplexTypeModelId::SimpleContent(fragment_idx) => {
                xs::SimpleContent::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::groups::ComplexTypeModel::from)
            }
            ComplexTypeModelId::ComplexContent(fragment_idx) => {
                xs::ComplexContent::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::groups::ComplexTypeModel::from)
            }
            ComplexTypeModelId::Other {
                particle,
                attr_decls,
                assertions,
            } => {
                let type_def_particle = particle
                    .as_ref()
                    .map(|fragment_id| {
                        xs::groups::TypeDefParticle::from_complex_fragments(compiler, fragment_id)
                            .map(Box::new)
                    })
                    .transpose()?;

                let attr_decls =
                    xs::groups::AttrDecls::from_complex_fragments(compiler, attr_decls)?;

                let assertions =
                    xs::groups::Assertions::from_complex_fragments(compiler, &assertions)?;

                Ok(
                    xs::groups::complex_type_model_items::complex_type_model_variants::Variant2 {
                        open_content: None,
                        type_def_particle,
                        attr_decls,
                        assertions,
                    }
                    .into(),
                )
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::types::TopLevelComplexType {
    type FragmentId = FragmentIdx<ComplexTypeRootFragment>;

    fn to_complex_fragments(
        &self,
        mut compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let content = self
            .complex_type_model
            .to_complex_fragments(&mut compiler, context)?;

        let fragment = ComplexTypeRootFragment {
            name: Some(self.name.clone()),
            content,
            mixed: self.mixed,
            abstract_: self.abstract_,
        };

        Ok(compiler.push_fragment(fragment))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let complex_type_model =
            xs::groups::ComplexTypeModel::from_complex_fragments(compiler, &fragment.content)?;

        Ok(Self::builder()
            .name(fragment.name.clone().ok_or(Error::NameMissingInTopLevel)?)
            .maybe_mixed(fragment.mixed)
            .complex_type_model(complex_type_model.into())
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::types::LocalComplexType {
    type FragmentId = FragmentIdx<ComplexTypeRootFragment>;

    fn to_complex_fragments(
        &self,
        mut compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let content = self
            .complex_type_model
            .to_complex_fragments(&mut compiler, context)?;

        let fragment = ComplexTypeRootFragment {
            name: None,
            content,
            mixed: self.mixed,
            abstract_: None,
        };

        Ok(compiler.push_fragment(fragment))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let content =
            xs::groups::ComplexTypeModel::from_complex_fragments(compiler, &fragment.content)?;

        Ok(xs::types::LocalComplexType::builder()
            .maybe_mixed(fragment.mixed)
            .complex_type_model(content.into())
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

/// Identifier for named group type content.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NamedGroupTypeContentId {
    /// All compositor content.
    All(FragmentIdx<AllFragment>),
    /// Sequence compositor content.
    Sequence(FragmentIdx<SequenceFragment>),
    /// Choice compositor content.
    Choice(FragmentIdx<ChoiceFragment>),
}

impl From<FragmentIdx<AllFragment>> for NamedGroupTypeContentId {
    fn from(value: FragmentIdx<AllFragment>) -> Self {
        Self::All(value)
    }
}

impl From<FragmentIdx<SequenceFragment>> for NamedGroupTypeContentId {
    fn from(value: FragmentIdx<SequenceFragment>) -> Self {
        Self::Sequence(value)
    }
}

impl From<FragmentIdx<ChoiceFragment>> for NamedGroupTypeContentId {
    fn from(value: FragmentIdx<ChoiceFragment>) -> Self {
        Self::Choice(value)
    }
}

impl ComplexFragmentEquivalent for xs::types::named_group_items::Child1 {
    type FragmentId = NamedGroupTypeContentId;

    fn to_complex_fragments(
        &self,
        mut compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        use xs::types::named_group_items::Child1;

        match self {
            Child1::All {
                min_occurs,
                max_occurs,
                all_model,
                ..
            } => {
                //TODO: Handle any_attributes
                let fragments = all_model
                    .child_1
                    .iter()
                    .map(|content| content.to_complex_fragments(&mut compiler, context))
                    .collect::<Result<_, _>>()?;

                let fragment = AllFragment {
                    min_occurs: *min_occurs,
                    max_occurs: max_occurs.as_ref().map(|a| AllNNI::from(&**a)),
                    fragments,
                };

                Ok(NamedGroupTypeContentId::All(
                    compiler.push_fragment(fragment),
                ))
            }
            Child1::Choice(choice) => {
                let fragments = choice
                    .nested_particle
                    .iter()
                    .map(|content| content.to_complex_fragments(&mut compiler, context))
                    .collect::<Result<_, _>>()?;

                let fragment = ChoiceFragment {
                    min_occurs: None,
                    max_occurs: None,
                    fragments,
                };

                Ok(NamedGroupTypeContentId::Choice(
                    compiler.push_fragment(fragment),
                ))
            }
            Child1::Sequence(sequence) => {
                let fragments = sequence
                    .nested_particle
                    .iter()
                    .map(|content| content.to_complex_fragments(&mut compiler, context))
                    .collect::<Result<_, _>>()?;

                let fragment = SequenceFragment {
                    id: None,
                    min_occurs: None,
                    max_occurs: None,
                    fragments,
                };

                Ok(NamedGroupTypeContentId::Sequence(
                    compiler.push_fragment(fragment),
                ))
            }
        }
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        match fragment_id {
            NamedGroupTypeContentId::All(all) => {
                let all = compiler
                    .get_fragment(all)
                    .expect("Fragment not found in compiler.");

                Ok(xs::types::named_group_items::Child1::All {
                    id: None,
                    min_occurs: all.min_occurs,
                    max_occurs: all.max_occurs.map(xs::types::AllNNI::from).map(Box::new),
                    all_model: xs::groups::AllModel::builder()
                        .child_1(
                            all.fragments
                                .iter()
                                .map(|fragment| {
                                    xs::groups::all_model_items::Child1::from_complex_fragments(
                                        compiler, fragment,
                                    )
                                })
                                .collect::<Result<_, _>>()?,
                        )
                        .build()
                        .into(),
                    any_attributes: ns::AnyAttributes::default(),
                })
            }
            NamedGroupTypeContentId::Choice(choice) => {
                let choice = compiler
                    .get_fragment(choice)
                    .expect("Fragment not found in compiler.");

                Ok(xs::types::named_group_items::Child1::Choice(
                    xs::types::SimpleExplicitGroup::builder()
                        .nested_particle(
                            choice
                                .fragments
                                .iter()
                                .map(|fragment| {
                                    xs::groups::NestedParticle::from_complex_fragments(
                                        compiler, fragment,
                                    )
                                })
                                .collect::<Result<_, _>>()?,
                        )
                        .any_attributes(ns::AnyAttributes::default())
                        .build()
                        .into(),
                ))
            }
            NamedGroupTypeContentId::Sequence(sequence) => {
                let sequence = compiler
                    .get_fragment(sequence)
                    .expect("Fragment not found in compiler.");

                Ok(xs::types::named_group_items::Child1::Sequence(
                    xs::types::SimpleExplicitGroup::builder()
                        .nested_particle(
                            sequence
                                .fragments
                                .iter()
                                .map(|fragment| {
                                    xs::groups::NestedParticle::from_complex_fragments(
                                        compiler, fragment,
                                    )
                                })
                                .collect::<Result<_, _>>()?,
                        )
                        .any_attributes(ns::AnyAttributes::default())
                        .build()
                        .into(),
                ))
            }
        }
    }
}

impl ComplexFragmentEquivalent for xs::types::NamedGroup {
    type FragmentId = FragmentIdx<TopLevelGroupFragment>;

    fn to_complex_fragments(
        &self,
        mut compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let content = self.child_1.to_complex_fragments(&mut compiler, context)?;

        let fragment = TopLevelGroupFragment {
            name: self.name.clone(),
            content,
        };

        Ok(compiler.push_fragment(fragment))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let content = xs::types::named_group_items::Child1::from_complex_fragments(
            compiler,
            &fragment.content,
        )?;

        Ok(Self::builder()
            .name(fragment.name.clone())
            .child_1(content)
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::AnyAttribute {
    type FragmentId = FragmentIdx<AnyAttributeFragment>;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        _context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let xs::AnyAttribute::AnyAttribute(any_attribute) = self else {
            return Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            });
        };

        Ok(compiler.push_fragment(AnyAttributeFragment {
            id: any_attribute.id.clone(),
            process_contents: any_attribute.process_contents.map(Into::into),
        }))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        Ok(xs::AnyAttribute::from(
            xs::any_attribute_items::AnyAttribute::builder()
                .maybe_id(fragment.id.clone())
                .maybe_process_contents(fragment.process_contents.map(Into::into))
                .build(),
        ))
    }
}

impl ComplexFragmentEquivalent for xs::groups::AttrDecls {
    type FragmentId = FragmentIdx<AttributeDeclarationsFragment>;

    fn to_complex_fragments(
        &self,
        mut compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let declarations = self
            .attribute
            .iter()
            .map(|decl| decl.to_complex_fragments(&mut compiler, context))
            .collect::<Result<_, _>>()?;

        let any_attribute = self
            .any_attribute
            .as_ref()
            .map(|a| a.to_complex_fragments(&mut compiler, context))
            .transpose()?;

        Ok(compiler.push_fragment(AttributeDeclarationsFragment {
            declarations,
            any_attribute,
        }))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let attributes = fragment
            .declarations
            .iter()
            .map(|attr| {
                xs::groups::attr_decls_items::Attribute::from_complex_fragments(compiler, attr)
            })
            .collect::<Result<_, _>>()?;

        let any_attribute = fragment
            .any_attribute
            .as_ref()
            .map(|any_attr| {
                xs::AnyAttribute::from_complex_fragments(compiler, any_attr).map(Box::new)
            })
            .transpose()?;

        Ok(xs::groups::AttrDecls::builder()
            .attribute(attributes)
            .maybe_any_attribute(any_attribute)
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::types::NamedAttributeGroup {
    type FragmentId = FragmentIdx<TopLevelAttributeGroupFragment>;

    fn to_complex_fragments(
        &self,
        mut compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let attr_decls = self
            .attr_decls
            .to_complex_fragments(&mut compiler, context)?;

        Ok(compiler.push_fragment(TopLevelAttributeGroupFragment {
            name: self.name.clone(),
            attr_decls,
        }))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let attr_decls =
            xs::groups::AttrDecls::from_complex_fragments(compiler, &fragment.attr_decls)?;

        Ok(Self::builder()
            .name(fragment.name.clone())
            .attr_decls(attr_decls.into())
            .any_attributes(ns::AnyAttributes::default())
            .build())
    }
}

impl ComplexFragmentEquivalent for xs::groups::Redefinable {
    type FragmentId = RedefinableId;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        match self {
            xs::groups::Redefinable::SimpleType(simple_type) => {
                let xs::SimpleType::SimpleType(simple_type) = simple_type.deref() else {
                    todo!();
                };

                let fragment_id =
                    simple_type.to_simple_fragments(&mut compiler.simple_type_compiler, context)?;

                Ok(RedefinableId::SimpleType(fragment_id))
            }
            xs::groups::Redefinable::ComplexType(complex_type) => {
                let xs::ComplexType::ComplexType(complex_type) = complex_type.deref() else {
                    todo!();
                };

                let fragment_id = complex_type.to_complex_fragments(compiler, context)?;

                Ok(RedefinableId::ComplexType(fragment_id))
            }
            xs::groups::Redefinable::Group(group) => {
                let xs::Group::Group(group) = group.deref() else {
                    todo!();
                };

                let fragment_id = group.to_complex_fragments(compiler, context)?;

                Ok(RedefinableId::Group(fragment_id))
            }
            xs::groups::Redefinable::AttributeGroup(attribute_group) => {
                let xs::AttributeGroup::AttributeGroup(attribute_group) = attribute_group.deref()
                else {
                    todo!();
                };

                let fragment_id = attribute_group.to_complex_fragments(compiler, context)?;

                Ok(RedefinableId::AttributeGroup(fragment_id))
            }
        }
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        match fragment_id {
            RedefinableId::ComplexType(fragment_idx) => {
                xs::types::TopLevelComplexType::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::ComplexType::from)
                    .map(xs::groups::Redefinable::from)
            }
            RedefinableId::SimpleType(fragment_idx) => {
                xs::types::TopLevelSimpleType::from_simple_fragments(
                    &compiler.simple_type_compiler,
                    fragment_idx,
                )
                .map(xs::SimpleType::from)
                .map(xs::groups::Redefinable::from)
                .map_err(Error::Simple)
            }
            RedefinableId::AttributeGroup(fragment_idx) => {
                xs::types::NamedAttributeGroup::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::AttributeGroup::from)
                    .map(xs::groups::Redefinable::from)
            }
            RedefinableId::Group(fragment_idx) => {
                xs::types::NamedGroup::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::Group::from)
                    .map(xs::groups::Redefinable::from)
            }
            RedefinableId::Notation => todo!(),
        }
    }
}

impl ComplexFragmentEquivalent for xs::groups::SchemaTop {
    type FragmentId = SchemaTopId;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        match self {
            xs::groups::SchemaTop::Redefinable(redefinable) => redefinable
                .to_complex_fragments(compiler, context)
                .map(SchemaTopId::Redefinable),
            xs::groups::SchemaTop::Element(element) => {
                let xs::Element::Element(element) = element.deref() else {
                    todo!();
                };

                element
                    .to_complex_fragments(compiler, context)
                    .map(SchemaTopId::Element)
            }
            xs::groups::SchemaTop::Attribute(attribute) => {
                let xs::Attribute::Attribute(attribute) = attribute.deref() else {
                    todo!();
                };

                attribute
                    .to_complex_fragments(compiler, context)
                    .map(SchemaTopId::Attribute)
            }
            xs::groups::SchemaTop::Notation(notation) => {
                //TODO
                Ok(SchemaTopId::Notation)
            }
        }
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        match fragment_id {
            SchemaTopId::Redefinable(fragment_idx) => {
                xs::groups::Redefinable::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::groups::SchemaTop::from)
            }
            SchemaTopId::Element(fragment_idx) => {
                xs::types::TopLevelElement::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::Element::from)
                    .map(xs::groups::SchemaTop::from)
            }
            SchemaTopId::Attribute(fragment_idx) => {
                xs::types::TopLevelAttribute::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::Attribute::from)
                    .map(xs::groups::SchemaTop::from)
            }
            SchemaTopId::Notation => todo!(),
        }
    }
}

impl ComplexFragmentEquivalent for xs::Import {
    type FragmentId = FragmentIdx<ImportFragment>;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let xs::Import::Import(import) = self else {
            return Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            });
        };

        let namespace = import
            .namespace
            .clone()
            .map(XmlNamespace::new)
            .transpose()
            .expect("Invalid namespace");

        let schema_location = import.schema_location.clone();

        let fragment = ImportFragment {
            schema_location,
            namespace,
        };

        let fragment_id = compiler.push_fragment(fragment);

        Ok(fragment_id)
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let namespace = fragment.namespace.as_ref().map(|ns| ns.to_string());

        Ok(xs::Import::from(xs::import_items::Import {
            id: None,
            namespace,
            schema_location: fragment.schema_location.clone(),
            annotation: None,
        }))
    }
}

impl ComplexFragmentEquivalent for xs::Include {
    type FragmentId = FragmentIdx<IncludeFragment>;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let xs::Include::Include(include) = self else {
            return Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            });
        };

        let schema_location = include.schema_location.clone();

        let fragment = IncludeFragment { schema_location };

        Ok(compiler.push_fragment(fragment))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        Ok(xs::Include::from(xs::include_items::Include {
            id: None,
            schema_location: fragment.schema_location.clone(),
            annotation: None,
        }))
    }
}

impl ComplexFragmentEquivalent for xs::Redefine {
    type FragmentId = FragmentIdx<RedefineFragment>;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let xs::Redefine::Redefine(redefine) = self else {
            return Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            });
        };

        let schema_location = redefine.schema_location.clone();

        let redefineable = redefine
            .redefine_content
            .iter()
            .filter_map(|redefinable| match redefinable {
                xs::redefine_items::RedefineContent::Annotation(_) => None,
                xs::redefine_items::RedefineContent::Redefinable(redefinable) => {
                    Some(redefinable.deref())
                }
            })
            .map(|redefinable| redefinable.to_complex_fragments(compiler, context))
            .collect::<Result<_, _>>()?;

        let fragment = RedefineFragment {
            schema_location,
            redefineable,
        };

        Ok(compiler.push_fragment(fragment))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let schema_location = fragment.schema_location.clone();

        let redefine_content = fragment
            .redefineable
            .iter()
            .map(|redefinable_id| {
                xs::groups::Redefinable::from_complex_fragments(compiler, redefinable_id)
            })
            .map(|redefinable| redefinable.map(xs::redefine_items::RedefineContent::from))
            .collect::<Result<_, _>>()?;

        Ok(xs::Redefine::from(xs::redefine_items::Redefine {
            id: None,
            schema_location,
            redefine_content,
        }))
    }
}

impl ComplexFragmentEquivalent for xs::Override {
    type FragmentId = FragmentIdx<OverrideFragment>;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        let xs::Override::Override(override_) = self else {
            return Err(Error::SubstitutionGroupNotSupported {
                fragment_type: type_name::<Self>(),
            });
        };

        let schema_location = override_.schema_location.clone();

        let schema_tops = override_
            .schema_top
            .iter()
            .map(|top| top.to_complex_fragments(compiler, context))
            .collect::<Result<_, _>>()?;

        let fragment = OverrideFragment {
            schema_location,
            schema_tops,
        };

        Ok(compiler.push_fragment(fragment))
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        let fragment = compiler
            .get_fragment(fragment_id)
            .expect("Fragment not found in compiler.");

        let schema_top = fragment
            .schema_tops
            .iter()
            .map(|schema_top_id| {
                xs::groups::SchemaTop::from_complex_fragments(compiler, schema_top_id)
            })
            .collect::<Result<_, _>>()?;

        Ok(xs::Override::from(xs::override_items::Override {
            id: None,
            schema_location: fragment.schema_location.clone(),
            schema_top,
            annotation: None,
        }))
    }
}

impl ComplexFragmentEquivalent for xs::groups::Composition {
    type FragmentId = CompositionId;

    fn to_complex_fragments(
        &self,
        compiler: &mut ComplexTypeFragmentCompiler,
        context: &Context,
    ) -> Result<Self::FragmentId, Error> {
        match self {
            xs::groups::Composition::Include(include) => include
                .to_complex_fragments(compiler, context)
                .map(CompositionId::Include),
            xs::groups::Composition::Import(import) => import
                .to_complex_fragments(compiler, context)
                .map(CompositionId::Import),
            xs::groups::Composition::Redefine(redefine) => redefine
                .to_complex_fragments(compiler, context)
                .map(CompositionId::Redefine),
            xs::groups::Composition::Override(override_) => override_
                .to_complex_fragments(compiler, context)
                .map(CompositionId::Override),
            //TODO
            xs::groups::Composition::Annotation(annotation) => Ok(CompositionId::AnnotationId),
        }
    }

    fn from_complex_fragments(
        compiler: &ComplexTypeFragmentCompiler,
        fragment_id: &Self::FragmentId,
    ) -> Result<Self, Error> {
        match fragment_id {
            CompositionId::Include(fragment_idx) => {
                xs::Include::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::groups::Composition::from)
            }
            CompositionId::Import(fragment_idx) => {
                xs::Import::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::groups::Composition::from)
            }
            CompositionId::Redefine(fragment_idx) => {
                xs::Redefine::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::groups::Composition::from)
            }
            CompositionId::Override(fragment_idx) => {
                xs::Override::from_complex_fragments(compiler, fragment_idx)
                    .map(xs::groups::Composition::from)
            }
            CompositionId::AnnotationId => todo!(),
        }
    }
}
