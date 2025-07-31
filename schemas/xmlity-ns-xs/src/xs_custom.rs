use core::fmt;

use xmlity::{
    de::DeserializeContext, Deserialize, ExpandedName, LocalName, Prefix, Serialize, XmlNamespace,
};

pub mod types {

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct QName(pub ExpandedName<'static>);

    impl From<QName> for ExpandedName<'static> {
        fn from(qname: QName) -> Self {
            qname.0
        }
    }

    impl From<ExpandedName<'static>> for QName {
        fn from(expanded_name: ExpandedName<'static>) -> Self {
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
                            let local_name = LocalName::new(last_part).unwrap().into_owned();

                            let prefix = Prefix::new(first_part).unwrap();
                            let namespace = ctx.resolve_prefix(prefix).unwrap().into_owned();

                            ExpandedName::new(local_name, Some(namespace))
                        }
                        None => {
                            let local_name = LocalName::new(first_part).unwrap().into_owned();

                            let default_namespace =
                                ctx.default_namespace().map(XmlNamespace::into_owned);

                            ExpandedName::new(local_name, default_namespace)
                        }
                    };

                    Ok(QName(expanded_name))
                }
            }

            reader.deserialize_any(QNameVisitor)
        }
    }

    impl Serialize for QName {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: xmlity::Serializer,
        {
            todo!()
        }
    }
}
