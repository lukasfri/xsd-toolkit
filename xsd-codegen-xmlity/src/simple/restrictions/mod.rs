use crate::simple::SimpleContext;
use crate::{misc::TypeReference, simple::SimpleToTypeTemplate};
use core::num::NonZeroUsize;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::ops::Deref;
use syn::parse_quote;
use xsd::xsn;
use xsd_type_compiler::fragments::simple as sm;

mod numeric;
use numeric::NumericRestrictionBuilder;
mod string;
pub use string::StringRestrictionBuilder;

trait RestrictionBuilder<C: super::SimpleContext, S: crate::Scope> {
    fn build(
        &self,
        context: &C,
        scope: &mut S,
        facets: &[&sm::FacetFragment],
    ) -> crate::Result<crate::ToTypeTemplateData<TypeReference<'static>>>;
}

impl SimpleToTypeTemplate for sm::RestrictionFragment {
    type TypeTemplate = TypeReference<'static>;

    fn to_type_template<C: super::SimpleContext, S: crate::Scope>(
        &self,
        context: &C,
        scope: &mut S,
    ) -> crate::Result<crate::ToTypeTemplateData<Self::TypeTemplate>> {
        let facets = self
            .facets
            .iter()
            .map(|facet| context.get_fragment(facet, scope))
            .collect::<crate::Result<Vec<_>>>()?;

        let mut restriction_builders: HashMap<_, Box<dyn RestrictionBuilder<C, S>>> =
            HashMap::new();

        macro_rules! add_numeric_restriction {
            ($name: expr, $ty: ty) => {
                restriction_builders.insert(
                    $name.deref(),
                    Box::new(NumericRestrictionBuilder::<$ty>::new())
                        as Box<dyn RestrictionBuilder<_, _>>,
                );
            };
        }
        macro_rules! add_string_restriction {
            ($name: expr, $ty: ty) => {
                restriction_builders.insert(
                    $name.deref(),
                    Box::new(StringRestrictionBuilder::<$ty>::new())
                        as Box<dyn RestrictionBuilder<_, _>>,
                );
            };
        }
        add_numeric_restriction!(xsn::DECIMAL, Decimal);
        add_numeric_restriction!(xsn::FLOAT, f32);
        add_numeric_restriction!(xsn::DOUBLE, f64);
        add_numeric_restriction!(xsn::INTEGER, usize);
        add_numeric_restriction!(xsn::NON_POSITIVE_INTEGER, isize);
        add_numeric_restriction!(xsn::NEGATIVE_INTEGER, isize);
        add_numeric_restriction!(xsn::LONG, i64);
        add_numeric_restriction!(xsn::INT, i32);
        add_numeric_restriction!(xsn::SHORT, i16);
        add_numeric_restriction!(xsn::BYTE, i8);
        add_numeric_restriction!(xsn::NON_NEGATIVE_INTEGER, usize);
        add_numeric_restriction!(xsn::UNSIGNED_LONG, u64);
        add_numeric_restriction!(xsn::UNSIGNED_INT, u32);
        add_numeric_restriction!(xsn::UNSIGNED_SHORT, u16);
        add_numeric_restriction!(xsn::UNSIGNED_BYTE, u8);
        add_numeric_restriction!(xsn::POSITIVE_INTEGER, NonZeroUsize);
        add_string_restriction!(xsn::STRING, String);
        add_string_restriction!(xsn::NORMALIZED_STRING, String);
        add_string_restriction!(xsn::TOKEN, String);
        add_string_restriction!(xsn::LANGUAGE, String);
        add_string_restriction!(xsn::NAME, String);
        add_string_restriction!(xsn::NCNAME, String);
        add_string_restriction!(xsn::ID, String);
        add_string_restriction!(xsn::IDREF, String);
        add_string_restriction!(xsn::IDREFS, String);
        add_string_restriction!(xsn::ENTITY, String);
        add_string_restriction!(xsn::ENTITIES, String);
        add_string_restriction!(xsn::NMTOKEN, String);
        add_string_restriction!(xsn::NMTOKENS, String);

        let Some(builder) = restriction_builders.get(&self.base) else {
            // TODO: This should be an error in the future, but for now we just return a String type as a fallback for unsupported types.
            return Ok(crate::ToTypeTemplateData {
                ident: None,
                template: TypeReference::new_static(parse_quote!(::std::string::String)),
            });
        };

        // TODO types:
        // "string"
        // "boolean"
        // "duration"
        // "dateTime"
        // "time"
        // "date"
        // "gYearMonth"
        // "gYear"
        // "gMonthDay"
        // "gDay"
        // "gMonth"
        // "hexBinary"
        // "base64Binary"
        // "anyURI"
        // "QName"
        // "NOTATION"
        // "normalizedString"
        // "token"
        // "language"
        // "NMTOKEN"
        // "NMTOKENS"
        // "NAME"
        // "NCNAME"
        // "ID"
        // "IDREF"
        // "IDREFS"
        // "ENTITY"
        // "ENTITIES"
        // "yearMonthDuration"
        // "dayTimeDuration"
        // "dateTimeStamp"

        builder.build(context, scope, &facets)
    }
}
