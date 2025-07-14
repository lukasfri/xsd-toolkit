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

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        List(iter.into_iter().collect())
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Extend<T> for List<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List(Vec::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}
