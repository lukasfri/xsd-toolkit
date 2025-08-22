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
    MaxTransformationLoopsReached,
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
        let mut total_change = TransformChange::Changed;

        for i in 0..self.max_iterations {
            if i >= self.max_iterations - 1 {
                return Err(Error::MaxTransformationLoopsReached);
            }

            let mut i_change = TransformChange::Unchanged;

            i_change |= context
                .xmlns_context
                .context_transform(ExpandIncludeFragments::new())
                .map_err(Error::ExpandIncludeFragmentsError)?;

            i_change |= context
                .xmlns_context
                .context_transform(ExpandRedefineFragments::new())
                .map_err(Error::ExpandRedefineFragmentsError)?;

            i_change |= context
                .xmlns_context
                .context_transform(ExpandSimpleRestriction::new(&self.allowed_simple_bases))
                .map_err(Error::ExpandSimpleRestrictionError)?;

            i_change |= context
                .xmlns_context
                .local_transform_all(&ExpandShortFormComplexTypes::new())
                .map_err(Error::ExpandShortFormComplexTypesError)?;

            i_change |= context
                .xmlns_context
                .local_transform_all(&SingleChoiceToSequence::new())
                .map_err(Error::SingleChoiceToSequenceError)?;

            i_change |= context
                .xmlns_context
                .local_transform_all(&FlattenNestedSequences::new())
                .map_err(Error::FlattenNestedSequencesError)?;

            i_change |= context
                .xmlns_context
                .local_transform_all(&FlattenNestedChoices::new())
                .map_err(Error::FlattenNestedChoicesError)?;

            i_change |= context
                .xmlns_context
                .context_transform(ExpandAttributeDeclarations::new())
                .map_err(Error::ExpandAttributeDeclarationsError)?;

            i_change |= context
                .xmlns_context
                .context_transform(ExpandExtensionFragments::new())
                .map_err(Error::ExpandExtensionFragmentsError)?;

            i_change |= context
                .xmlns_context
                .context_transform(ExpandRestrictionFragments::new())
                .map_err(Error::ExpandRestrictionFragmentsError)?;

            i_change |= context
                .xmlns_context
                .context_transform(&RemoveProhibitedAttributes::new())
                .map_err(Error::RemoveProhibitedAttributesError)?;

            total_change |= i_change;

            if i_change == TransformChange::Unchanged {
                break;
            }
        }

        Ok(total_change)
    }
}
