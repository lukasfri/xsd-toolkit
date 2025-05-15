pub mod builtin;
pub mod schema;

pub struct XmlSchema {
    pub underlying_schema: schema::Schema,
}

impl XmlSchema {
    pub fn new(underlying_schema: schema::Schema) -> Self {
        Self { underlying_schema }
    }

    pub fn top_level_elements(&self) -> impl Iterator<Item = &schema::TopLevelElement> {
        self.underlying_schema.schema_top.iter().filter_map(|top| {
            if let schema::SchemaTop::Element(element) = top {
                Some(element)
            } else {
                None
            }
        })
    }

    pub fn top_level_attributes(&self) -> impl Iterator<Item = &schema::TopLevelAttribute> {
        self.underlying_schema.schema_top.iter().filter_map(|top| {
            if let schema::SchemaTop::Attribute(attribute) = top {
                Some(attribute)
            } else {
                None
            }
        })
    }

    pub fn redefineable(&self) -> impl Iterator<Item = &schema::Redefineable> {
        self.underlying_schema.schema_top.iter().filter_map(|top| {
            if let schema::SchemaTop::Redefineable(redefinable) = top {
                Some(redefinable)
            } else {
                None
            }
        })
    }

    pub fn simple_types(&self) -> impl Iterator<Item = &schema::TopLevelSimpleType> {
        self.redefineable().filter_map(|re| {
            if let schema::Redefineable::SimpleType(simple_type) = re {
                Some(simple_type)
            } else {
                None
            }
        })
    }

    pub fn complex_types(&self) -> impl Iterator<Item = &schema::TopLevelComplexType> {
        self.redefineable().filter_map(|re| {
            if let schema::Redefineable::ComplexType(complex_type) = re {
                Some(complex_type)
            } else {
                None
            }
        })
    }
}

pub struct XmlSchemaContext {}
