use core::fmt;
use std::marker::PhantomData;

use xmlity::{
    de, value::XmlElement, Deserialize, Deserializer, ExpandedName, Serialize, Serializer,
};

pub struct SubstitutionGroupContext<T> {
    allowed_names: Vec<ExpandedName<'static>>,
    _marker: PhantomData<T>,
}
impl<T> SubstitutionGroupContext<T> {
    pub fn new(allowed_names: Vec<ExpandedName<'static>>) -> Self {
        SubstitutionGroupContext {
            allowed_names,
            _marker: PhantomData,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct SubstitutionGroup<T> {
    value: XmlElement,
    _marker: PhantomData<T>,
}

impl<T> SubstitutionGroup<T> {
    pub fn new(value: XmlElement) -> Self {
        SubstitutionGroup {
            value,
            _marker: PhantomData,
        }
    }

    pub fn into_inner(self) -> XmlElement {
        self.value
    }
}

impl<'de, T: 'static> Deserialize<'de> for SubstitutionGroup<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ItemSubstitutionGroupVisitor<T> {
            _marker: PhantomData<T>,
        }

        impl<'de, T: 'static> de::Visitor<'de> for ItemSubstitutionGroupVisitor<T> {
            type Value = SubstitutionGroup<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an item substitution group element")
            }

            fn visit_element<A>(self, element: A) -> Result<Self::Value, A::Error>
            where
                A: de::ElementAccess<'de>,
            {
                {
                    let ctx = element.context();
                    let item_ctx = de::DeserializeContext::external_data::<
                        SubstitutionGroupContext<T>,
                    >(&ctx)
                    .ok_or_else(|| de::Error::custom("Missing ItemSubstitutionGroupContext"))?;

                    let name = element.name();

                    if !item_ctx.allowed_names.contains(&name) {
                        return Err(de::Error::custom(format!(
                            "Unexpected element name: {:?}. Expected one of: {:?}",
                            name, item_ctx.allowed_names
                        )));
                    }
                }

                xmlity::value::deserialize::XmlElementVisitor::new()
                    .visit_element(element)
                    .map(|value| SubstitutionGroup {
                        value,
                        _marker: PhantomData,
                    })
            }
        }

        deserializer.deserialize_any(ItemSubstitutionGroupVisitor {
            _marker: PhantomData,
        })
    }
}

impl<T: 'static> Serialize for SubstitutionGroup<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.value.serialize(serializer)
    }
}
