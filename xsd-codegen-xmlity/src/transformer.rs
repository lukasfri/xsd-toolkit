use std::collections::HashSet;

use xmlity::ExpandedName;
use xsd_fragments::transformers::{
    context::{
        complex::{
            ExpandAttributeDeclarations, ExpandAttributeDeclarationsError,
            ExpandExtensionFragments, ExpandExtensionFragmentsError, ExpandIncludeFragments,
            ExpandIncludeFragmentsError, ExpandRedefineFragments, ExpandRedefineFragmentsError,
            ExpandRestrictionFragments, ExpandRestrictionFragmentsError,
            RemoveProhibitedAttributes, RemoveProhibitedAttributesError,
        },
        simple::{ExpandSimpleRestriction, ExpandSimpleRestrictionError},
    },
    local::complex::{
        ExpandShortFormComplexTypes, ExpandShortFormComplexTypesError, FlattenNestedChoices,
        FlattenNestedChoicesError, FlattenNestedSequences, FlattenNestedSequencesError,
        SingleChoiceToSequence, SingleChoiceToSequenceError,
    },
    TransformChange, XmlnsContextTransformer,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error when expanding include fragments: {0}")]
    ExpandIncludeFragmentsError(ExpandIncludeFragmentsError),
    #[error("Error when expanding redefine fragments: {0}")]
    ExpandRedefineFragmentsError(ExpandRedefineFragmentsError),
    #[error("Error when flattening nested sequences: {0}")]
    FlattenNestedSequencesError(FlattenNestedSequencesError),
    #[error("Error when flattening nested choices: {0}")]
    FlattenNestedChoicesError(FlattenNestedChoicesError),
    #[error("Error when expanding simple restriction: {0}")]
    ExpandSimpleRestrictionError(ExpandSimpleRestrictionError),
    #[error("Error when expanding short form complex types: {0}")]
    ExpandShortFormComplexTypesError(ExpandShortFormComplexTypesError),
    #[error("Error when expanding restriction fragments: {0}")]
    ExpandRestrictionFragmentsError(ExpandRestrictionFragmentsError),
    #[error("Error when expanding extension fragments: {0}")]
    ExpandExtensionFragmentsError(ExpandExtensionFragmentsError),
    #[error("Error when expanding attribute declarations: {0}")]
    ExpandAttributeDeclarationsError(ExpandAttributeDeclarationsError),
    #[error("Error when removing prohibited attributes: {0}")]
    RemoveProhibitedAttributesError(RemoveProhibitedAttributesError),
    #[error("Error when transforming single choices to sequences: {0}")]
    SingleChoiceToSequenceError(SingleChoiceToSequenceError),
    #[error("Maximum transformation loops reached")]
    MaxTransformationLoopsReached {
        expand_include_fragments: TransformChange,
        expand_redefine_fragments: TransformChange,
        flatten_nested_sequences: TransformChange,
        flatten_nested_choices: TransformChange,
        expand_simple_restriction: TransformChange,
        expand_short_form_complex_types: TransformChange,
        expand_restriction_fragments: TransformChange,
        expand_extension_fragments: TransformChange,
        expand_attribute_declarations: TransformChange,
        remove_prohibited_attributes: TransformChange,
        single_choice_to_sequence: TransformChange,
    },
}

#[non_exhaustive]
#[derive(Debug)]
/// This transformer is used to transform the XSD into a form that is required for the codegen to work.
pub struct CodegenTransformer {
    max_iterations: usize,
    allowed_simple_bases: HashSet<ExpandedName<'static>>,
}

impl CodegenTransformer {
    #[allow(clippy::new_without_default)]
    pub fn new(allowed_simple_bases: HashSet<ExpandedName<'static>>) -> Self {
        Self {
            max_iterations: 100,
            allowed_simple_bases,
        }
    }
}

impl XmlnsContextTransformer for CodegenTransformer {
    type Error = Error;
    fn transform(
        self,
        context: xsd_fragments::transformers::XmlnsContextTransformerContext<'_>,
    ) -> std::result::Result<TransformChange, Self::Error> {
        let mut has_changed = TransformChange::Unchanged;

        for i in 0..self.max_iterations {
            let expand_include_fragments = context
                .xmlns_context
                .context_transform(ExpandIncludeFragments::new())
                .map_err(Error::ExpandIncludeFragmentsError)?;

            let expand_redefine_fragments = context
                .xmlns_context
                .context_transform(ExpandRedefineFragments::new())
                .map_err(Error::ExpandRedefineFragmentsError)?;

            let expand_simple_restriction = context
                .xmlns_context
                .context_transform(ExpandSimpleRestriction::new(&self.allowed_simple_bases))
                .map_err(Error::ExpandSimpleRestrictionError)?;

            let expand_short_form_complex_types = context
                .xmlns_context
                .local_transform_all(&ExpandShortFormComplexTypes::new())
                .map_err(Error::ExpandShortFormComplexTypesError)?;

            let single_choice_to_sequence = context
                .xmlns_context
                .local_transform_all(&SingleChoiceToSequence::new())
                .map_err(Error::SingleChoiceToSequenceError)?;

            let flatten_nested_sequences = context
                .xmlns_context
                .local_transform_all(&FlattenNestedSequences::new())
                .map_err(Error::FlattenNestedSequencesError)?;

            let flatten_nested_choices = context
                .xmlns_context
                .local_transform_all(&FlattenNestedChoices::new())
                .map_err(Error::FlattenNestedChoicesError)?;

            let expand_attribute_declarations = context
                .xmlns_context
                .context_transform(ExpandAttributeDeclarations::new())
                .map_err(Error::ExpandAttributeDeclarationsError)?;

            let expand_extension_fragments = context
                .xmlns_context
                .context_transform(ExpandExtensionFragments::new())
                .map_err(Error::ExpandExtensionFragmentsError)?;

            let expand_restriction_fragments = context
                .xmlns_context
                .context_transform(ExpandRestrictionFragments::new())
                .map_err(Error::ExpandRestrictionFragmentsError)?;

            let remove_prohibited_attributes = context
                .xmlns_context
                .context_transform(&RemoveProhibitedAttributes::new())
                .map_err(Error::RemoveProhibitedAttributesError)?;

            let total_change = expand_include_fragments
                | expand_redefine_fragments
                | flatten_nested_sequences
                | flatten_nested_choices
                | expand_simple_restriction
                | expand_short_form_complex_types
                | expand_restriction_fragments
                | expand_extension_fragments
                | expand_attribute_declarations
                | remove_prohibited_attributes
                | single_choice_to_sequence;

            has_changed |= total_change;

            if total_change == TransformChange::Unchanged {
                break;
            } else if i >= self.max_iterations - 1 {
                return Err(Error::MaxTransformationLoopsReached {
                    expand_include_fragments,
                    expand_redefine_fragments,
                    flatten_nested_sequences,
                    flatten_nested_choices,
                    expand_simple_restriction,
                    expand_short_form_complex_types,
                    expand_restriction_fragments,
                    expand_extension_fragments,
                    expand_attribute_declarations,
                    remove_prohibited_attributes,
                    single_choice_to_sequence,
                });
            }
        }

        Ok(has_changed)
    }
}
