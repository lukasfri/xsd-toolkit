use core::fmt;
use std::{fmt::Display, marker::PhantomData};

use xmlity::{Deserialize, DeserializeOwned, Serialize};

use crate::utils::{SubStrDeserializer, SubStringSerializer};

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct List<T>(pub Vec<T>);

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl<'de, T: DeserializeOwned> Deserialize<'de> for List<T> {
    fn deserialize<D: xmlity::Deserializer<'de>>(reader: D) -> Result<Self, D::Error> {
        struct ListVisitor<T> {
            _marker: PhantomData<T>,
        }

        impl<'de, T: DeserializeOwned> xmlity::de::Visitor<'de> for ListVisitor<T> {
            type Value = List<T>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a List of items")
            }

            fn visit_text<E, V>(self, value: V) -> Result<Self::Value, E>
            where
                E: xmlity::de::Error,
                V: xmlity::de::XmlText<'de>,
            {
                value
                    .as_str()
                    .split([' ', '|', ',', ';'])
                    .filter(|v| !v.is_empty())
                    .map(|s| T::deserialize(SubStrDeserializer::new(s, &value.context())))
                    .collect::<Result<Vec<_>, _>>()
                    .map(List)
            }
        }

        reader.deserialize_any(ListVisitor {
            _marker: PhantomData,
        })
    }
}

impl<T: Serialize> Serialize for List<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: xmlity::Serializer,
    {
        let parts = self
            .0
            .iter()
            .map(|item| item.serialize(SubStringSerializer::new()))
            .collect::<Result<Vec<_>, _>>()?;

        let list_string = parts
            .into_iter()
            .flatten()
            .fold(String::new(), |mut acc, item| {
                if !acc.is_empty() {
                    acc.push(' ');
                }
                acc.push_str(&item);
                acc
            });

        serializer.serialize_text(&list_string)
    }
}
