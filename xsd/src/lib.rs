use std::ops::Deref;

pub use xmlity_ns_xs::xs;
pub mod xsn;

pub struct XmlSchema {
    pub underlying_schema: xs::Schema,
}

impl XmlSchema {
    pub fn new(underlying_schema: xs::Schema) -> Self {
        Self { underlying_schema }
    }

    pub fn namespace(&self) -> &xmlity::XmlNamespace<'static> {
        &self.underlying_schema.target_namespace.as_ref().unwrap().0
    }

    pub fn schema_tops(&self) -> impl Iterator<Item = &xs::groups::SchemaTop> {
        self.underlying_schema.child_2.iter().map(|a| &a.schema_top)
    }

    pub fn top_level_elements(&self) -> impl Iterator<Item = &xs::Element> {
        self.schema_tops().filter_map(|top| {
            if let xs::groups::SchemaTop::Element(element) = top {
                Some(element.deref())
            } else {
                None
            }
        })
    }

    pub fn top_level_attributes(&self) -> impl Iterator<Item = &xs::Attribute> {
        self.schema_tops().filter_map(|top| {
            if let xs::groups::SchemaTop::Attribute(attribute) = top {
                Some(attribute.deref())
            } else {
                None
            }
        })
    }

    pub fn redefinable(&self) -> impl Iterator<Item = &xs::groups::Redefinable> {
        self.schema_tops().filter_map(|top| {
            if let xs::groups::SchemaTop::Redefinable(redefinable) = top {
                Some(redefinable.deref())
            } else {
                None
            }
        })
    }

    pub fn top_level_simple_types(&self) -> impl Iterator<Item = &xs::SimpleType> {
        self.redefinable().filter_map(|re| {
            if let xs::groups::Redefinable::SimpleType(simple_type) = re {
                Some(simple_type.deref())
            } else {
                None
            }
        })
    }

    pub fn top_level_complex_types(&self) -> impl Iterator<Item = &xs::ComplexType> {
        self.redefinable().filter_map(|re| {
            if let xs::groups::Redefinable::ComplexType(complex_type) = re {
                Some(complex_type.deref())
            } else {
                None
            }
        })
    }
}

pub struct XmlSchemaContext {}
