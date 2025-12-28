use core::fmt;

use xmlity::{
    de::DeserializeContext, Deserialize, ExpandedName, ExpandedNameBuf, LocalName, Prefix,
    Serialize,
};

pub mod types {

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct QName(pub ExpandedNameBuf);

    impl From<QName> for ExpandedNameBuf {
        fn from(qname: QName) -> Self {
            qname.0
        }
    }

    impl From<ExpandedNameBuf> for QName {
        fn from(expanded_name: ExpandedNameBuf) -> Self {
            Self(expanded_name)
        }
    }

    impl<'de> Deserialize<'de> for QName {
        fn deserialize<D: xmlity::Deserializer<'de>>(reader: D) -> Result<Self, D::Error> {
            struct QNameVisitor;

            impl<'de> xmlity::de::Visitor<'de> for QNameVisitor {
                type Value = QName;
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a QName")
                }

                fn visit_text<E, V>(self, value: V) -> Result<Self::Value, E>
                where
                    E: xmlity::de::Error,
                    V: xmlity::de::XmlText<'de>,
                {
                    let ctx = value.context();

                    let mut qname_parts = value.as_str().split(":");
                    let first_part = qname_parts.next().expect("Always has at least one part.");
                    let last_part = qname_parts.next();

                    let expanded_name = match last_part {
                        Some(last_part) => {
                            let local_name = LocalName::new(last_part).unwrap();

                            let prefix = Prefix::new(first_part).unwrap();
                            let namespace = ctx.resolve_prefix(prefix).unwrap();

                            ExpandedName::new(local_name, Some(namespace)).into_owned()
                        }
                        None => {
                            let local_name = LocalName::new(first_part).unwrap();

                            let default_namespace = ctx.default_namespace();

                            ExpandedName::new(local_name, default_namespace).into_owned()
                        }
                    };

                    Ok(QName(expanded_name))
                }
            }

            reader.deserialize_any(QNameVisitor)
        }
    }

    impl Serialize for QName {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: xmlity::Serializer,
        {
            //TODO
            format!(
                "{}:{}",
                self.0
                    .namespace()
                    .as_ref()
                    .map(|a| a.to_string())
                    .unwrap_or_default(),
                self.0.local_name()
            )
            .serialize(serializer)
        }
    }
}
