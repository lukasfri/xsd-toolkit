use std::collections::HashSet;

use xmlity::ExpandedName;
use xsd_fragments_transformer::{
    complex::{
        ExpandAttributeDeclarations, ExpandAttributeDeclarationsError, ExpandExtensionFragments,
        ExpandExtensionFragmentsError, ExpandRestrictionFragments, ExpandRestrictionFragmentsError,
        ExpandShortFormComplexTypes, ExpandShortFormComplexTypesError, FlattenNestedChoices,
        FlattenNestedChoicesError, FlattenNestedSequences, FlattenNestedSequencesError,
        RemoveProhibitedAttributes, RemoveProhibitedAttributesError, SingleChoiceToSequence,
        SingleChoiceToSequenceError,
    },
    simple::{ExpandSimpleRestriction, ExpandSimpleRestrictionError},
    TransformChange, XmlnsContextExt, XmlnsContextTransformer,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
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
pub struct XmlityCodegenTransformer {
    allowed_simple_bases: HashSet<ExpandedName<'static>>,
}

impl XmlityCodegenTransformer {
    #[allow(clippy::new_without_default)]
    pub fn new(allowed_simple_bases: HashSet<ExpandedName<'static>>) -> Self {
        Self {
            allowed_simple_bases,
        }
    }
}

impl XmlnsContextTransformer for XmlityCodegenTransformer {
    type Error = Error;
    fn transform(
        self,
        context: xsd_fragments_transformer::XmlnsContextTransformerContext<'_>,
    ) -> std::result::Result<TransformChange, Self::Error> {
        for i in 0..100 {
            let mut total_change = TransformChange::Unchanged;

            total_change |= context
                .xmlns_context
                .context_transform(ExpandSimpleRestriction::new(&self.allowed_simple_bases))
                .map_err(Error::ExpandSimpleRestrictionError)?;

            total_change |= context
                .xmlns_context
                .local_transform_all(&ExpandShortFormComplexTypes::new())
                .map_err(Error::ExpandShortFormComplexTypesError)?;

            total_change |= context
                .xmlns_context
                .local_transform_all(&SingleChoiceToSequence::new())
                .map_err(Error::SingleChoiceToSequenceError)?;

            total_change |= context
                .xmlns_context
                .local_transform_all(&FlattenNestedSequences::new())
                .map_err(Error::FlattenNestedSequencesError)?;

            total_change |= context
                .xmlns_context
                .local_transform_all(&FlattenNestedChoices::new())
                .map_err(Error::FlattenNestedChoicesError)?;

            total_change |= context
                .xmlns_context
                .context_transform(ExpandAttributeDeclarations::new())
                .map_err(Error::ExpandAttributeDeclarationsError)?;

            total_change |= context
                .xmlns_context
                .context_transform(ExpandExtensionFragments::new())
                .map_err(Error::ExpandExtensionFragmentsError)?;

            total_change |= context
                .xmlns_context
                .context_transform(ExpandRestrictionFragments::new())
                .map_err(Error::ExpandRestrictionFragmentsError)?;

            total_change |= context
                .xmlns_context
                .context_transform(&RemoveProhibitedAttributes::new())
                .map_err(Error::RemoveProhibitedAttributesError)?;

            if total_change == TransformChange::Unchanged {
                return Ok(TransformChange::from(i > 0));
            }
        }

        Err(Error::MaxTransformationLoopsReached)
    }
}
