pub mod types {
    pub mod all_nni_items {
        pub mod all_nni_variants {
            #[derive(
                ::core::fmt::Debug,
                ::core::clone::Clone,
                ::core::marker::Copy,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq
            )]
            #[xvalue(with = variant_0_with)]
            pub enum Variant0 {
                Unbounded,
            }
            pub mod variant_0_with {
                pub fn deserialize<'de, D>(
                    deserializer: D,
                ) -> ::core::result::Result<super::Variant0, D::Error>
                where
                    D: ::xmlity::Deserializer<'de>,
                {
                    let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                        deserializer,
                    )?;
                    let value: ::std::string::String = text
                        .parse()
                        .map_err(::xmlity::de::Error::custom)?;
                    super::Variant0::try_from(value).map_err(::xmlity::de::Error::custom)
                }
                pub fn serialize<S>(
                    value: &super::Variant0,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::xmlity::Serializer,
                {
                    let value: ::std::string::String = ::core::clone::Clone::clone(value)
                        .into();
                    ::xmlity::Serialize::serialize(
                        ::std::string::String::as_str(
                            &::std::string::ToString::to_string(&value),
                        ),
                        serializer,
                    )
                }
            }
            #[derive(::core::fmt::Debug)]
            pub enum Variant0ParseError {
                NonExistent { value: ::std::string::String },
            }
            impl ::core::fmt::Display for Variant0ParseError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::result::Result<(), ::core::fmt::Error> {
                    match self {
                        Variant0ParseError::NonExistent { value } => {
                            write!(
                                f, "Value '{:?}' does not exist in the enumeration", value
                            )
                        }
                    }
                }
            }
            impl ::core::convert::TryFrom<::std::string::String> for Variant0 {
                type Error = Variant0ParseError;
                fn try_from(
                    value: ::std::string::String,
                ) -> ::core::result::Result<Self, Self::Error> {
                    match ::std::string::String::as_str(&value) {
                        "unbounded" => Ok(Variant0::Unbounded),
                        _ => {
                            Err(Variant0ParseError::NonExistent {
                                value,
                            })
                        }
                    }
                }
            }
            impl ::core::convert::From<Variant0> for ::std::string::String {
                fn from(value: Variant0) -> Self {
                    match value {
                        Variant0::Unbounded => ::std::string::String::from("unbounded"),
                    }
                }
            }
        }
        impl ::core::convert::From<usize> for AllNNI {
            fn from(value: usize) -> Self {
                AllNNI::NonNegativeInteger(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<all_nni_variants::Variant0> for AllNNI {
            fn from(value: all_nni_variants::Variant0) -> Self {
                AllNNI::Variant0(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum AllNNI {
            NonNegativeInteger(::std::boxed::Box<usize>),
            Variant0(::std::boxed::Box<all_nni_variants::Variant0>),
        }
    }
    pub type AllNNI = all_nni_items::AllNNI;
    pub mod basic_namespace_list_items {
        pub mod basic_namespace_list_variants {
            #[derive(
                ::core::fmt::Debug,
                ::core::clone::Clone,
                ::core::marker::Copy,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq
            )]
            #[xvalue(with = variant_0_with)]
            pub enum Variant0 {
                TargetNamespace,
                Local,
            }
            pub mod variant_0_with {
                pub fn deserialize<'de, D>(
                    deserializer: D,
                ) -> ::core::result::Result<super::Variant0, D::Error>
                where
                    D: ::xmlity::Deserializer<'de>,
                {
                    let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                        deserializer,
                    )?;
                    let value: ::std::string::String = text
                        .parse()
                        .map_err(::xmlity::de::Error::custom)?;
                    super::Variant0::try_from(value).map_err(::xmlity::de::Error::custom)
                }
                pub fn serialize<S>(
                    value: &super::Variant0,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::xmlity::Serializer,
                {
                    let value: ::std::string::String = ::core::clone::Clone::clone(value)
                        .into();
                    ::xmlity::Serialize::serialize(
                        ::std::string::String::as_str(
                            &::std::string::ToString::to_string(&value),
                        ),
                        serializer,
                    )
                }
            }
            #[derive(::core::fmt::Debug)]
            pub enum Variant0ParseError {
                NonExistent { value: ::std::string::String },
            }
            impl ::core::fmt::Display for Variant0ParseError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::result::Result<(), ::core::fmt::Error> {
                    match self {
                        Variant0ParseError::NonExistent { value } => {
                            write!(
                                f, "Value '{:?}' does not exist in the enumeration", value
                            )
                        }
                    }
                }
            }
            impl ::core::convert::TryFrom<::std::string::String> for Variant0 {
                type Error = Variant0ParseError;
                fn try_from(
                    value: ::std::string::String,
                ) -> ::core::result::Result<Self, Self::Error> {
                    match ::std::string::String::as_str(&value) {
                        "##targetNamespace" => Ok(Variant0::TargetNamespace),
                        "##local" => Ok(Variant0::Local),
                        _ => {
                            Err(Variant0ParseError::NonExistent {
                                value,
                            })
                        }
                    }
                }
            }
            impl ::core::convert::From<Variant0> for ::std::string::String {
                fn from(value: Variant0) -> Self {
                    match value {
                        Variant0::TargetNamespace => {
                            ::std::string::String::from("##targetNamespace")
                        }
                        Variant0::Local => ::std::string::String::from("##local"),
                    }
                }
            }
        }
        impl ::core::convert::From<crate::types::TargetNamespace>
        for BasicNamespaceList {
            fn from(value: crate::types::TargetNamespace) -> Self {
                BasicNamespaceList::AnyURI(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<basic_namespace_list_variants::Variant0>
        for BasicNamespaceList {
            fn from(value: basic_namespace_list_variants::Variant0) -> Self {
                BasicNamespaceList::Variant0(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum BasicNamespaceList {
            AnyURI(::std::boxed::Box<crate::types::TargetNamespace>),
            Variant0(::std::boxed::Box<basic_namespace_list_variants::Variant0>),
        }
    }
    pub type BasicNamespaceList = ::xmlity_ns::List<
        basic_namespace_list_items::BasicNamespaceList,
    >;
    pub mod block_set_items {
        pub mod variant_variants {
            #[derive(
                ::core::fmt::Debug,
                ::core::clone::Clone,
                ::core::marker::Copy,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq
            )]
            #[xvalue(with = variant_0_with)]
            pub enum Variant0 {
                All,
            }
            pub mod variant_0_with {
                pub fn deserialize<'de, D>(
                    deserializer: D,
                ) -> ::core::result::Result<super::Variant0, D::Error>
                where
                    D: ::xmlity::Deserializer<'de>,
                {
                    let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                        deserializer,
                    )?;
                    let value: ::std::string::String = text
                        .parse()
                        .map_err(::xmlity::de::Error::custom)?;
                    super::Variant0::try_from(value).map_err(::xmlity::de::Error::custom)
                }
                pub fn serialize<S>(
                    value: &super::Variant0,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::xmlity::Serializer,
                {
                    let value: ::std::string::String = ::core::clone::Clone::clone(value)
                        .into();
                    ::xmlity::Serialize::serialize(
                        ::std::string::String::as_str(
                            &::std::string::ToString::to_string(&value),
                        ),
                        serializer,
                    )
                }
            }
            #[derive(::core::fmt::Debug)]
            pub enum Variant0ParseError {
                NonExistent { value: ::std::string::String },
            }
            impl ::core::fmt::Display for Variant0ParseError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::result::Result<(), ::core::fmt::Error> {
                    match self {
                        Variant0ParseError::NonExistent { value } => {
                            write!(
                                f, "Value '{:?}' does not exist in the enumeration", value
                            )
                        }
                    }
                }
            }
            impl ::core::convert::TryFrom<::std::string::String> for Variant0 {
                type Error = Variant0ParseError;
                fn try_from(
                    value: ::std::string::String,
                ) -> ::core::result::Result<Self, Self::Error> {
                    match ::std::string::String::as_str(&value) {
                        "#all" => Ok(Variant0::All),
                        _ => {
                            Err(Variant0ParseError::NonExistent {
                                value,
                            })
                        }
                    }
                }
            }
            impl ::core::convert::From<Variant0> for ::std::string::String {
                fn from(value: Variant0) -> Self {
                    match value {
                        Variant0::All => ::std::string::String::from("#all"),
                    }
                }
            }
            #[derive(
                ::core::fmt::Debug,
                ::core::clone::Clone,
                ::core::marker::Copy,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq
            )]
            #[xvalue(with = variant_1_with)]
            pub enum Variant1 {
                Extension,
                Restriction,
                Substitution,
            }
            pub mod variant_1_with {
                pub fn deserialize<'de, D>(
                    deserializer: D,
                ) -> ::core::result::Result<super::Variant1, D::Error>
                where
                    D: ::xmlity::Deserializer<'de>,
                {
                    let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                        deserializer,
                    )?;
                    let value: ::std::string::String = text
                        .parse()
                        .map_err(::xmlity::de::Error::custom)?;
                    super::Variant1::try_from(value).map_err(::xmlity::de::Error::custom)
                }
                pub fn serialize<S>(
                    value: &super::Variant1,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::xmlity::Serializer,
                {
                    let value: ::std::string::String = ::core::clone::Clone::clone(value)
                        .into();
                    ::xmlity::Serialize::serialize(
                        ::std::string::String::as_str(
                            &::std::string::ToString::to_string(&value),
                        ),
                        serializer,
                    )
                }
            }
            #[derive(::core::fmt::Debug)]
            pub enum Variant1ParseError {
                NonExistent { value: ::std::string::String },
            }
            impl ::core::fmt::Display for Variant1ParseError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::result::Result<(), ::core::fmt::Error> {
                    match self {
                        Variant1ParseError::NonExistent { value } => {
                            write!(
                                f, "Value '{:?}' does not exist in the enumeration", value
                            )
                        }
                    }
                }
            }
            impl ::core::convert::TryFrom<::std::string::String> for Variant1 {
                type Error = Variant1ParseError;
                fn try_from(
                    value: ::std::string::String,
                ) -> ::core::result::Result<Self, Self::Error> {
                    match ::std::string::String::as_str(&value) {
                        "extension" => Ok(Variant1::Extension),
                        "restriction" => Ok(Variant1::Restriction),
                        "substitution" => Ok(Variant1::Substitution),
                        _ => {
                            Err(Variant1ParseError::NonExistent {
                                value,
                            })
                        }
                    }
                }
            }
            impl ::core::convert::From<Variant1> for ::std::string::String {
                fn from(value: Variant1) -> Self {
                    match value {
                        Variant1::Extension => ::std::string::String::from("extension"),
                        Variant1::Restriction => {
                            ::std::string::String::from("restriction")
                        }
                        Variant1::Substitution => {
                            ::std::string::String::from("substitution")
                        }
                    }
                }
            }
        }
        impl ::core::convert::From<variant_variants::Variant0> for BlockSet {
            fn from(value: variant_variants::Variant0) -> Self {
                BlockSet::Variant0(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<::xmlity_ns::List<variant_variants::Variant1>>
        for BlockSet {
            fn from(value: ::xmlity_ns::List<variant_variants::Variant1>) -> Self {
                BlockSet::Variant1(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum BlockSet {
            Variant0(::std::boxed::Box<variant_variants::Variant0>),
            Variant1(::std::boxed::Box<::xmlity_ns::List<variant_variants::Variant1>>),
        }
    }
    pub type BlockSet = block_set_items::BlockSet;
    pub mod derivation_control_items {
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq
        )]
        #[xvalue(with = derivation_control_with)]
        pub enum DerivationControl {
            Substitution,
            Extension,
            Restriction,
            List,
            Union,
        }
        pub mod derivation_control_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::DerivationControl, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::DerivationControl::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::DerivationControl,
                serializer: S,
            ) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::xmlity::Serializer,
            {
                let value: ::std::string::String = ::core::clone::Clone::clone(value)
                    .into();
                ::xmlity::Serialize::serialize(
                    ::std::string::String::as_str(
                        &::std::string::ToString::to_string(&value),
                    ),
                    serializer,
                )
            }
        }
        #[derive(::core::fmt::Debug)]
        pub enum DerivationControlParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for DerivationControlParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    DerivationControlParseError::NonExistent { value } => {
                        write!(
                            f, "Value '{:?}' does not exist in the enumeration", value
                        )
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for DerivationControl {
            type Error = DerivationControlParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "substitution" => Ok(DerivationControl::Substitution),
                    "extension" => Ok(DerivationControl::Extension),
                    "restriction" => Ok(DerivationControl::Restriction),
                    "list" => Ok(DerivationControl::List),
                    "union" => Ok(DerivationControl::Union),
                    _ => {
                        Err(DerivationControlParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<DerivationControl> for ::std::string::String {
            fn from(value: DerivationControl) -> Self {
                match value {
                    DerivationControl::Substitution => {
                        ::std::string::String::from("substitution")
                    }
                    DerivationControl::Extension => {
                        ::std::string::String::from("extension")
                    }
                    DerivationControl::Restriction => {
                        ::std::string::String::from("restriction")
                    }
                    DerivationControl::List => ::std::string::String::from("list"),
                    DerivationControl::Union => ::std::string::String::from("union"),
                }
            }
        }
    }
    pub type DerivationControl = derivation_control_items::DerivationControl;
    pub mod derivation_set_items {
        pub mod variant_variants {
            #[derive(
                ::core::fmt::Debug,
                ::core::clone::Clone,
                ::core::marker::Copy,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq
            )]
            #[xvalue(with = variant_0_with)]
            pub enum Variant0 {
                All,
            }
            pub mod variant_0_with {
                pub fn deserialize<'de, D>(
                    deserializer: D,
                ) -> ::core::result::Result<super::Variant0, D::Error>
                where
                    D: ::xmlity::Deserializer<'de>,
                {
                    let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                        deserializer,
                    )?;
                    let value: ::std::string::String = text
                        .parse()
                        .map_err(::xmlity::de::Error::custom)?;
                    super::Variant0::try_from(value).map_err(::xmlity::de::Error::custom)
                }
                pub fn serialize<S>(
                    value: &super::Variant0,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::xmlity::Serializer,
                {
                    let value: ::std::string::String = ::core::clone::Clone::clone(value)
                        .into();
                    ::xmlity::Serialize::serialize(
                        ::std::string::String::as_str(
                            &::std::string::ToString::to_string(&value),
                        ),
                        serializer,
                    )
                }
            }
            #[derive(::core::fmt::Debug)]
            pub enum Variant0ParseError {
                NonExistent { value: ::std::string::String },
            }
            impl ::core::fmt::Display for Variant0ParseError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::result::Result<(), ::core::fmt::Error> {
                    match self {
                        Variant0ParseError::NonExistent { value } => {
                            write!(
                                f, "Value '{:?}' does not exist in the enumeration", value
                            )
                        }
                    }
                }
            }
            impl ::core::convert::TryFrom<::std::string::String> for Variant0 {
                type Error = Variant0ParseError;
                fn try_from(
                    value: ::std::string::String,
                ) -> ::core::result::Result<Self, Self::Error> {
                    match ::std::string::String::as_str(&value) {
                        "#all" => Ok(Variant0::All),
                        _ => {
                            Err(Variant0ParseError::NonExistent {
                                value,
                            })
                        }
                    }
                }
            }
            impl ::core::convert::From<Variant0> for ::std::string::String {
                fn from(value: Variant0) -> Self {
                    match value {
                        Variant0::All => ::std::string::String::from("#all"),
                    }
                }
            }
        }
        impl ::core::convert::From<variant_variants::Variant0> for DerivationSet {
            fn from(value: variant_variants::Variant0) -> Self {
                DerivationSet::Variant0(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<
            ::xmlity_ns::List<::std::boxed::Box<crate::types::ReducedDerivationControl>>,
        > for DerivationSet {
            fn from(
                value: ::xmlity_ns::List<
                    ::std::boxed::Box<crate::types::ReducedDerivationControl>,
                >,
            ) -> Self {
                DerivationSet::Variant1(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum DerivationSet {
            Variant0(::std::boxed::Box<variant_variants::Variant0>),
            Variant1(
                ::std::boxed::Box<
                    ::xmlity_ns::List<
                        ::std::boxed::Box<crate::types::ReducedDerivationControl>,
                    >,
                >,
            ),
        }
    }
    pub type DerivationSet = derivation_set_items::DerivationSet;
    pub mod form_choice_items {
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq
        )]
        #[xvalue(with = form_choice_with)]
        pub enum FormChoice {
            Qualified,
            Unqualified,
        }
        pub mod form_choice_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::FormChoice, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::FormChoice::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::FormChoice,
                serializer: S,
            ) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::xmlity::Serializer,
            {
                let value: ::std::string::String = ::core::clone::Clone::clone(value)
                    .into();
                ::xmlity::Serialize::serialize(
                    ::std::string::String::as_str(
                        &::std::string::ToString::to_string(&value),
                    ),
                    serializer,
                )
            }
        }
        #[derive(::core::fmt::Debug)]
        pub enum FormChoiceParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for FormChoiceParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    FormChoiceParseError::NonExistent { value } => {
                        write!(
                            f, "Value '{:?}' does not exist in the enumeration", value
                        )
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for FormChoice {
            type Error = FormChoiceParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "qualified" => Ok(FormChoice::Qualified),
                    "unqualified" => Ok(FormChoice::Unqualified),
                    _ => {
                        Err(FormChoiceParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<FormChoice> for ::std::string::String {
            fn from(value: FormChoice) -> Self {
                match value {
                    FormChoice::Qualified => ::std::string::String::from("qualified"),
                    FormChoice::Unqualified => ::std::string::String::from("unqualified"),
                }
            }
        }
    }
    pub type FormChoice = form_choice_items::FormChoice;
    pub mod full_derivation_set_items {
        pub mod variant_variants {
            #[derive(
                ::core::fmt::Debug,
                ::core::clone::Clone,
                ::core::marker::Copy,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq
            )]
            #[xvalue(with = variant_0_with)]
            pub enum Variant0 {
                All,
            }
            pub mod variant_0_with {
                pub fn deserialize<'de, D>(
                    deserializer: D,
                ) -> ::core::result::Result<super::Variant0, D::Error>
                where
                    D: ::xmlity::Deserializer<'de>,
                {
                    let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                        deserializer,
                    )?;
                    let value: ::std::string::String = text
                        .parse()
                        .map_err(::xmlity::de::Error::custom)?;
                    super::Variant0::try_from(value).map_err(::xmlity::de::Error::custom)
                }
                pub fn serialize<S>(
                    value: &super::Variant0,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::xmlity::Serializer,
                {
                    let value: ::std::string::String = ::core::clone::Clone::clone(value)
                        .into();
                    ::xmlity::Serialize::serialize(
                        ::std::string::String::as_str(
                            &::std::string::ToString::to_string(&value),
                        ),
                        serializer,
                    )
                }
            }
            #[derive(::core::fmt::Debug)]
            pub enum Variant0ParseError {
                NonExistent { value: ::std::string::String },
            }
            impl ::core::fmt::Display for Variant0ParseError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::result::Result<(), ::core::fmt::Error> {
                    match self {
                        Variant0ParseError::NonExistent { value } => {
                            write!(
                                f, "Value '{:?}' does not exist in the enumeration", value
                            )
                        }
                    }
                }
            }
            impl ::core::convert::TryFrom<::std::string::String> for Variant0 {
                type Error = Variant0ParseError;
                fn try_from(
                    value: ::std::string::String,
                ) -> ::core::result::Result<Self, Self::Error> {
                    match ::std::string::String::as_str(&value) {
                        "#all" => Ok(Variant0::All),
                        _ => {
                            Err(Variant0ParseError::NonExistent {
                                value,
                            })
                        }
                    }
                }
            }
            impl ::core::convert::From<Variant0> for ::std::string::String {
                fn from(value: Variant0) -> Self {
                    match value {
                        Variant0::All => ::std::string::String::from("#all"),
                    }
                }
            }
        }
        impl ::core::convert::From<variant_variants::Variant0> for FullDerivationSet {
            fn from(value: variant_variants::Variant0) -> Self {
                FullDerivationSet::Variant0(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<
            ::xmlity_ns::List<::std::boxed::Box<crate::types::TypeDerivationControl>>,
        > for FullDerivationSet {
            fn from(
                value: ::xmlity_ns::List<
                    ::std::boxed::Box<crate::types::TypeDerivationControl>,
                >,
            ) -> Self {
                FullDerivationSet::Variant1(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum FullDerivationSet {
            Variant0(::std::boxed::Box<variant_variants::Variant0>),
            Variant1(
                ::std::boxed::Box<
                    ::xmlity_ns::List<
                        ::std::boxed::Box<crate::types::TypeDerivationControl>,
                    >,
                >,
            ),
        }
    }
    pub type FullDerivationSet = full_derivation_set_items::FullDerivationSet;
    pub mod namespace_list_items {
        impl ::core::convert::From<crate::types::SpecialNamespaceList>
        for NamespaceList {
            fn from(value: crate::types::SpecialNamespaceList) -> Self {
                NamespaceList::SpecialNamespaceList(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::types::BasicNamespaceList> for NamespaceList {
            fn from(value: crate::types::BasicNamespaceList) -> Self {
                NamespaceList::BasicNamespaceList(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum NamespaceList {
            SpecialNamespaceList(::std::boxed::Box<crate::types::SpecialNamespaceList>),
            BasicNamespaceList(::std::boxed::Box<crate::types::BasicNamespaceList>),
        }
    }
    pub type NamespaceList = namespace_list_items::NamespaceList;
    pub mod public_items {
        impl ::core::convert::From<::std::string::String> for Public {
            fn from(value: ::std::string::String) -> Self {
                Public(value)
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(with = public_with)]
        pub struct Public(pub ::std::string::String);
        pub mod public_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Public, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::Public::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Public,
                serializer: S,
            ) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::xmlity::Serializer,
            {
                let value: ::std::string::String = ::core::clone::Clone::clone(value)
                    .into();
                ::xmlity::Serialize::serialize(
                    ::std::string::String::as_str(
                        &::std::string::ToString::to_string(&value),
                    ),
                    serializer,
                )
            }
        }
        #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
        pub enum PublicParseError {}
        impl ::core::convert::From<Public> for ::std::string::String {
            fn from(value: Public) -> Self {
                value.0
            }
        }
    }
    pub type Public = public_items::Public;
    pub mod qname_list_items {
        pub mod qname_list_variants {
            #[derive(
                ::core::fmt::Debug,
                ::core::clone::Clone,
                ::core::marker::Copy,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq
            )]
            #[xvalue(with = variant_0_with)]
            pub enum Variant0 {
                Defined,
                DefinedSibling,
            }
            pub mod variant_0_with {
                pub fn deserialize<'de, D>(
                    deserializer: D,
                ) -> ::core::result::Result<super::Variant0, D::Error>
                where
                    D: ::xmlity::Deserializer<'de>,
                {
                    let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                        deserializer,
                    )?;
                    let value: ::std::string::String = text
                        .parse()
                        .map_err(::xmlity::de::Error::custom)?;
                    super::Variant0::try_from(value).map_err(::xmlity::de::Error::custom)
                }
                pub fn serialize<S>(
                    value: &super::Variant0,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::xmlity::Serializer,
                {
                    let value: ::std::string::String = ::core::clone::Clone::clone(value)
                        .into();
                    ::xmlity::Serialize::serialize(
                        ::std::string::String::as_str(
                            &::std::string::ToString::to_string(&value),
                        ),
                        serializer,
                    )
                }
            }
            #[derive(::core::fmt::Debug)]
            pub enum Variant0ParseError {
                NonExistent { value: ::std::string::String },
            }
            impl ::core::fmt::Display for Variant0ParseError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::result::Result<(), ::core::fmt::Error> {
                    match self {
                        Variant0ParseError::NonExistent { value } => {
                            write!(
                                f, "Value '{:?}' does not exist in the enumeration", value
                            )
                        }
                    }
                }
            }
            impl ::core::convert::TryFrom<::std::string::String> for Variant0 {
                type Error = Variant0ParseError;
                fn try_from(
                    value: ::std::string::String,
                ) -> ::core::result::Result<Self, Self::Error> {
                    match ::std::string::String::as_str(&value) {
                        "##defined" => Ok(Variant0::Defined),
                        "##definedSibling" => Ok(Variant0::DefinedSibling),
                        _ => {
                            Err(Variant0ParseError::NonExistent {
                                value,
                            })
                        }
                    }
                }
            }
            impl ::core::convert::From<Variant0> for ::std::string::String {
                fn from(value: Variant0) -> Self {
                    match value {
                        Variant0::Defined => ::std::string::String::from("##defined"),
                        Variant0::DefinedSibling => {
                            ::std::string::String::from("##definedSibling")
                        }
                    }
                }
            }
        }
        impl ::core::convert::From<crate::types::QName> for QnameList {
            fn from(value: crate::types::QName) -> Self {
                QnameList::Qname(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<qname_list_variants::Variant0> for QnameList {
            fn from(value: qname_list_variants::Variant0) -> Self {
                QnameList::Variant0(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum QnameList {
            Qname(::std::boxed::Box<crate::types::QName>),
            Variant0(::std::boxed::Box<qname_list_variants::Variant0>),
        }
    }
    pub type QnameList = ::xmlity_ns::List<qname_list_items::QnameList>;
    pub mod qname_list_a_items {
        pub mod qname_list_a_variants {
            #[derive(
                ::core::fmt::Debug,
                ::core::clone::Clone,
                ::core::marker::Copy,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq
            )]
            #[xvalue(with = variant_0_with)]
            pub enum Variant0 {
                Defined,
            }
            pub mod variant_0_with {
                pub fn deserialize<'de, D>(
                    deserializer: D,
                ) -> ::core::result::Result<super::Variant0, D::Error>
                where
                    D: ::xmlity::Deserializer<'de>,
                {
                    let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                        deserializer,
                    )?;
                    let value: ::std::string::String = text
                        .parse()
                        .map_err(::xmlity::de::Error::custom)?;
                    super::Variant0::try_from(value).map_err(::xmlity::de::Error::custom)
                }
                pub fn serialize<S>(
                    value: &super::Variant0,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::xmlity::Serializer,
                {
                    let value: ::std::string::String = ::core::clone::Clone::clone(value)
                        .into();
                    ::xmlity::Serialize::serialize(
                        ::std::string::String::as_str(
                            &::std::string::ToString::to_string(&value),
                        ),
                        serializer,
                    )
                }
            }
            #[derive(::core::fmt::Debug)]
            pub enum Variant0ParseError {
                NonExistent { value: ::std::string::String },
            }
            impl ::core::fmt::Display for Variant0ParseError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::result::Result<(), ::core::fmt::Error> {
                    match self {
                        Variant0ParseError::NonExistent { value } => {
                            write!(
                                f, "Value '{:?}' does not exist in the enumeration", value
                            )
                        }
                    }
                }
            }
            impl ::core::convert::TryFrom<::std::string::String> for Variant0 {
                type Error = Variant0ParseError;
                fn try_from(
                    value: ::std::string::String,
                ) -> ::core::result::Result<Self, Self::Error> {
                    match ::std::string::String::as_str(&value) {
                        "##defined" => Ok(Variant0::Defined),
                        _ => {
                            Err(Variant0ParseError::NonExistent {
                                value,
                            })
                        }
                    }
                }
            }
            impl ::core::convert::From<Variant0> for ::std::string::String {
                fn from(value: Variant0) -> Self {
                    match value {
                        Variant0::Defined => ::std::string::String::from("##defined"),
                    }
                }
            }
        }
        impl ::core::convert::From<crate::types::QName> for QnameListA {
            fn from(value: crate::types::QName) -> Self {
                QnameListA::Qname(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<qname_list_a_variants::Variant0> for QnameListA {
            fn from(value: qname_list_a_variants::Variant0) -> Self {
                QnameListA::Variant0(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum QnameListA {
            Qname(::std::boxed::Box<crate::types::QName>),
            Variant0(::std::boxed::Box<qname_list_a_variants::Variant0>),
        }
    }
    pub type QnameListA = ::xmlity_ns::List<qname_list_a_items::QnameListA>;
    pub mod reduced_derivation_control_items {
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq
        )]
        #[xvalue(with = reduced_derivation_control_with)]
        pub enum ReducedDerivationControl {
            Extension,
            Restriction,
        }
        pub mod reduced_derivation_control_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::ReducedDerivationControl, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::ReducedDerivationControl::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::ReducedDerivationControl,
                serializer: S,
            ) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::xmlity::Serializer,
            {
                let value: ::std::string::String = ::core::clone::Clone::clone(value)
                    .into();
                ::xmlity::Serialize::serialize(
                    ::std::string::String::as_str(
                        &::std::string::ToString::to_string(&value),
                    ),
                    serializer,
                )
            }
        }
        #[derive(::core::fmt::Debug)]
        pub enum ReducedDerivationControlParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for ReducedDerivationControlParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    ReducedDerivationControlParseError::NonExistent { value } => {
                        write!(
                            f, "Value '{:?}' does not exist in the enumeration", value
                        )
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String>
        for ReducedDerivationControl {
            type Error = ReducedDerivationControlParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "extension" => Ok(ReducedDerivationControl::Extension),
                    "restriction" => Ok(ReducedDerivationControl::Restriction),
                    _ => {
                        Err(ReducedDerivationControlParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<ReducedDerivationControl> for ::std::string::String {
            fn from(value: ReducedDerivationControl) -> Self {
                match value {
                    ReducedDerivationControl::Extension => {
                        ::std::string::String::from("extension")
                    }
                    ReducedDerivationControl::Restriction => {
                        ::std::string::String::from("restriction")
                    }
                }
            }
        }
    }
    pub type ReducedDerivationControl = reduced_derivation_control_items::ReducedDerivationControl;
    pub mod simple_derivation_set_items {
        pub mod variant_variants {
            #[derive(
                ::core::fmt::Debug,
                ::core::clone::Clone,
                ::core::marker::Copy,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq
            )]
            #[xvalue(with = variant_0_with)]
            pub enum Variant0 {
                All,
            }
            pub mod variant_0_with {
                pub fn deserialize<'de, D>(
                    deserializer: D,
                ) -> ::core::result::Result<super::Variant0, D::Error>
                where
                    D: ::xmlity::Deserializer<'de>,
                {
                    let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                        deserializer,
                    )?;
                    let value: ::std::string::String = text
                        .parse()
                        .map_err(::xmlity::de::Error::custom)?;
                    super::Variant0::try_from(value).map_err(::xmlity::de::Error::custom)
                }
                pub fn serialize<S>(
                    value: &super::Variant0,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::xmlity::Serializer,
                {
                    let value: ::std::string::String = ::core::clone::Clone::clone(value)
                        .into();
                    ::xmlity::Serialize::serialize(
                        ::std::string::String::as_str(
                            &::std::string::ToString::to_string(&value),
                        ),
                        serializer,
                    )
                }
            }
            #[derive(::core::fmt::Debug)]
            pub enum Variant0ParseError {
                NonExistent { value: ::std::string::String },
            }
            impl ::core::fmt::Display for Variant0ParseError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::result::Result<(), ::core::fmt::Error> {
                    match self {
                        Variant0ParseError::NonExistent { value } => {
                            write!(
                                f, "Value '{:?}' does not exist in the enumeration", value
                            )
                        }
                    }
                }
            }
            impl ::core::convert::TryFrom<::std::string::String> for Variant0 {
                type Error = Variant0ParseError;
                fn try_from(
                    value: ::std::string::String,
                ) -> ::core::result::Result<Self, Self::Error> {
                    match ::std::string::String::as_str(&value) {
                        "#all" => Ok(Variant0::All),
                        _ => {
                            Err(Variant0ParseError::NonExistent {
                                value,
                            })
                        }
                    }
                }
            }
            impl ::core::convert::From<Variant0> for ::std::string::String {
                fn from(value: Variant0) -> Self {
                    match value {
                        Variant0::All => ::std::string::String::from("#all"),
                    }
                }
            }
            #[derive(
                ::core::fmt::Debug,
                ::core::clone::Clone,
                ::core::marker::Copy,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq
            )]
            #[xvalue(with = variant_1_with)]
            pub enum Variant1 {
                List,
                Union,
                Restriction,
                Extension,
            }
            pub mod variant_1_with {
                pub fn deserialize<'de, D>(
                    deserializer: D,
                ) -> ::core::result::Result<super::Variant1, D::Error>
                where
                    D: ::xmlity::Deserializer<'de>,
                {
                    let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                        deserializer,
                    )?;
                    let value: ::std::string::String = text
                        .parse()
                        .map_err(::xmlity::de::Error::custom)?;
                    super::Variant1::try_from(value).map_err(::xmlity::de::Error::custom)
                }
                pub fn serialize<S>(
                    value: &super::Variant1,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::xmlity::Serializer,
                {
                    let value: ::std::string::String = ::core::clone::Clone::clone(value)
                        .into();
                    ::xmlity::Serialize::serialize(
                        ::std::string::String::as_str(
                            &::std::string::ToString::to_string(&value),
                        ),
                        serializer,
                    )
                }
            }
            #[derive(::core::fmt::Debug)]
            pub enum Variant1ParseError {
                NonExistent { value: ::std::string::String },
            }
            impl ::core::fmt::Display for Variant1ParseError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::result::Result<(), ::core::fmt::Error> {
                    match self {
                        Variant1ParseError::NonExistent { value } => {
                            write!(
                                f, "Value '{:?}' does not exist in the enumeration", value
                            )
                        }
                    }
                }
            }
            impl ::core::convert::TryFrom<::std::string::String> for Variant1 {
                type Error = Variant1ParseError;
                fn try_from(
                    value: ::std::string::String,
                ) -> ::core::result::Result<Self, Self::Error> {
                    match ::std::string::String::as_str(&value) {
                        "list" => Ok(Variant1::List),
                        "union" => Ok(Variant1::Union),
                        "restriction" => Ok(Variant1::Restriction),
                        "extension" => Ok(Variant1::Extension),
                        _ => {
                            Err(Variant1ParseError::NonExistent {
                                value,
                            })
                        }
                    }
                }
            }
            impl ::core::convert::From<Variant1> for ::std::string::String {
                fn from(value: Variant1) -> Self {
                    match value {
                        Variant1::List => ::std::string::String::from("list"),
                        Variant1::Union => ::std::string::String::from("union"),
                        Variant1::Restriction => {
                            ::std::string::String::from("restriction")
                        }
                        Variant1::Extension => ::std::string::String::from("extension"),
                    }
                }
            }
        }
        impl ::core::convert::From<variant_variants::Variant0> for SimpleDerivationSet {
            fn from(value: variant_variants::Variant0) -> Self {
                SimpleDerivationSet::Variant0(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<::xmlity_ns::List<variant_variants::Variant1>>
        for SimpleDerivationSet {
            fn from(value: ::xmlity_ns::List<variant_variants::Variant1>) -> Self {
                SimpleDerivationSet::Variant1(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum SimpleDerivationSet {
            Variant0(::std::boxed::Box<variant_variants::Variant0>),
            Variant1(::std::boxed::Box<::xmlity_ns::List<variant_variants::Variant1>>),
        }
    }
    pub type SimpleDerivationSet = simple_derivation_set_items::SimpleDerivationSet;
    pub mod special_namespace_list_items {
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq
        )]
        #[xvalue(with = special_namespace_list_with)]
        pub enum SpecialNamespaceList {
            Any,
            Other,
        }
        pub mod special_namespace_list_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::SpecialNamespaceList, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::SpecialNamespaceList::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::SpecialNamespaceList,
                serializer: S,
            ) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::xmlity::Serializer,
            {
                let value: ::std::string::String = ::core::clone::Clone::clone(value)
                    .into();
                ::xmlity::Serialize::serialize(
                    ::std::string::String::as_str(
                        &::std::string::ToString::to_string(&value),
                    ),
                    serializer,
                )
            }
        }
        #[derive(::core::fmt::Debug)]
        pub enum SpecialNamespaceListParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for SpecialNamespaceListParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    SpecialNamespaceListParseError::NonExistent { value } => {
                        write!(
                            f, "Value '{:?}' does not exist in the enumeration", value
                        )
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for SpecialNamespaceList {
            type Error = SpecialNamespaceListParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "##any" => Ok(SpecialNamespaceList::Any),
                    "##other" => Ok(SpecialNamespaceList::Other),
                    _ => {
                        Err(SpecialNamespaceListParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<SpecialNamespaceList> for ::std::string::String {
            fn from(value: SpecialNamespaceList) -> Self {
                match value {
                    SpecialNamespaceList::Any => ::std::string::String::from("##any"),
                    SpecialNamespaceList::Other => ::std::string::String::from("##other"),
                }
            }
        }
    }
    pub type SpecialNamespaceList = special_namespace_list_items::SpecialNamespaceList;
    pub mod type_derivation_control_items {
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq
        )]
        #[xvalue(with = type_derivation_control_with)]
        pub enum TypeDerivationControl {
            Extension,
            Restriction,
            List,
            Union,
        }
        pub mod type_derivation_control_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::TypeDerivationControl, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::TypeDerivationControl::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::TypeDerivationControl,
                serializer: S,
            ) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::xmlity::Serializer,
            {
                let value: ::std::string::String = ::core::clone::Clone::clone(value)
                    .into();
                ::xmlity::Serialize::serialize(
                    ::std::string::String::as_str(
                        &::std::string::ToString::to_string(&value),
                    ),
                    serializer,
                )
            }
        }
        #[derive(::core::fmt::Debug)]
        pub enum TypeDerivationControlParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for TypeDerivationControlParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    TypeDerivationControlParseError::NonExistent { value } => {
                        write!(
                            f, "Value '{:?}' does not exist in the enumeration", value
                        )
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for TypeDerivationControl {
            type Error = TypeDerivationControlParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "extension" => Ok(TypeDerivationControl::Extension),
                    "restriction" => Ok(TypeDerivationControl::Restriction),
                    "list" => Ok(TypeDerivationControl::List),
                    "union" => Ok(TypeDerivationControl::Union),
                    _ => {
                        Err(TypeDerivationControlParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<TypeDerivationControl> for ::std::string::String {
            fn from(value: TypeDerivationControl) -> Self {
                match value {
                    TypeDerivationControl::Extension => {
                        ::std::string::String::from("extension")
                    }
                    TypeDerivationControl::Restriction => {
                        ::std::string::String::from("restriction")
                    }
                    TypeDerivationControl::List => ::std::string::String::from("list"),
                    TypeDerivationControl::Union => ::std::string::String::from("union"),
                }
            }
        }
    }
    pub type TypeDerivationControl = type_derivation_control_items::TypeDerivationControl;
    pub mod xpath_default_namespace_items {
        pub mod xpath_default_namespace_variants {
            #[derive(
                ::core::fmt::Debug,
                ::core::clone::Clone,
                ::core::marker::Copy,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq
            )]
            #[xvalue(with = variant_0_with)]
            pub enum Variant0 {
                DefaultNamespace,
                TargetNamespace,
                Local,
            }
            pub mod variant_0_with {
                pub fn deserialize<'de, D>(
                    deserializer: D,
                ) -> ::core::result::Result<super::Variant0, D::Error>
                where
                    D: ::xmlity::Deserializer<'de>,
                {
                    let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                        deserializer,
                    )?;
                    let value: ::std::string::String = text
                        .parse()
                        .map_err(::xmlity::de::Error::custom)?;
                    super::Variant0::try_from(value).map_err(::xmlity::de::Error::custom)
                }
                pub fn serialize<S>(
                    value: &super::Variant0,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::xmlity::Serializer,
                {
                    let value: ::std::string::String = ::core::clone::Clone::clone(value)
                        .into();
                    ::xmlity::Serialize::serialize(
                        ::std::string::String::as_str(
                            &::std::string::ToString::to_string(&value),
                        ),
                        serializer,
                    )
                }
            }
            #[derive(::core::fmt::Debug)]
            pub enum Variant0ParseError {
                NonExistent { value: ::std::string::String },
            }
            impl ::core::fmt::Display for Variant0ParseError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::result::Result<(), ::core::fmt::Error> {
                    match self {
                        Variant0ParseError::NonExistent { value } => {
                            write!(
                                f, "Value '{:?}' does not exist in the enumeration", value
                            )
                        }
                    }
                }
            }
            impl ::core::convert::TryFrom<::std::string::String> for Variant0 {
                type Error = Variant0ParseError;
                fn try_from(
                    value: ::std::string::String,
                ) -> ::core::result::Result<Self, Self::Error> {
                    match ::std::string::String::as_str(&value) {
                        "##defaultNamespace" => Ok(Variant0::DefaultNamespace),
                        "##targetNamespace" => Ok(Variant0::TargetNamespace),
                        "##local" => Ok(Variant0::Local),
                        _ => {
                            Err(Variant0ParseError::NonExistent {
                                value,
                            })
                        }
                    }
                }
            }
            impl ::core::convert::From<Variant0> for ::std::string::String {
                fn from(value: Variant0) -> Self {
                    match value {
                        Variant0::DefaultNamespace => {
                            ::std::string::String::from("##defaultNamespace")
                        }
                        Variant0::TargetNamespace => {
                            ::std::string::String::from("##targetNamespace")
                        }
                        Variant0::Local => ::std::string::String::from("##local"),
                    }
                }
            }
        }
        impl ::core::convert::From<crate::types::TargetNamespace>
        for XpathDefaultNamespace {
            fn from(value: crate::types::TargetNamespace) -> Self {
                XpathDefaultNamespace::AnyURI(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<xpath_default_namespace_variants::Variant0>
        for XpathDefaultNamespace {
            fn from(value: xpath_default_namespace_variants::Variant0) -> Self {
                XpathDefaultNamespace::Variant0(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum XpathDefaultNamespace {
            AnyURI(::std::boxed::Box<crate::types::TargetNamespace>),
            Variant0(::std::boxed::Box<xpath_default_namespace_variants::Variant0>),
        }
    }
    pub type XpathDefaultNamespace = xpath_default_namespace_items::XpathDefaultNamespace;
    pub mod all_items {
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq
        )]
        #[xvalue(with = test_enum_with)]
        #[repr(usize)]
        pub enum MinOccursValue {
            U0 = 0usize,
            U1 = 1usize,
        }
        pub mod test_enum_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::MinOccursValue, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: usize = text.parse().map_err(::xmlity::de::Error::custom)?;
                super::MinOccursValue::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::MinOccursValue,
                serializer: S,
            ) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::xmlity::Serializer,
            {
                let value: usize = ::core::clone::Clone::clone(value).into();
                ::xmlity::Serialize::serialize(
                    ::std::string::String::as_str(
                        &::std::string::ToString::to_string(&value),
                    ),
                    serializer,
                )
            }
        }
        #[derive(::core::fmt::Debug)]
        pub enum TestEnumParseError {
            NonExistent { value: usize },
        }
        impl ::core::fmt::Display for TestEnumParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    TestEnumParseError::NonExistent { value } => {
                        write!(
                            f, "Value '{:?}' does not exist in the enumeration", value
                        )
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<usize> for MinOccursValue {
            type Error = TestEnumParseError;
            fn try_from(value: usize) -> ::core::result::Result<Self, Self::Error> {
                match value {
                    0usize => Ok(MinOccursValue::U0),
                    1usize => Ok(MinOccursValue::U1),
                    _ => {
                        Err(TestEnumParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<MinOccursValue> for usize {
            fn from(value: MinOccursValue) -> Self {
                match value {
                    MinOccursValue::U0 => 0usize,
                    MinOccursValue::U1 => 1usize,
                }
            }
        }
        pub mod max_occurs_value_variants {
            #[derive(
                ::core::fmt::Debug,
                ::core::clone::Clone,
                ::core::marker::Copy,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq
            )]
            #[xvalue(with = variant_0_with)]
            pub enum Variant0 {
                Unbounded,
            }
            pub mod variant_0_with {
                pub fn deserialize<'de, D>(
                    deserializer: D,
                ) -> ::core::result::Result<super::Variant0, D::Error>
                where
                    D: ::xmlity::Deserializer<'de>,
                {
                    let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                        deserializer,
                    )?;
                    let value: ::std::string::String = text
                        .parse()
                        .map_err(::xmlity::de::Error::custom)?;
                    super::Variant0::try_from(value).map_err(::xmlity::de::Error::custom)
                }
                pub fn serialize<S>(
                    value: &super::Variant0,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::xmlity::Serializer,
                {
                    let value: ::std::string::String = ::core::clone::Clone::clone(value)
                        .into();
                    ::xmlity::Serialize::serialize(
                        ::std::string::String::as_str(
                            &::std::string::ToString::to_string(&value),
                        ),
                        serializer,
                    )
                }
            }
            #[derive(::core::fmt::Debug)]
            pub enum Variant0ParseError {
                NonExistent { value: ::std::string::String },
            }
            impl ::core::fmt::Display for Variant0ParseError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::result::Result<(), ::core::fmt::Error> {
                    match self {
                        Variant0ParseError::NonExistent { value } => {
                            write!(
                                f, "Value '{:?}' does not exist in the enumeration", value
                            )
                        }
                    }
                }
            }
            impl ::core::convert::TryFrom<::std::string::String> for Variant0 {
                type Error = Variant0ParseError;
                fn try_from(
                    value: ::std::string::String,
                ) -> ::core::result::Result<Self, Self::Error> {
                    match ::std::string::String::as_str(&value) {
                        "unbounded" => Ok(Variant0::Unbounded),
                        _ => {
                            Err(Variant0ParseError::NonExistent {
                                value,
                            })
                        }
                    }
                }
            }
            impl ::core::convert::From<Variant0> for ::std::string::String {
                fn from(value: Variant0) -> Self {
                    match value {
                        Variant0::Unbounded => ::std::string::String::from("unbounded"),
                    }
                }
            }
        }
        impl ::core::convert::From<usize> for MaxOccursValue {
            fn from(value: usize) -> Self {
                MaxOccursValue::NonNegativeInteger(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<max_occurs_value_variants::Variant0>
        for MaxOccursValue {
            fn from(value: max_occurs_value_variants::Variant0) -> Self {
                MaxOccursValue::Variant0(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum MaxOccursValue {
            NonNegativeInteger(::std::boxed::Box<usize>),
            Variant0(::std::boxed::Box<max_occurs_value_variants::Variant0>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct All {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "minOccurs", optional)]
        pub min_occurs: ::core::option::Option<all_items::MinOccursValue>,
        #[xattribute(name = "maxOccurs", optional)]
        pub max_occurs: ::core::option::Option<all_items::MaxOccursValue>,
        pub all_model: ::std::boxed::Box<crate::groups::AllModel>,
    }
    pub mod alt_type_items {
        impl ::core::convert::From<crate::types::LocalSimpleType> for Type {
            fn from(value: crate::types::LocalSimpleType) -> Self {
                Type::SimpleType(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::types::LocalComplexType> for Type {
            fn from(value: crate::types::LocalComplexType) -> Self {
                Type::ComplexType(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Type {
            #[xelement(
                name = "simpleType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            SimpleType(#[xgroup] ::std::boxed::Box<crate::types::LocalSimpleType>),
            #[xelement(
                name = "complexType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            ComplexType(#[xgroup] ::std::boxed::Box<crate::types::LocalComplexType>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct AltType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "test", optional)]
        pub test: ::core::option::Option<String>,
        #[xattribute(name = "type", optional)]
        pub type_attribute: ::core::option::Option<crate::types::QName>,
        #[xattribute(name = "xpathDefaultNamespace", optional)]
        pub xpath_default_namespace: ::core::option::Option<
            ::std::boxed::Box<crate::types::XpathDefaultNamespace>,
        >,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        #[xvalue(default)]
        pub type_: ::core::option::Option<alt_type_items::Type>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Annotated {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct AnyType {
        pub child_0: ::xmlity::XmlValue,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Assertion {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "test", optional)]
        pub test: ::core::option::Option<String>,
        #[xattribute(name = "xpathDefaultNamespace", optional)]
        pub xpath_default_namespace: ::core::option::Option<
            ::std::boxed::Box<crate::types::XpathDefaultNamespace>,
        >,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
    }
    pub mod attribute_items {
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq
        )]
        #[xvalue(with = use_value_with)]
        pub enum UseValue {
            Prohibited,
            Optional,
            Required,
        }
        pub mod use_value_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::UseValue, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::UseValue::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::UseValue,
                serializer: S,
            ) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::xmlity::Serializer,
            {
                let value: ::std::string::String = ::core::clone::Clone::clone(value)
                    .into();
                ::xmlity::Serialize::serialize(
                    ::std::string::String::as_str(
                        &::std::string::ToString::to_string(&value),
                    ),
                    serializer,
                )
            }
        }
        #[derive(::core::fmt::Debug)]
        pub enum UseValueParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for UseValueParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    UseValueParseError::NonExistent { value } => {
                        write!(
                            f, "Value '{:?}' does not exist in the enumeration", value
                        )
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for UseValue {
            type Error = UseValueParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "prohibited" => Ok(UseValue::Prohibited),
                    "optional" => Ok(UseValue::Optional),
                    "required" => Ok(UseValue::Required),
                    _ => {
                        Err(UseValueParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<UseValue> for ::std::string::String {
            fn from(value: UseValue) -> Self {
                match value {
                    UseValue::Prohibited => ::std::string::String::from("prohibited"),
                    UseValue::Optional => ::std::string::String::from("optional"),
                    UseValue::Required => ::std::string::String::from("required"),
                }
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Attribute {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name", optional)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xattribute(name = "ref", optional)]
        pub ref_: ::core::option::Option<crate::types::QName>,
        #[xattribute(name = "type", optional)]
        pub type_: ::core::option::Option<crate::types::QName>,
        #[xattribute(name = "use", optional)]
        pub use_: ::core::option::Option<attribute_items::UseValue>,
        #[xattribute(name = "default", optional)]
        pub default: ::core::option::Option<String>,
        #[xattribute(name = "fixed", optional)]
        pub fixed: ::core::option::Option<String>,
        #[xattribute(name = "form", optional)]
        pub form: ::core::option::Option<::std::boxed::Box<crate::types::FormChoice>>,
        #[xattribute(name = "targetNamespace", optional)]
        pub target_namespace: ::core::option::Option<crate::types::TargetNamespace>,
        #[xattribute(name = "inheritable", optional)]
        pub inheritable: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        #[xelement(
            name = "simpleType",
            namespace = "http://www.w3.org/2001/XMLSchema",
            group,
            optional
        )]
        pub simple_type: ::core::option::Option<
            ::std::boxed::Box<crate::types::LocalSimpleType>,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct AttributeGroup {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name", optional)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xattribute(name = "ref", optional)]
        pub ref_: ::core::option::Option<crate::types::QName>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        pub attr_decls: ::std::boxed::Box<crate::groups::AttrDecls>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct AttributeGroupRef {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "ref")]
        pub ref_: crate::types::QName,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct ComplexBaseType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name", optional)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xattribute(name = "mixed", optional)]
        pub mixed: ::core::option::Option<bool>,
        #[xattribute(name = "abstract", optional)]
        pub abstract_: ::core::option::Option<bool>,
        #[xattribute(name = "final", optional)]
        pub final_: ::core::option::Option<
            ::std::boxed::Box<crate::types::DerivationSet>,
        >,
        #[xattribute(name = "block", optional)]
        pub block: ::core::option::Option<
            ::std::boxed::Box<crate::types::DerivationSet>,
        >,
        #[xattribute(name = "defaultAttributesApply", optional)]
        pub default_attributes_apply: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        pub complex_type_model: ::std::boxed::Box<crate::groups::ComplexTypeModel>,
    }
    pub mod complex_restriction_type_items {
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::bon::Builder,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(order = "strict")]
        pub struct Child1 {
            #[xvalue(default)]
            pub open_content: ::core::option::Option<
                ::std::boxed::Box<crate::OpenContent>,
            >,
            pub type_def_particle: ::std::boxed::Box<crate::groups::TypeDefParticle>,
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct ComplexRestrictionType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "base")]
        pub base: crate::types::QName,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        #[xvalue(default)]
        pub child_1: ::core::option::Option<complex_restriction_type_items::Child1>,
        pub attr_decls: ::std::boxed::Box<crate::groups::AttrDecls>,
        pub assertions: ::std::boxed::Box<crate::groups::Assertions>,
    }
    pub mod element_items {
        impl ::core::convert::From<crate::types::LocalSimpleType> for Type {
            fn from(value: crate::types::LocalSimpleType) -> Self {
                Type::SimpleType(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::types::LocalComplexType> for Type {
            fn from(value: crate::types::LocalComplexType) -> Self {
                Type::ComplexType(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Type {
            #[xelement(
                name = "simpleType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            SimpleType(#[xgroup] ::std::boxed::Box<crate::types::LocalSimpleType>),
            #[xelement(
                name = "complexType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            ComplexType(#[xgroup] ::std::boxed::Box<crate::types::LocalComplexType>),
        }
        impl ::core::convert::From<crate::types::AltType> for Alternative {
            fn from(value: crate::types::AltType) -> Self {
                Alternative(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xelement(
            name = "alternative",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        pub struct Alternative(#[xgroup] pub ::std::boxed::Box<crate::types::AltType>);
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Element {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name", optional)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xattribute(name = "ref", optional)]
        pub ref_: ::core::option::Option<crate::types::QName>,
        #[xattribute(name = "type", optional)]
        pub type_attribute: ::core::option::Option<crate::types::QName>,
        #[xattribute(name = "substitutionGroup", optional)]
        pub substitution_group: ::core::option::Option<
            ::xmlity_ns::List<crate::types::QName>,
        >,
        #[xattribute(name = "minOccurs", optional)]
        pub min_occurs: ::core::option::Option<usize>,
        #[xattribute(name = "maxOccurs", optional)]
        pub max_occurs: ::core::option::Option<::std::boxed::Box<crate::types::AllNNI>>,
        #[xattribute(name = "default", optional)]
        pub default: ::core::option::Option<String>,
        #[xattribute(name = "fixed", optional)]
        pub fixed: ::core::option::Option<String>,
        #[xattribute(name = "nillable", optional)]
        pub nillable: ::core::option::Option<bool>,
        #[xattribute(name = "abstract", optional)]
        pub abstract_: ::core::option::Option<bool>,
        #[xattribute(name = "final", optional)]
        pub final_: ::core::option::Option<
            ::std::boxed::Box<crate::types::DerivationSet>,
        >,
        #[xattribute(name = "block", optional)]
        pub block: ::core::option::Option<::std::boxed::Box<crate::types::BlockSet>>,
        #[xattribute(name = "form", optional)]
        pub form: ::core::option::Option<::std::boxed::Box<crate::types::FormChoice>>,
        #[xattribute(name = "targetNamespace", optional)]
        pub target_namespace: ::core::option::Option<crate::types::TargetNamespace>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        #[xvalue(default)]
        pub type_: ::core::option::Option<element_items::Type>,
        #[xvalue(default)]
        #[builder(default)]
        pub alternative: ::std::vec::Vec<element_items::Alternative>,
        #[xvalue(default)]
        #[builder(default)]
        pub identity_constraint: ::std::vec::Vec<crate::groups::IdentityConstraint>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct ExplicitGroup {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "minOccurs", optional)]
        pub min_occurs: ::core::option::Option<usize>,
        #[xattribute(name = "maxOccurs", optional)]
        pub max_occurs: ::core::option::Option<::std::boxed::Box<crate::types::AllNNI>>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        #[xvalue(default)]
        #[builder(default)]
        pub nested_particle: ::std::vec::Vec<crate::groups::NestedParticle>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct ExtensionType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "base")]
        pub base: crate::types::QName,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        #[xvalue(default)]
        pub open_content: ::core::option::Option<::std::boxed::Box<crate::OpenContent>>,
        #[xvalue(default)]
        pub type_def_particle: ::core::option::Option<
            ::std::boxed::Box<crate::groups::TypeDefParticle>,
        >,
        pub attr_decls: ::std::boxed::Box<crate::groups::AttrDecls>,
        pub assertions: ::std::boxed::Box<crate::groups::Assertions>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Facet {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "value")]
        pub value: String,
        #[xattribute(name = "fixed", optional)]
        pub fixed: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Group {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name", optional)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xattribute(name = "ref", optional)]
        pub ref_: ::core::option::Option<crate::types::QName>,
        #[xattribute(name = "minOccurs", optional)]
        pub min_occurs: ::core::option::Option<usize>,
        #[xattribute(name = "maxOccurs", optional)]
        pub max_occurs: ::core::option::Option<::std::boxed::Box<crate::types::AllNNI>>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        #[xvalue(default)]
        #[builder(default)]
        pub particle: ::std::vec::Vec<crate::groups::Particle>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct GroupRef {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "ref")]
        pub ref_: crate::types::QName,
        #[xattribute(name = "minOccurs", optional)]
        pub min_occurs: ::core::option::Option<usize>,
        #[xattribute(name = "maxOccurs", optional)]
        pub max_occurs: ::core::option::Option<::std::boxed::Box<crate::types::AllNNI>>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct IntFacet {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "value")]
        pub value: i32,
        #[xattribute(name = "fixed", optional)]
        pub fixed: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
    }
    pub mod keybase_items {
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::bon::Builder,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(order = "strict")]
        pub struct Child1 {
            pub selector: ::std::boxed::Box<crate::Selector>,
            #[xvalue(default)]
            #[builder(default)]
            pub field: ::std::vec::Vec<crate::Field>,
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Keybase {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name", optional)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xattribute(name = "ref", optional)]
        pub ref_: ::core::option::Option<crate::types::QName>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        #[xvalue(default)]
        pub child_1: ::core::option::Option<keybase_items::Child1>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct LocalComplexType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "mixed", optional)]
        pub mixed: ::core::option::Option<bool>,
        #[xattribute(name = "defaultAttributesApply", optional)]
        pub default_attributes_apply: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        pub complex_type_model: ::std::boxed::Box<crate::groups::ComplexTypeModel>,
    }
    pub mod local_element_items {
        impl ::core::convert::From<crate::types::LocalSimpleType> for Type {
            fn from(value: crate::types::LocalSimpleType) -> Self {
                Type::SimpleType(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::types::LocalComplexType> for Type {
            fn from(value: crate::types::LocalComplexType) -> Self {
                Type::ComplexType(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Type {
            #[xelement(
                name = "simpleType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            SimpleType(#[xgroup] ::std::boxed::Box<crate::types::LocalSimpleType>),
            #[xelement(
                name = "complexType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            ComplexType(#[xgroup] ::std::boxed::Box<crate::types::LocalComplexType>),
        }
        impl ::core::convert::From<crate::types::AltType> for Alternative {
            fn from(value: crate::types::AltType) -> Self {
                Alternative(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xelement(
            name = "alternative",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        pub struct Alternative(#[xgroup] pub ::std::boxed::Box<crate::types::AltType>);
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct LocalElement {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name", optional)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xattribute(name = "ref", optional)]
        pub ref_: ::core::option::Option<crate::types::QName>,
        #[xattribute(name = "type", optional)]
        pub type_attribute: ::core::option::Option<crate::types::QName>,
        #[xattribute(name = "minOccurs", optional)]
        pub min_occurs: ::core::option::Option<usize>,
        #[xattribute(name = "maxOccurs", optional)]
        pub max_occurs: ::core::option::Option<::std::boxed::Box<crate::types::AllNNI>>,
        #[xattribute(name = "default", optional)]
        pub default: ::core::option::Option<String>,
        #[xattribute(name = "fixed", optional)]
        pub fixed: ::core::option::Option<String>,
        #[xattribute(name = "nillable", optional)]
        pub nillable: ::core::option::Option<bool>,
        #[xattribute(name = "block", optional)]
        pub block: ::core::option::Option<::std::boxed::Box<crate::types::BlockSet>>,
        #[xattribute(name = "form", optional)]
        pub form: ::core::option::Option<::std::boxed::Box<crate::types::FormChoice>>,
        #[xattribute(name = "targetNamespace", optional)]
        pub target_namespace: ::core::option::Option<crate::types::TargetNamespace>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        #[xvalue(default)]
        pub type_: ::core::option::Option<local_element_items::Type>,
        #[xvalue(default)]
        #[builder(default)]
        pub alternative: ::std::vec::Vec<local_element_items::Alternative>,
        #[xvalue(default)]
        #[builder(default)]
        pub identity_constraint: ::std::vec::Vec<crate::groups::IdentityConstraint>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct LocalSimpleType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        pub simple_derivation: ::std::boxed::Box<crate::groups::SimpleDerivation>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct NamedAttributeGroup {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name")]
        pub name: ::xmlity::LocalName<'static>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        pub attr_decls: ::std::boxed::Box<crate::groups::AttrDecls>,
    }
    pub mod named_group_items {
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Child1 {
            #[xelement(
                name = "all",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            All {
                #[xattribute(name = "id", optional)]
                id: ::core::option::Option<String>,
                #[xattribute(name = "minOccurs", optional)]
                min_occurs: ::core::option::Option<usize>,
                #[xattribute(name = "maxOccurs", optional)]
                max_occurs: ::core::option::Option<
                    ::std::boxed::Box<crate::types::AllNNI>,
                >,
                all_model: ::std::boxed::Box<crate::groups::AllModel>,
            },
            #[xelement(
                name = "choice",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            Choice(#[xgroup] ::std::boxed::Box<crate::types::SimpleExplicitGroup>),
            #[xelement(
                name = "sequence",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            Sequence(#[xgroup] ::std::boxed::Box<crate::types::SimpleExplicitGroup>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct NamedGroup {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name")]
        pub name: ::xmlity::LocalName<'static>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        pub child_1: named_group_items::Child1,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct NoFixedFacet {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "value")]
        pub value: String,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct NumFacet {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "value")]
        pub value: usize,
        #[xattribute(name = "fixed", optional)]
        pub fixed: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct OpenAttrs;
    pub mod real_group_items {
        impl ::core::convert::From<crate::All> for Child1 {
            fn from(value: crate::All) -> Self {
                Child1::All(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::Choice> for Child1 {
            fn from(value: crate::Choice) -> Self {
                Child1::Choice(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::Sequence> for Child1 {
            fn from(value: crate::Sequence) -> Self {
                Child1::Sequence(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Child1 {
            All(::std::boxed::Box<crate::All>),
            Choice(::std::boxed::Box<crate::Choice>),
            Sequence(::std::boxed::Box<crate::Sequence>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct RealGroup {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name", optional)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xattribute(name = "ref", optional)]
        pub ref_: ::core::option::Option<crate::types::QName>,
        #[xattribute(name = "minOccurs", optional)]
        pub min_occurs: ::core::option::Option<usize>,
        #[xattribute(name = "maxOccurs", optional)]
        pub max_occurs: ::core::option::Option<::std::boxed::Box<crate::types::AllNNI>>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        #[xvalue(default)]
        pub child_1: ::core::option::Option<real_group_items::Child1>,
    }
    pub mod restriction_type_items {
        pub mod child_1_variants {
            #[derive(
                ::core::fmt::Debug,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::bon::Builder,
                ::core::cmp::PartialEq,
                ::core::clone::Clone
            )]
            #[xvalue(order = "strict")]
            pub struct Variant0 {
                #[xvalue(default)]
                pub open_content: ::core::option::Option<
                    ::std::boxed::Box<crate::OpenContent>,
                >,
                pub type_def_particle: ::std::boxed::Box<crate::groups::TypeDefParticle>,
            }
        }
        impl ::core::convert::From<child_1_variants::Variant0> for Child1 {
            fn from(value: child_1_variants::Variant0) -> Self {
                Child1::Variant0(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::groups::SimpleRestrictionModel> for Child1 {
            fn from(value: crate::groups::SimpleRestrictionModel) -> Self {
                Child1::SimpleRestrictionModel(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Child1 {
            Variant0(::std::boxed::Box<child_1_variants::Variant0>),
            SimpleRestrictionModel(
                ::std::boxed::Box<crate::groups::SimpleRestrictionModel>,
            ),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct RestrictionType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "base")]
        pub base: crate::types::QName,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        #[xvalue(default)]
        pub child_1: ::core::option::Option<restriction_type_items::Child1>,
        pub attr_decls: ::std::boxed::Box<crate::groups::AttrDecls>,
        pub assertions: ::std::boxed::Box<crate::groups::Assertions>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct SimpleBaseType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "final", optional)]
        pub final_: ::core::option::Option<
            ::std::boxed::Box<crate::types::SimpleDerivationSet>,
        >,
        #[xattribute(name = "name", optional)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        pub simple_derivation: ::std::boxed::Box<crate::groups::SimpleDerivation>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct SimpleExplicitGroup {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        #[xvalue(default)]
        #[builder(default)]
        pub nested_particle: ::std::vec::Vec<crate::groups::NestedParticle>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct SimpleExtensionType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "base")]
        pub base: crate::types::QName,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        pub attr_decls: ::std::boxed::Box<crate::groups::AttrDecls>,
        pub assertions: ::std::boxed::Box<crate::groups::Assertions>,
    }
    pub mod simple_restriction_type_items {
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::bon::Builder,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xvalue(order = "strict")]
        pub struct SimpleRestrictionModel {
            pub simple_restriction_model: ::std::boxed::Box<
                crate::groups::SimpleRestrictionModel,
            >,
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct SimpleRestrictionType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "base")]
        pub base: crate::types::QName,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        #[xvalue(default)]
        pub simple_restriction_model: ::core::option::Option<
            simple_restriction_type_items::SimpleRestrictionModel,
        >,
        pub attr_decls: ::std::boxed::Box<crate::groups::AttrDecls>,
        pub assertions: ::std::boxed::Box<crate::groups::Assertions>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct TopLevelAttribute {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name")]
        pub name: ::xmlity::LocalName<'static>,
        #[xattribute(name = "type", optional)]
        pub type_: ::core::option::Option<crate::types::QName>,
        #[xattribute(name = "default", optional)]
        pub default: ::core::option::Option<String>,
        #[xattribute(name = "fixed", optional)]
        pub fixed: ::core::option::Option<String>,
        #[xattribute(name = "inheritable", optional)]
        pub inheritable: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        #[xelement(
            name = "simpleType",
            namespace = "http://www.w3.org/2001/XMLSchema",
            group,
            optional
        )]
        pub simple_type: ::core::option::Option<
            ::std::boxed::Box<crate::types::LocalSimpleType>,
        >,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct TopLevelComplexType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name")]
        pub name: ::xmlity::LocalName<'static>,
        #[xattribute(name = "mixed", optional)]
        pub mixed: ::core::option::Option<bool>,
        #[xattribute(name = "abstract", optional)]
        pub abstract_: ::core::option::Option<bool>,
        #[xattribute(name = "final", optional)]
        pub final_: ::core::option::Option<
            ::std::boxed::Box<crate::types::DerivationSet>,
        >,
        #[xattribute(name = "block", optional)]
        pub block: ::core::option::Option<
            ::std::boxed::Box<crate::types::DerivationSet>,
        >,
        #[xattribute(name = "defaultAttributesApply", optional)]
        pub default_attributes_apply: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        pub complex_type_model: ::std::boxed::Box<crate::groups::ComplexTypeModel>,
    }
    pub mod top_level_element_items {
        impl ::core::convert::From<crate::types::LocalSimpleType> for Type {
            fn from(value: crate::types::LocalSimpleType) -> Self {
                Type::SimpleType(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::types::LocalComplexType> for Type {
            fn from(value: crate::types::LocalComplexType) -> Self {
                Type::ComplexType(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Type {
            #[xelement(
                name = "simpleType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            SimpleType(#[xgroup] ::std::boxed::Box<crate::types::LocalSimpleType>),
            #[xelement(
                name = "complexType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            ComplexType(#[xgroup] ::std::boxed::Box<crate::types::LocalComplexType>),
        }
        impl ::core::convert::From<crate::types::AltType> for Alternative {
            fn from(value: crate::types::AltType) -> Self {
                Alternative(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xelement(
            name = "alternative",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        pub struct Alternative(#[xgroup] pub ::std::boxed::Box<crate::types::AltType>);
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct TopLevelElement {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name")]
        pub name: ::xmlity::LocalName<'static>,
        #[xattribute(name = "type", optional)]
        pub type_attribute: ::core::option::Option<crate::types::QName>,
        #[xattribute(name = "substitutionGroup", optional)]
        pub substitution_group: ::core::option::Option<
            ::xmlity_ns::List<crate::types::QName>,
        >,
        #[xattribute(name = "default", optional)]
        pub default: ::core::option::Option<String>,
        #[xattribute(name = "fixed", optional)]
        pub fixed: ::core::option::Option<String>,
        #[xattribute(name = "nillable", optional)]
        pub nillable: ::core::option::Option<bool>,
        #[xattribute(name = "abstract", optional)]
        pub abstract_: ::core::option::Option<bool>,
        #[xattribute(name = "final", optional)]
        pub final_: ::core::option::Option<
            ::std::boxed::Box<crate::types::DerivationSet>,
        >,
        #[xattribute(name = "block", optional)]
        pub block: ::core::option::Option<::std::boxed::Box<crate::types::BlockSet>>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        #[xvalue(default)]
        pub type_: ::core::option::Option<top_level_element_items::Type>,
        #[xvalue(default)]
        #[builder(default)]
        pub alternative: ::std::vec::Vec<top_level_element_items::Alternative>,
        #[xvalue(default)]
        #[builder(default)]
        pub identity_constraint: ::std::vec::Vec<crate::groups::IdentityConstraint>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct TopLevelSimpleType {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "final", optional)]
        pub final_: ::core::option::Option<
            ::std::boxed::Box<crate::types::SimpleDerivationSet>,
        >,
        #[xattribute(name = "name")]
        pub name: ::xmlity::LocalName<'static>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        pub simple_derivation: ::std::boxed::Box<crate::groups::SimpleDerivation>,
    }
    pub mod wildcard_items {
        pub mod not_namespace_value_variants {
            #[derive(
                ::core::fmt::Debug,
                ::core::clone::Clone,
                ::core::marker::Copy,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::core::cmp::PartialEq
            )]
            #[xvalue(with = variant_0_with)]
            pub enum Variant0 {
                TargetNamespace,
                Local,
            }
            pub mod variant_0_with {
                pub fn deserialize<'de, D>(
                    deserializer: D,
                ) -> ::core::result::Result<super::Variant0, D::Error>
                where
                    D: ::xmlity::Deserializer<'de>,
                {
                    let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                        deserializer,
                    )?;
                    let value: ::std::string::String = text
                        .parse()
                        .map_err(::xmlity::de::Error::custom)?;
                    super::Variant0::try_from(value).map_err(::xmlity::de::Error::custom)
                }
                pub fn serialize<S>(
                    value: &super::Variant0,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::xmlity::Serializer,
                {
                    let value: ::std::string::String = ::core::clone::Clone::clone(value)
                        .into();
                    ::xmlity::Serialize::serialize(
                        ::std::string::String::as_str(
                            &::std::string::ToString::to_string(&value),
                        ),
                        serializer,
                    )
                }
            }
            #[derive(::core::fmt::Debug)]
            pub enum Variant0ParseError {
                NonExistent { value: ::std::string::String },
            }
            impl ::core::fmt::Display for Variant0ParseError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::result::Result<(), ::core::fmt::Error> {
                    match self {
                        Variant0ParseError::NonExistent { value } => {
                            write!(
                                f, "Value '{:?}' does not exist in the enumeration", value
                            )
                        }
                    }
                }
            }
            impl ::core::convert::TryFrom<::std::string::String> for Variant0 {
                type Error = Variant0ParseError;
                fn try_from(
                    value: ::std::string::String,
                ) -> ::core::result::Result<Self, Self::Error> {
                    match ::std::string::String::as_str(&value) {
                        "##targetNamespace" => Ok(Variant0::TargetNamespace),
                        "##local" => Ok(Variant0::Local),
                        _ => {
                            Err(Variant0ParseError::NonExistent {
                                value,
                            })
                        }
                    }
                }
            }
            impl ::core::convert::From<Variant0> for ::std::string::String {
                fn from(value: Variant0) -> Self {
                    match value {
                        Variant0::TargetNamespace => {
                            ::std::string::String::from("##targetNamespace")
                        }
                        Variant0::Local => ::std::string::String::from("##local"),
                    }
                }
            }
        }
        impl ::core::convert::From<crate::types::TargetNamespace> for NotNamespaceValue {
            fn from(value: crate::types::TargetNamespace) -> Self {
                NotNamespaceValue::AnyURI(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<not_namespace_value_variants::Variant0>
        for NotNamespaceValue {
            fn from(value: not_namespace_value_variants::Variant0) -> Self {
                NotNamespaceValue::Variant0(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum NotNamespaceValue {
            AnyURI(::std::boxed::Box<crate::types::TargetNamespace>),
            Variant0(::std::boxed::Box<not_namespace_value_variants::Variant0>),
        }
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq
        )]
        #[xvalue(with = process_contents_value_with)]
        pub enum ProcessContentsValue {
            Skip,
            Lax,
            Strict,
        }
        pub mod process_contents_value_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::ProcessContentsValue, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::ProcessContentsValue::try_from(value)
                    .map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::ProcessContentsValue,
                serializer: S,
            ) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::xmlity::Serializer,
            {
                let value: ::std::string::String = ::core::clone::Clone::clone(value)
                    .into();
                ::xmlity::Serialize::serialize(
                    ::std::string::String::as_str(
                        &::std::string::ToString::to_string(&value),
                    ),
                    serializer,
                )
            }
        }
        #[derive(::core::fmt::Debug)]
        pub enum ProcessContentsValueParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for ProcessContentsValueParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    ProcessContentsValueParseError::NonExistent { value } => {
                        write!(
                            f, "Value '{:?}' does not exist in the enumeration", value
                        )
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for ProcessContentsValue {
            type Error = ProcessContentsValueParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "skip" => Ok(ProcessContentsValue::Skip),
                    "lax" => Ok(ProcessContentsValue::Lax),
                    "strict" => Ok(ProcessContentsValue::Strict),
                    _ => {
                        Err(ProcessContentsValueParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<ProcessContentsValue> for ::std::string::String {
            fn from(value: ProcessContentsValue) -> Self {
                match value {
                    ProcessContentsValue::Skip => ::std::string::String::from("skip"),
                    ProcessContentsValue::Lax => ::std::string::String::from("lax"),
                    ProcessContentsValue::Strict => ::std::string::String::from("strict"),
                }
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Wildcard {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "namespace", optional)]
        pub namespace: ::core::option::Option<
            ::std::boxed::Box<crate::types::NamespaceList>,
        >,
        #[xattribute(name = "notNamespace", optional)]
        pub not_namespace: ::core::option::Option<
            ::xmlity_ns::List<wildcard_items::NotNamespaceValue>,
        >,
        #[xattribute(name = "processContents", optional)]
        pub process_contents: ::core::option::Option<
            wildcard_items::ProcessContentsValue,
        >,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
    }
}
pub mod groups {
    pub mod all_model_items {
        impl ::core::convert::From<crate::types::LocalElement> for Child1 {
            fn from(value: crate::types::LocalElement) -> Self {
                Child1::Element(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::Any> for Child1 {
            fn from(value: crate::Any) -> Self {
                Child1::Any(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Child1 {
            #[xelement(
                name = "element",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            Element(#[xgroup] ::std::boxed::Box<crate::types::LocalElement>),
            Any(::std::boxed::Box<crate::Any>),
            #[xelement(
                name = "group",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any",
                children_order = "strict"
            )]
            Group {
                #[xattribute(name = "id", optional)]
                id: ::core::option::Option<String>,
                #[xattribute(name = "ref")]
                ref_: crate::types::QName,
                #[xattribute(name = "minOccurs", optional)]
                min_occurs: ::core::option::Option<usize>,
                #[xattribute(name = "maxOccurs", optional)]
                max_occurs: ::core::option::Option<usize>,
                #[xvalue(default)]
                annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
            },
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct AllModel {
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::Annotation>>,
        #[xvalue(default)]
        #[builder(default)]
        pub child_1: ::std::vec::Vec<all_model_items::Child1>,
    }
    pub mod assertions_items {
        impl ::core::convert::From<crate::types::Assertion> for Assert {
            fn from(value: crate::types::Assertion) -> Self {
                Assert(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        #[xelement(
            name = "assert",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        pub struct Assert(#[xgroup] pub ::std::boxed::Box<crate::types::Assertion>);
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct Assertions {
        #[xvalue(default)]
        #[builder(default)]
        pub assert: ::std::vec::Vec<assertions_items::Assert>,
    }
    pub mod attr_decls_items {
        impl ::core::convert::From<crate::types::Attribute> for Attribute {
            fn from(value: crate::types::Attribute) -> Self {
                Attribute::Attribute(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::types::AttributeGroupRef> for Attribute {
            fn from(value: crate::types::AttributeGroupRef) -> Self {
                Attribute::AttributeGroup(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Attribute {
            #[xelement(
                name = "attribute",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            Attribute(#[xgroup] ::std::boxed::Box<crate::types::Attribute>),
            #[xelement(
                name = "attributeGroup",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            AttributeGroup(#[xgroup] ::std::boxed::Box<crate::types::AttributeGroupRef>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct AttrDecls {
        #[xvalue(default)]
        #[builder(default)]
        pub attribute: ::std::vec::Vec<attr_decls_items::Attribute>,
        #[xvalue(default)]
        pub any_attribute: ::core::option::Option<
            ::std::boxed::Box<crate::AnyAttribute>,
        >,
    }
    pub mod complex_type_model_items {
        pub mod complex_type_model_variants {
            #[derive(
                ::core::fmt::Debug,
                ::xmlity::Serialize,
                ::xmlity::Deserialize,
                ::bon::Builder,
                ::core::cmp::PartialEq,
                ::core::clone::Clone
            )]
            #[xvalue(order = "strict")]
            pub struct Variant2 {
                #[xvalue(default)]
                pub open_content: ::core::option::Option<
                    ::std::boxed::Box<crate::OpenContent>,
                >,
                #[xvalue(default)]
                pub type_def_particle: ::core::option::Option<
                    ::std::boxed::Box<crate::groups::TypeDefParticle>,
                >,
                pub attr_decls: crate::groups::AttrDecls,
                pub assertions: crate::groups::Assertions,
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum ComplexTypeModel {
        SimpleContent(::std::boxed::Box<crate::SimpleContent>),
        ComplexContent(::std::boxed::Box<crate::ComplexContent>),
        Variant2(
            ::std::boxed::Box<
                complex_type_model_items::complex_type_model_variants::Variant2,
            >,
        ),
    }
    impl ::core::convert::From<crate::SimpleContent> for ComplexTypeModel {
        fn from(value: crate::SimpleContent) -> Self {
            ComplexTypeModel::SimpleContent(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::ComplexContent> for ComplexTypeModel {
        fn from(value: crate::ComplexContent) -> Self {
            ComplexTypeModel::ComplexContent(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<
        complex_type_model_items::complex_type_model_variants::Variant2,
    > for ComplexTypeModel {
        fn from(
            value: complex_type_model_items::complex_type_model_variants::Variant2,
        ) -> Self {
            ComplexTypeModel::Variant2(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Composition {
        Include(::std::boxed::Box<crate::Include>),
        Import(::std::boxed::Box<crate::Import>),
        Redefine(::std::boxed::Box<crate::Redefine>),
        Override(::std::boxed::Box<crate::Override>),
        Annotation(::std::boxed::Box<crate::Annotation>),
    }
    impl ::core::convert::From<crate::Include> for Composition {
        fn from(value: crate::Include) -> Self {
            Composition::Include(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Import> for Composition {
        fn from(value: crate::Import) -> Self {
            Composition::Import(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Redefine> for Composition {
        fn from(value: crate::Redefine) -> Self {
            Composition::Redefine(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Override> for Composition {
        fn from(value: crate::Override) -> Self {
            Composition::Override(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Annotation> for Composition {
        fn from(value: crate::Annotation) -> Self {
            Composition::Annotation(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum IdentityConstraint {
        Unique(::std::boxed::Box<crate::Unique>),
        Key(::std::boxed::Box<crate::Key>),
        Keyref(::std::boxed::Box<crate::Keyref>),
    }
    impl ::core::convert::From<crate::Unique> for IdentityConstraint {
        fn from(value: crate::Unique) -> Self {
            IdentityConstraint::Unique(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Key> for IdentityConstraint {
        fn from(value: crate::Key) -> Self {
            IdentityConstraint::Key(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Keyref> for IdentityConstraint {
        fn from(value: crate::Keyref) -> Self {
            IdentityConstraint::Keyref(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum NestedParticle {
        #[xelement(
            name = "element",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Element(#[xgroup] ::std::boxed::Box<crate::types::LocalElement>),
        #[xelement(
            name = "group",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Group(#[xgroup] ::std::boxed::Box<crate::types::GroupRef>),
        Choice(::std::boxed::Box<crate::Choice>),
        Sequence(::std::boxed::Box<crate::Sequence>),
        Any(::std::boxed::Box<crate::Any>),
    }
    impl ::core::convert::From<crate::types::LocalElement> for NestedParticle {
        fn from(value: crate::types::LocalElement) -> Self {
            NestedParticle::Element(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::types::GroupRef> for NestedParticle {
        fn from(value: crate::types::GroupRef) -> Self {
            NestedParticle::Group(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Choice> for NestedParticle {
        fn from(value: crate::Choice) -> Self {
            NestedParticle::Choice(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Sequence> for NestedParticle {
        fn from(value: crate::Sequence) -> Self {
            NestedParticle::Sequence(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Any> for NestedParticle {
        fn from(value: crate::Any) -> Self {
            NestedParticle::Any(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Particle {
        #[xelement(
            name = "element",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Element(#[xgroup] ::std::boxed::Box<crate::types::LocalElement>),
        #[xelement(
            name = "group",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Group(#[xgroup] ::std::boxed::Box<crate::types::GroupRef>),
        All(::std::boxed::Box<crate::All>),
        Choice(::std::boxed::Box<crate::Choice>),
        Sequence(::std::boxed::Box<crate::Sequence>),
        Any(::std::boxed::Box<crate::Any>),
    }
    impl ::core::convert::From<crate::types::LocalElement> for Particle {
        fn from(value: crate::types::LocalElement) -> Self {
            Particle::Element(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::types::GroupRef> for Particle {
        fn from(value: crate::types::GroupRef) -> Self {
            Particle::Group(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::All> for Particle {
        fn from(value: crate::All) -> Self {
            Particle::All(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Choice> for Particle {
        fn from(value: crate::Choice) -> Self {
            Particle::Choice(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Sequence> for Particle {
        fn from(value: crate::Sequence) -> Self {
            Particle::Sequence(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Any> for Particle {
        fn from(value: crate::Any) -> Self {
            Particle::Any(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Redefinable {
        SimpleType(::std::boxed::Box<crate::SimpleType>),
        ComplexType(::std::boxed::Box<crate::ComplexType>),
        Group(::std::boxed::Box<crate::Group>),
        AttributeGroup(::std::boxed::Box<crate::AttributeGroup>),
    }
    impl ::core::convert::From<crate::SimpleType> for Redefinable {
        fn from(value: crate::SimpleType) -> Self {
            Redefinable::SimpleType(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::ComplexType> for Redefinable {
        fn from(value: crate::ComplexType) -> Self {
            Redefinable::ComplexType(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Group> for Redefinable {
        fn from(value: crate::Group) -> Self {
            Redefinable::Group(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::AttributeGroup> for Redefinable {
        fn from(value: crate::AttributeGroup) -> Self {
            Redefinable::AttributeGroup(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum SchemaTop {
        Redefinable(::std::boxed::Box<crate::groups::Redefinable>),
        Element(::std::boxed::Box<crate::Element>),
        Attribute(::std::boxed::Box<crate::Attribute>),
        Notation(::std::boxed::Box<crate::Notation>),
    }
    impl ::core::convert::From<crate::groups::Redefinable> for SchemaTop {
        fn from(value: crate::groups::Redefinable) -> Self {
            SchemaTop::Redefinable(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Element> for SchemaTop {
        fn from(value: crate::Element) -> Self {
            SchemaTop::Element(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Attribute> for SchemaTop {
        fn from(value: crate::Attribute) -> Self {
            SchemaTop::Attribute(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Notation> for SchemaTop {
        fn from(value: crate::Notation) -> Self {
            SchemaTop::Notation(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum SimpleDerivation {
        Restriction(::std::boxed::Box<crate::Restriction>),
        List(::std::boxed::Box<crate::List>),
        Union(::std::boxed::Box<crate::Union>),
    }
    impl ::core::convert::From<crate::Restriction> for SimpleDerivation {
        fn from(value: crate::Restriction) -> Self {
            SimpleDerivation::Restriction(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::List> for SimpleDerivation {
        fn from(value: crate::List) -> Self {
            SimpleDerivation::List(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Union> for SimpleDerivation {
        fn from(value: crate::Union) -> Self {
            SimpleDerivation::Union(::std::boxed::Box::new(value))
        }
    }
    pub mod simple_restriction_model_items {
        impl ::core::convert::From<crate::Facet> for Child1 {
            fn from(value: crate::Facet) -> Self {
                Child1::Facet(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<::xmlity::XmlValue> for Child1 {
            fn from(value: ::xmlity::XmlValue) -> Self {
                Child1::Variant1(::std::boxed::Box::new(value))
            }
        }
        #[derive(
            ::core::fmt::Debug,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq,
            ::core::clone::Clone
        )]
        pub enum Child1 {
            Facet(::std::boxed::Box<crate::Facet>),
            Variant1(::std::boxed::Box<::xmlity::XmlValue>),
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct SimpleRestrictionModel {
        #[xelement(
            name = "simpleType",
            namespace = "http://www.w3.org/2001/XMLSchema",
            group,
            optional
        )]
        pub simple_type: ::core::option::Option<
            ::std::boxed::Box<crate::types::LocalSimpleType>,
        >,
        #[xvalue(default)]
        #[builder(default)]
        pub child_1: ::std::vec::Vec<simple_restriction_model_items::Child1>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum TypeDefParticle {
        #[xelement(
            name = "group",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Group(#[xgroup] ::std::boxed::Box<crate::types::GroupRef>),
        All(::std::boxed::Box<crate::All>),
        Choice(::std::boxed::Box<crate::Choice>),
        Sequence(::std::boxed::Box<crate::Sequence>),
    }
    impl ::core::convert::From<crate::types::GroupRef> for TypeDefParticle {
        fn from(value: crate::types::GroupRef) -> Self {
            TypeDefParticle::Group(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::All> for TypeDefParticle {
        fn from(value: crate::All) -> Self {
            TypeDefParticle::All(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Choice> for TypeDefParticle {
        fn from(value: crate::Choice) -> Self {
            TypeDefParticle::Choice(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Sequence> for TypeDefParticle {
        fn from(value: crate::Sequence) -> Self {
            TypeDefParticle::Sequence(::std::boxed::Box::new(value))
        }
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum All {
    #[xelement(
        name = "all",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    All(#[xgroup] ::std::boxed::Box<crate::types::All>),
    SubstitutionGroup(::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::All>>),
}
impl ::core::convert::From<crate::types::All> for All {
    fn from(value: crate::types::All) -> Self {
        All::All(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::All>>>
for All {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::All>>,
    ) -> Self {
        All::SubstitutionGroup(value)
    }
}
pub mod annotation_items {
    impl ::core::convert::From<crate::Appinfo> for AnnotationContent {
        fn from(value: crate::Appinfo) -> Self {
            AnnotationContent::Appinfo(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::Documentation> for AnnotationContent {
        fn from(value: crate::Documentation) -> Self {
            AnnotationContent::Documentation(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum AnnotationContent {
        Appinfo(::std::boxed::Box<crate::Appinfo>),
        Documentation(::std::boxed::Box<crate::Documentation>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct Annotation {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xvalue(default)]
        #[builder(default)]
        pub annotation_content: ::std::vec::Vec<AnnotationContent>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Annotation {
    #[xelement(
        name = "annotation",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Annotation(#[xgroup] annotation_items::Annotation),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Annotation>>,
    ),
}
impl ::core::convert::From<annotation_items::Annotation> for Annotation {
    fn from(value: annotation_items::Annotation) -> Self {
        Annotation::Annotation(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Annotation>>,
> for Annotation {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Annotation>>,
    ) -> Self {
        Annotation::SubstitutionGroup(value)
    }
}
pub mod any_items {
    pub mod not_namespace_value_variants {
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq
        )]
        #[xvalue(with = variant_0_with)]
        pub enum Variant0 {
            TargetNamespace,
            Local,
        }
        pub mod variant_0_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Variant0, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::Variant0::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Variant0,
                serializer: S,
            ) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::xmlity::Serializer,
            {
                let value: ::std::string::String = ::core::clone::Clone::clone(value)
                    .into();
                ::xmlity::Serialize::serialize(
                    ::std::string::String::as_str(
                        &::std::string::ToString::to_string(&value),
                    ),
                    serializer,
                )
            }
        }
        #[derive(::core::fmt::Debug)]
        pub enum Variant0ParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for Variant0ParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    Variant0ParseError::NonExistent { value } => {
                        write!(
                            f, "Value '{:?}' does not exist in the enumeration", value
                        )
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for Variant0 {
            type Error = Variant0ParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "##targetNamespace" => Ok(Variant0::TargetNamespace),
                    "##local" => Ok(Variant0::Local),
                    _ => {
                        Err(Variant0ParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<Variant0> for ::std::string::String {
            fn from(value: Variant0) -> Self {
                match value {
                    Variant0::TargetNamespace => {
                        ::std::string::String::from("##targetNamespace")
                    }
                    Variant0::Local => ::std::string::String::from("##local"),
                }
            }
        }
    }
    impl ::core::convert::From<crate::types::TargetNamespace> for NotNamespaceValue {
        fn from(value: crate::types::TargetNamespace) -> Self {
            NotNamespaceValue::AnyURI(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<not_namespace_value_variants::Variant0>
    for NotNamespaceValue {
        fn from(value: not_namespace_value_variants::Variant0) -> Self {
            NotNamespaceValue::Variant0(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum NotNamespaceValue {
        AnyURI(::std::boxed::Box<crate::types::TargetNamespace>),
        Variant0(::std::boxed::Box<not_namespace_value_variants::Variant0>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = process_contents_value_with)]
    pub enum ProcessContentsValue {
        Skip,
        Lax,
        Strict,
    }
    pub mod process_contents_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::ProcessContentsValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::ProcessContentsValue::try_from(value)
                .map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::ProcessContentsValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum ProcessContentsValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for ProcessContentsValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                ProcessContentsValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for ProcessContentsValue {
        type Error = ProcessContentsValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "skip" => Ok(ProcessContentsValue::Skip),
                "lax" => Ok(ProcessContentsValue::Lax),
                "strict" => Ok(ProcessContentsValue::Strict),
                _ => {
                    Err(ProcessContentsValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<ProcessContentsValue> for ::std::string::String {
        fn from(value: ProcessContentsValue) -> Self {
            match value {
                ProcessContentsValue::Skip => ::std::string::String::from("skip"),
                ProcessContentsValue::Lax => ::std::string::String::from("lax"),
                ProcessContentsValue::Strict => ::std::string::String::from("strict"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Any {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "namespace", optional)]
        pub namespace: ::core::option::Option<
            ::std::boxed::Box<crate::types::NamespaceList>,
        >,
        #[xattribute(name = "notNamespace", optional)]
        pub not_namespace: ::core::option::Option<::xmlity_ns::List<NotNamespaceValue>>,
        #[xattribute(name = "processContents", optional)]
        pub process_contents: ::core::option::Option<ProcessContentsValue>,
        #[xattribute(name = "notQName", optional)]
        pub not_q_name: ::core::option::Option<
            ::std::boxed::Box<crate::types::QnameList>,
        >,
        #[xattribute(name = "minOccurs", optional)]
        pub min_occurs: ::core::option::Option<usize>,
        #[xattribute(name = "maxOccurs", optional)]
        pub max_occurs: ::core::option::Option<::std::boxed::Box<crate::types::AllNNI>>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<crate::Annotation>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Any {
    #[xelement(
        name = "any",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Any(#[xgroup] any_items::Any),
    SubstitutionGroup(::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Any>>),
}
impl ::core::convert::From<any_items::Any> for Any {
    fn from(value: any_items::Any) -> Self {
        Any::Any(value)
    }
}
impl ::core::convert::From<::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Any>>>
for Any {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Any>>,
    ) -> Self {
        Any::SubstitutionGroup(value)
    }
}
pub mod any_attribute_items {
    pub mod not_namespace_value_variants {
        #[derive(
            ::core::fmt::Debug,
            ::core::clone::Clone,
            ::core::marker::Copy,
            ::xmlity::Serialize,
            ::xmlity::Deserialize,
            ::core::cmp::PartialEq
        )]
        #[xvalue(with = variant_0_with)]
        pub enum Variant0 {
            TargetNamespace,
            Local,
        }
        pub mod variant_0_with {
            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> ::core::result::Result<super::Variant0, D::Error>
            where
                D: ::xmlity::Deserializer<'de>,
            {
                let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                    deserializer,
                )?;
                let value: ::std::string::String = text
                    .parse()
                    .map_err(::xmlity::de::Error::custom)?;
                super::Variant0::try_from(value).map_err(::xmlity::de::Error::custom)
            }
            pub fn serialize<S>(
                value: &super::Variant0,
                serializer: S,
            ) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::xmlity::Serializer,
            {
                let value: ::std::string::String = ::core::clone::Clone::clone(value)
                    .into();
                ::xmlity::Serialize::serialize(
                    ::std::string::String::as_str(
                        &::std::string::ToString::to_string(&value),
                    ),
                    serializer,
                )
            }
        }
        #[derive(::core::fmt::Debug)]
        pub enum Variant0ParseError {
            NonExistent { value: ::std::string::String },
        }
        impl ::core::fmt::Display for Variant0ParseError {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                match self {
                    Variant0ParseError::NonExistent { value } => {
                        write!(
                            f, "Value '{:?}' does not exist in the enumeration", value
                        )
                    }
                }
            }
        }
        impl ::core::convert::TryFrom<::std::string::String> for Variant0 {
            type Error = Variant0ParseError;
            fn try_from(
                value: ::std::string::String,
            ) -> ::core::result::Result<Self, Self::Error> {
                match ::std::string::String::as_str(&value) {
                    "##targetNamespace" => Ok(Variant0::TargetNamespace),
                    "##local" => Ok(Variant0::Local),
                    _ => {
                        Err(Variant0ParseError::NonExistent {
                            value,
                        })
                    }
                }
            }
        }
        impl ::core::convert::From<Variant0> for ::std::string::String {
            fn from(value: Variant0) -> Self {
                match value {
                    Variant0::TargetNamespace => {
                        ::std::string::String::from("##targetNamespace")
                    }
                    Variant0::Local => ::std::string::String::from("##local"),
                }
            }
        }
    }
    impl ::core::convert::From<crate::types::TargetNamespace> for NotNamespaceValue {
        fn from(value: crate::types::TargetNamespace) -> Self {
            NotNamespaceValue::AnyURI(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<not_namespace_value_variants::Variant0>
    for NotNamespaceValue {
        fn from(value: not_namespace_value_variants::Variant0) -> Self {
            NotNamespaceValue::Variant0(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum NotNamespaceValue {
        AnyURI(::std::boxed::Box<crate::types::TargetNamespace>),
        Variant0(::std::boxed::Box<not_namespace_value_variants::Variant0>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = process_contents_value_with)]
    pub enum ProcessContentsValue {
        Skip,
        Lax,
        Strict,
    }
    pub mod process_contents_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::ProcessContentsValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::ProcessContentsValue::try_from(value)
                .map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::ProcessContentsValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum ProcessContentsValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for ProcessContentsValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                ProcessContentsValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for ProcessContentsValue {
        type Error = ProcessContentsValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "skip" => Ok(ProcessContentsValue::Skip),
                "lax" => Ok(ProcessContentsValue::Lax),
                "strict" => Ok(ProcessContentsValue::Strict),
                _ => {
                    Err(ProcessContentsValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<ProcessContentsValue> for ::std::string::String {
        fn from(value: ProcessContentsValue) -> Self {
            match value {
                ProcessContentsValue::Skip => ::std::string::String::from("skip"),
                ProcessContentsValue::Lax => ::std::string::String::from("lax"),
                ProcessContentsValue::Strict => ::std::string::String::from("strict"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct AnyAttribute {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "namespace", optional)]
        pub namespace: ::core::option::Option<
            ::std::boxed::Box<crate::types::NamespaceList>,
        >,
        #[xattribute(name = "notNamespace", optional)]
        pub not_namespace: ::core::option::Option<::xmlity_ns::List<NotNamespaceValue>>,
        #[xattribute(name = "processContents", optional)]
        pub process_contents: ::core::option::Option<ProcessContentsValue>,
        #[xattribute(name = "notQName", optional)]
        pub not_q_name: ::core::option::Option<
            ::std::boxed::Box<crate::types::QnameListA>,
        >,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum AnyAttribute {
    #[xelement(
        name = "anyAttribute",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    AnyAttribute(#[xgroup] any_attribute_items::AnyAttribute),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::AnyAttribute>>,
    ),
}
impl ::core::convert::From<any_attribute_items::AnyAttribute> for AnyAttribute {
    fn from(value: any_attribute_items::AnyAttribute) -> Self {
        AnyAttribute::AnyAttribute(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::AnyAttribute>>,
> for AnyAttribute {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::AnyAttribute>>,
    ) -> Self {
        AnyAttribute::SubstitutionGroup(value)
    }
}
pub mod appinfo_items {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct Child0 {
        pub child_0: ::xmlity::XmlValue,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct Appinfo {
        #[xattribute(name = "source", optional)]
        pub source: ::core::option::Option<crate::types::TargetNamespace>,
        #[xvalue(default)]
        #[builder(default)]
        pub child_0: ::std::vec::Vec<Child0>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Appinfo {
    #[xelement(
        name = "appinfo",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Appinfo(#[xgroup] appinfo_items::Appinfo),
    SubstitutionGroup(::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Appinfo>>),
}
impl ::core::convert::From<appinfo_items::Appinfo> for Appinfo {
    fn from(value: appinfo_items::Appinfo) -> Self {
        Appinfo::Appinfo(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Appinfo>>,
> for Appinfo {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Appinfo>>,
    ) -> Self {
        Appinfo::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Assertion {
    #[xelement(
        name = "assertion",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Assertion(#[xgroup] ::std::boxed::Box<crate::types::Assertion>),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Assertion>>,
    ),
}
impl ::core::convert::From<crate::types::Assertion> for Assertion {
    fn from(value: crate::types::Assertion) -> Self {
        Assertion::Assertion(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Assertion>>,
> for Assertion {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Assertion>>,
    ) -> Self {
        Assertion::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Attribute {
    #[xelement(
        name = "attribute",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Attribute(#[xgroup] ::std::boxed::Box<crate::types::TopLevelAttribute>),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Attribute>>,
    ),
}
impl ::core::convert::From<crate::types::TopLevelAttribute> for Attribute {
    fn from(value: crate::types::TopLevelAttribute) -> Self {
        Attribute::Attribute(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Attribute>>,
> for Attribute {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Attribute>>,
    ) -> Self {
        Attribute::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum AttributeGroup {
    #[xelement(
        name = "attributeGroup",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    AttributeGroup(#[xgroup] ::std::boxed::Box<crate::types::NamedAttributeGroup>),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::AttributeGroup>>,
    ),
}
impl ::core::convert::From<crate::types::NamedAttributeGroup> for AttributeGroup {
    fn from(value: crate::types::NamedAttributeGroup) -> Self {
        AttributeGroup::AttributeGroup(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::AttributeGroup>>,
> for AttributeGroup {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::AttributeGroup>>,
    ) -> Self {
        AttributeGroup::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Choice {
    #[xelement(
        name = "choice",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Choice(#[xgroup] ::std::boxed::Box<crate::types::ExplicitGroup>),
    SubstitutionGroup(::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Choice>>),
}
impl ::core::convert::From<crate::types::ExplicitGroup> for Choice {
    fn from(value: crate::types::ExplicitGroup) -> Self {
        Choice::Choice(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Choice>>,
> for Choice {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Choice>>,
    ) -> Self {
        Choice::SubstitutionGroup(value)
    }
}
pub mod complex_content_items {
    impl ::core::convert::From<crate::types::ComplexRestrictionType> for Child1 {
        fn from(value: crate::types::ComplexRestrictionType) -> Self {
            Child1::Restriction(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::types::ExtensionType> for Child1 {
        fn from(value: crate::types::ExtensionType) -> Self {
            Child1::Extension(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Child1 {
        #[xelement(
            name = "restriction",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Restriction(#[xgroup] ::std::boxed::Box<crate::types::ComplexRestrictionType>),
        #[xelement(
            name = "extension",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Extension(#[xgroup] ::std::boxed::Box<crate::types::ExtensionType>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct ComplexContent {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "mixed", optional)]
        pub mixed: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<crate::Annotation>,
        pub child_1: Child1,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum ComplexContent {
    #[xelement(
        name = "complexContent",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    ComplexContent(#[xgroup] complex_content_items::ComplexContent),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::ComplexContent>>,
    ),
}
impl ::core::convert::From<complex_content_items::ComplexContent> for ComplexContent {
    fn from(value: complex_content_items::ComplexContent) -> Self {
        ComplexContent::ComplexContent(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::ComplexContent>>,
> for ComplexContent {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::ComplexContent>>,
    ) -> Self {
        ComplexContent::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum ComplexType {
    #[xelement(
        name = "complexType",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    ComplexType(#[xgroup] ::std::boxed::Box<crate::types::TopLevelComplexType>),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::ComplexType>>,
    ),
}
impl ::core::convert::From<crate::types::TopLevelComplexType> for ComplexType {
    fn from(value: crate::types::TopLevelComplexType) -> Self {
        ComplexType::ComplexType(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::ComplexType>>,
> for ComplexType {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::ComplexType>>,
    ) -> Self {
        ComplexType::SubstitutionGroup(value)
    }
}
pub mod default_open_content_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = mode_value_with)]
    pub enum ModeValue {
        Interleave,
        Suffix,
    }
    pub mod mode_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::ModeValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::ModeValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::ModeValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum ModeValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for ModeValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                ModeValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for ModeValue {
        type Error = ModeValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "interleave" => Ok(ModeValue::Interleave),
                "suffix" => Ok(ModeValue::Suffix),
                _ => {
                    Err(ModeValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<ModeValue> for ::std::string::String {
        fn from(value: ModeValue) -> Self {
            match value {
                ModeValue::Interleave => ::std::string::String::from("interleave"),
                ModeValue::Suffix => ::std::string::String::from("suffix"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct DefaultOpenContent {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "appliesToEmpty", optional)]
        pub applies_to_empty: ::core::option::Option<bool>,
        #[xattribute(name = "mode", optional)]
        pub mode: ::core::option::Option<ModeValue>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<crate::Annotation>,
        #[xelement(name = "any", namespace = "http://www.w3.org/2001/XMLSchema", group)]
        pub any: ::std::boxed::Box<crate::types::Wildcard>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum DefaultOpenContent {
    #[xelement(
        name = "defaultOpenContent",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    DefaultOpenContent(#[xgroup] default_open_content_items::DefaultOpenContent),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::DefaultOpenContent>>,
    ),
}
impl ::core::convert::From<default_open_content_items::DefaultOpenContent>
for DefaultOpenContent {
    fn from(value: default_open_content_items::DefaultOpenContent) -> Self {
        DefaultOpenContent::DefaultOpenContent(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::DefaultOpenContent>>,
> for DefaultOpenContent {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<
            ::std::boxed::Box<crate::DefaultOpenContent>,
        >,
    ) -> Self {
        DefaultOpenContent::SubstitutionGroup(value)
    }
}
pub mod documentation_items {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct Child0 {
        pub child_0: ::xmlity::XmlValue,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct Documentation {
        #[xattribute(name = "source", optional)]
        pub source: ::core::option::Option<crate::types::TargetNamespace>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xvalue(default)]
        #[builder(default)]
        pub child_0: ::std::vec::Vec<Child0>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Documentation {
    #[xelement(
        name = "documentation",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Documentation(#[xgroup] documentation_items::Documentation),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Documentation>>,
    ),
}
impl ::core::convert::From<documentation_items::Documentation> for Documentation {
    fn from(value: documentation_items::Documentation) -> Self {
        Documentation::Documentation(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Documentation>>,
> for Documentation {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Documentation>>,
    ) -> Self {
        Documentation::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Element {
    #[xelement(
        name = "element",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Element(#[xgroup] ::std::boxed::Box<crate::types::TopLevelElement>),
    SubstitutionGroup(::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Element>>),
}
impl ::core::convert::From<crate::types::TopLevelElement> for Element {
    fn from(value: crate::types::TopLevelElement) -> Self {
        Element::Element(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Element>>,
> for Element {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Element>>,
    ) -> Self {
        Element::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Enumeration {
    #[xelement(
        name = "enumeration",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Enumeration(#[xgroup] ::std::boxed::Box<crate::types::NoFixedFacet>),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Enumeration>>,
    ),
}
impl ::core::convert::From<crate::types::NoFixedFacet> for Enumeration {
    fn from(value: crate::types::NoFixedFacet) -> Self {
        Enumeration::Enumeration(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Enumeration>>,
> for Enumeration {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Enumeration>>,
    ) -> Self {
        Enumeration::SubstitutionGroup(value)
    }
}
pub mod explicit_timezone_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = value_value_with)]
    pub enum ValueValue {
        Optional,
        Required,
        Prohibited,
    }
    pub mod value_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::ValueValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::ValueValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::ValueValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum ValueValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for ValueValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                ValueValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for ValueValue {
        type Error = ValueValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "optional" => Ok(ValueValue::Optional),
                "required" => Ok(ValueValue::Required),
                "prohibited" => Ok(ValueValue::Prohibited),
                _ => {
                    Err(ValueValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<ValueValue> for ::std::string::String {
        fn from(value: ValueValue) -> Self {
            match value {
                ValueValue::Optional => ::std::string::String::from("optional"),
                ValueValue::Required => ::std::string::String::from("required"),
                ValueValue::Prohibited => ::std::string::String::from("prohibited"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct ExplicitTimezone {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "value")]
        pub value: ValueValue,
        #[xattribute(name = "fixed", optional)]
        pub fixed: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<crate::Annotation>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum ExplicitTimezone {
    #[xelement(
        name = "explicitTimezone",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    ExplicitTimezone(#[xgroup] explicit_timezone_items::ExplicitTimezone),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::ExplicitTimezone>>,
    ),
}
impl ::core::convert::From<explicit_timezone_items::ExplicitTimezone>
for ExplicitTimezone {
    fn from(value: explicit_timezone_items::ExplicitTimezone) -> Self {
        ExplicitTimezone::ExplicitTimezone(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::ExplicitTimezone>>,
> for ExplicitTimezone {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::ExplicitTimezone>>,
    ) -> Self {
        ExplicitTimezone::SubstitutionGroup(value)
    }
}
pub mod field_items {
    impl ::core::convert::From<::std::string::String> for XpathValue {
        fn from(value: ::std::string::String) -> Self {
            XpathValue(value)
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(with = xpath_value_with)]
    pub struct XpathValue(pub ::std::string::String);
    pub mod xpath_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::XpathValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::XpathValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::XpathValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
    pub enum XpathValueParseError {}
    impl ::core::convert::From<XpathValue> for ::std::string::String {
        fn from(value: XpathValue) -> Self {
            value.0
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Field {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "xpath")]
        pub xpath: XpathValue,
        #[xattribute(name = "xpathDefaultNamespace", optional)]
        pub xpath_default_namespace: ::core::option::Option<
            ::std::boxed::Box<crate::types::XpathDefaultNamespace>,
        >,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<crate::Annotation>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Field {
    #[xelement(
        name = "field",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Field(#[xgroup] field_items::Field),
    SubstitutionGroup(::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Field>>),
}
impl ::core::convert::From<field_items::Field> for Field {
    fn from(value: field_items::Field) -> Self {
        Field::Field(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Field>>,
> for Field {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Field>>,
    ) -> Self {
        Field::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum FractionDigits {
    #[xelement(
        name = "fractionDigits",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    FractionDigits(#[xgroup] ::std::boxed::Box<crate::types::NumFacet>),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::FractionDigits>>,
    ),
}
impl ::core::convert::From<crate::types::NumFacet> for FractionDigits {
    fn from(value: crate::types::NumFacet) -> Self {
        FractionDigits::FractionDigits(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::FractionDigits>>,
> for FractionDigits {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::FractionDigits>>,
    ) -> Self {
        FractionDigits::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Group {
    #[xelement(
        name = "group",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Group(#[xgroup] ::std::boxed::Box<crate::types::NamedGroup>),
    SubstitutionGroup(::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Group>>),
}
impl ::core::convert::From<crate::types::NamedGroup> for Group {
    fn from(value: crate::types::NamedGroup) -> Self {
        Group::Group(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Group>>,
> for Group {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Group>>,
    ) -> Self {
        Group::SubstitutionGroup(value)
    }
}
pub mod import_items {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Import {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "namespace", optional)]
        pub namespace: ::core::option::Option<crate::types::TargetNamespace>,
        #[xattribute(name = "schemaLocation", optional)]
        pub schema_location: ::core::option::Option<crate::types::TargetNamespace>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<crate::Annotation>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Import {
    #[xelement(
        name = "import",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Import(#[xgroup] import_items::Import),
    SubstitutionGroup(::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Import>>),
}
impl ::core::convert::From<import_items::Import> for Import {
    fn from(value: import_items::Import) -> Self {
        Import::Import(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Import>>,
> for Import {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Import>>,
    ) -> Self {
        Import::SubstitutionGroup(value)
    }
}
pub mod include_items {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Include {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "schemaLocation")]
        pub schema_location: crate::types::TargetNamespace,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<crate::Annotation>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Include {
    #[xelement(
        name = "include",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Include(#[xgroup] include_items::Include),
    SubstitutionGroup(::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Include>>),
}
impl ::core::convert::From<include_items::Include> for Include {
    fn from(value: include_items::Include) -> Self {
        Include::Include(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Include>>,
> for Include {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Include>>,
    ) -> Self {
        Include::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Key {
    #[xelement(
        name = "key",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Key(#[xgroup] ::std::boxed::Box<crate::types::Keybase>),
    SubstitutionGroup(::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Key>>),
}
impl ::core::convert::From<crate::types::Keybase> for Key {
    fn from(value: crate::types::Keybase) -> Self {
        Key::Key(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Key>>>
for Key {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Key>>,
    ) -> Self {
        Key::SubstitutionGroup(value)
    }
}
pub mod keyref_items {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct Child1 {
        pub selector: ::std::boxed::Box<crate::Selector>,
        #[xvalue(default)]
        #[builder(default)]
        pub field: ::std::vec::Vec<crate::Field>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Keyref {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name", optional)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xattribute(name = "ref", optional)]
        pub ref_: ::core::option::Option<crate::types::QName>,
        #[xattribute(name = "refer", optional)]
        pub refer: ::core::option::Option<crate::types::QName>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<crate::Annotation>,
        #[xvalue(default)]
        pub child_1: ::core::option::Option<Child1>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Keyref {
    #[xelement(
        name = "keyref",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Keyref(#[xgroup] keyref_items::Keyref),
    SubstitutionGroup(::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Keyref>>),
}
impl ::core::convert::From<keyref_items::Keyref> for Keyref {
    fn from(value: keyref_items::Keyref) -> Self {
        Keyref::Keyref(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Keyref>>,
> for Keyref {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Keyref>>,
    ) -> Self {
        Keyref::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Length {
    #[xelement(
        name = "length",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Length(#[xgroup] ::std::boxed::Box<crate::types::NumFacet>),
    SubstitutionGroup(::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Length>>),
}
impl ::core::convert::From<crate::types::NumFacet> for Length {
    fn from(value: crate::types::NumFacet) -> Self {
        Length::Length(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Length>>,
> for Length {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Length>>,
    ) -> Self {
        Length::SubstitutionGroup(value)
    }
}
pub mod list_items {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct List {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "itemType", optional)]
        pub item_type: ::core::option::Option<crate::types::QName>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<crate::Annotation>,
        #[xelement(
            name = "simpleType",
            namespace = "http://www.w3.org/2001/XMLSchema",
            group,
            optional
        )]
        pub simple_type: ::core::option::Option<
            ::std::boxed::Box<crate::types::LocalSimpleType>,
        >,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum List {
    #[xelement(
        name = "list",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    List(#[xgroup] list_items::List),
    SubstitutionGroup(::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::List>>),
}
impl ::core::convert::From<list_items::List> for List {
    fn from(value: list_items::List) -> Self {
        List::List(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::List>>,
> for List {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::List>>,
    ) -> Self {
        List::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum MaxExclusive {
    #[xelement(
        name = "maxExclusive",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    MaxExclusive(#[xgroup] ::std::boxed::Box<crate::types::Facet>),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::MaxExclusive>>,
    ),
}
impl ::core::convert::From<crate::types::Facet> for MaxExclusive {
    fn from(value: crate::types::Facet) -> Self {
        MaxExclusive::MaxExclusive(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::MaxExclusive>>,
> for MaxExclusive {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::MaxExclusive>>,
    ) -> Self {
        MaxExclusive::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum MaxInclusive {
    #[xelement(
        name = "maxInclusive",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    MaxInclusive(#[xgroup] ::std::boxed::Box<crate::types::Facet>),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::MaxInclusive>>,
    ),
}
impl ::core::convert::From<crate::types::Facet> for MaxInclusive {
    fn from(value: crate::types::Facet) -> Self {
        MaxInclusive::MaxInclusive(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::MaxInclusive>>,
> for MaxInclusive {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::MaxInclusive>>,
    ) -> Self {
        MaxInclusive::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum MaxLength {
    #[xelement(
        name = "maxLength",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    MaxLength(#[xgroup] ::std::boxed::Box<crate::types::NumFacet>),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::MaxLength>>,
    ),
}
impl ::core::convert::From<crate::types::NumFacet> for MaxLength {
    fn from(value: crate::types::NumFacet) -> Self {
        MaxLength::MaxLength(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::MaxLength>>,
> for MaxLength {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::MaxLength>>,
    ) -> Self {
        MaxLength::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum MinExclusive {
    #[xelement(
        name = "minExclusive",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    MinExclusive(#[xgroup] ::std::boxed::Box<crate::types::Facet>),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::MinExclusive>>,
    ),
}
impl ::core::convert::From<crate::types::Facet> for MinExclusive {
    fn from(value: crate::types::Facet) -> Self {
        MinExclusive::MinExclusive(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::MinExclusive>>,
> for MinExclusive {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::MinExclusive>>,
    ) -> Self {
        MinExclusive::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum MinInclusive {
    #[xelement(
        name = "minInclusive",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    MinInclusive(#[xgroup] ::std::boxed::Box<crate::types::Facet>),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::MinInclusive>>,
    ),
}
impl ::core::convert::From<crate::types::Facet> for MinInclusive {
    fn from(value: crate::types::Facet) -> Self {
        MinInclusive::MinInclusive(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::MinInclusive>>,
> for MinInclusive {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::MinInclusive>>,
    ) -> Self {
        MinInclusive::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum MinLength {
    #[xelement(
        name = "minLength",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    MinLength(#[xgroup] ::std::boxed::Box<crate::types::NumFacet>),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::MinLength>>,
    ),
}
impl ::core::convert::From<crate::types::NumFacet> for MinLength {
    fn from(value: crate::types::NumFacet) -> Self {
        MinLength::MinLength(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::MinLength>>,
> for MinLength {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::MinLength>>,
    ) -> Self {
        MinLength::SubstitutionGroup(value)
    }
}
pub mod notation_items {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Notation {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "name")]
        pub name: ::xmlity::LocalName<'static>,
        #[xattribute(name = "public", optional)]
        pub public: ::core::option::Option<::std::boxed::Box<crate::types::Public>>,
        #[xattribute(name = "system", optional)]
        pub system: ::core::option::Option<crate::types::TargetNamespace>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<crate::Annotation>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Notation {
    #[xelement(
        name = "notation",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Notation(#[xgroup] notation_items::Notation),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Notation>>,
    ),
}
impl ::core::convert::From<notation_items::Notation> for Notation {
    fn from(value: notation_items::Notation) -> Self {
        Notation::Notation(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Notation>>,
> for Notation {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Notation>>,
    ) -> Self {
        Notation::SubstitutionGroup(value)
    }
}
pub mod open_content_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = mode_value_with)]
    pub enum ModeValue {
        None,
        Interleave,
        Suffix,
    }
    pub mod mode_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::ModeValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::ModeValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::ModeValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum ModeValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for ModeValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                ModeValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for ModeValue {
        type Error = ModeValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "none" => Ok(ModeValue::None),
                "interleave" => Ok(ModeValue::Interleave),
                "suffix" => Ok(ModeValue::Suffix),
                _ => {
                    Err(ModeValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<ModeValue> for ::std::string::String {
        fn from(value: ModeValue) -> Self {
            match value {
                ModeValue::None => ::std::string::String::from("none"),
                ModeValue::Interleave => ::std::string::String::from("interleave"),
                ModeValue::Suffix => ::std::string::String::from("suffix"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct OpenContent {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "mode", optional)]
        pub mode: ::core::option::Option<ModeValue>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<crate::Annotation>,
        #[xelement(
            name = "any",
            namespace = "http://www.w3.org/2001/XMLSchema",
            group,
            optional
        )]
        pub any: ::core::option::Option<::std::boxed::Box<crate::types::Wildcard>>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum OpenContent {
    #[xelement(
        name = "openContent",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    OpenContent(#[xgroup] open_content_items::OpenContent),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::OpenContent>>,
    ),
}
impl ::core::convert::From<open_content_items::OpenContent> for OpenContent {
    fn from(value: open_content_items::OpenContent) -> Self {
        OpenContent::OpenContent(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::OpenContent>>,
> for OpenContent {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::OpenContent>>,
    ) -> Self {
        OpenContent::SubstitutionGroup(value)
    }
}
pub mod override_items {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Override {
        #[xattribute(name = "schemaLocation")]
        pub schema_location: crate::types::TargetNamespace,
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<crate::Annotation>,
        #[xvalue(default)]
        #[builder(default)]
        pub schema_top: ::std::vec::Vec<crate::groups::SchemaTop>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Override {
    #[xelement(
        name = "override",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Override(#[xgroup] override_items::Override),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Override>>,
    ),
}
impl ::core::convert::From<override_items::Override> for Override {
    fn from(value: override_items::Override) -> Self {
        Override::Override(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Override>>,
> for Override {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Override>>,
    ) -> Self {
        Override::SubstitutionGroup(value)
    }
}
pub mod pattern_items {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Pattern {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "value")]
        pub value: String,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<crate::Annotation>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Pattern {
    #[xelement(
        name = "pattern",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Pattern(#[xgroup] pattern_items::Pattern),
    SubstitutionGroup(::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Pattern>>),
}
impl ::core::convert::From<pattern_items::Pattern> for Pattern {
    fn from(value: pattern_items::Pattern) -> Self {
        Pattern::Pattern(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Pattern>>,
> for Pattern {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Pattern>>,
    ) -> Self {
        Pattern::SubstitutionGroup(value)
    }
}
pub mod redefine_items {
    impl ::core::convert::From<crate::Annotation> for RedefineContent {
        fn from(value: crate::Annotation) -> Self {
            RedefineContent::Annotation(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::groups::Redefinable> for RedefineContent {
        fn from(value: crate::groups::Redefinable) -> Self {
            RedefineContent::Redefinable(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum RedefineContent {
        Annotation(::std::boxed::Box<crate::Annotation>),
        Redefinable(::std::boxed::Box<crate::groups::Redefinable>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub struct Redefine {
        #[xattribute(name = "schemaLocation")]
        pub schema_location: crate::types::TargetNamespace,
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xvalue(default)]
        #[builder(default)]
        pub redefine_content: ::std::vec::Vec<RedefineContent>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Redefine {
    #[xelement(
        name = "redefine",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Redefine(#[xgroup] redefine_items::Redefine),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Redefine>>,
    ),
}
impl ::core::convert::From<redefine_items::Redefine> for Redefine {
    fn from(value: redefine_items::Redefine) -> Self {
        Redefine::Redefine(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Redefine>>,
> for Redefine {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Redefine>>,
    ) -> Self {
        Redefine::SubstitutionGroup(value)
    }
}
pub mod restriction_items {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Restriction {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "base", optional)]
        pub base: ::core::option::Option<crate::types::QName>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<crate::Annotation>,
        pub simple_restriction_model: crate::groups::SimpleRestrictionModel,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Restriction {
    #[xelement(
        name = "restriction",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Restriction(#[xgroup] restriction_items::Restriction),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Restriction>>,
    ),
}
impl ::core::convert::From<restriction_items::Restriction> for Restriction {
    fn from(value: restriction_items::Restriction) -> Self {
        Restriction::Restriction(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Restriction>>,
> for Restriction {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Restriction>>,
    ) -> Self {
        Restriction::SubstitutionGroup(value)
    }
}
pub mod schema_items {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct Child1 {
        pub default_open_content: crate::DefaultOpenContent,
        #[xvalue(default)]
        #[builder(default)]
        pub annotation: ::std::vec::Vec<crate::Annotation>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(order = "strict")]
    pub struct Child2 {
        pub schema_top: crate::groups::SchemaTop,
        #[xvalue(default)]
        #[builder(default)]
        pub annotation: ::std::vec::Vec<crate::Annotation>,
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Schema {
        #[xattribute(name = "targetNamespace", optional)]
        pub target_namespace: ::core::option::Option<crate::types::TargetNamespace>,
        #[xattribute(name = "version", optional)]
        pub version: ::core::option::Option<String>,
        #[xattribute(name = "finalDefault", optional)]
        pub final_default: ::core::option::Option<
            ::std::boxed::Box<crate::types::FullDerivationSet>,
        >,
        #[xattribute(name = "blockDefault", optional)]
        pub block_default: ::core::option::Option<
            ::std::boxed::Box<crate::types::BlockSet>,
        >,
        #[xattribute(name = "attributeFormDefault", optional)]
        pub attribute_form_default: ::core::option::Option<
            ::std::boxed::Box<crate::types::FormChoice>,
        >,
        #[xattribute(name = "elementFormDefault", optional)]
        pub element_form_default: ::core::option::Option<
            ::std::boxed::Box<crate::types::FormChoice>,
        >,
        #[xattribute(name = "defaultAttributes", optional)]
        pub default_attributes: ::core::option::Option<crate::types::QName>,
        #[xattribute(name = "xpathDefaultNamespace", optional)]
        pub xpath_default_namespace: ::core::option::Option<
            ::std::boxed::Box<crate::types::XpathDefaultNamespace>,
        >,
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(deferred = true, optional)]
        pub lang: ::core::option::Option<
            ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
        >,
        #[xvalue(default)]
        #[builder(default)]
        pub composition: ::std::vec::Vec<crate::groups::Composition>,
        #[xvalue(default)]
        pub child_1: ::core::option::Option<Child1>,
        #[xvalue(default)]
        #[builder(default)]
        pub child_2: ::std::vec::Vec<Child2>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Schema {
    #[xelement(
        name = "schema",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Schema(#[xgroup] schema_items::Schema),
    SubstitutionGroup(::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Schema>>),
}
impl ::core::convert::From<schema_items::Schema> for Schema {
    fn from(value: schema_items::Schema) -> Self {
        Schema::Schema(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Schema>>,
> for Schema {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Schema>>,
    ) -> Self {
        Schema::SubstitutionGroup(value)
    }
}
pub mod selector_items {
    impl ::core::convert::From<::std::string::String> for XpathValue {
        fn from(value: ::std::string::String) -> Self {
            XpathValue(value)
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xvalue(with = xpath_value_with)]
    pub struct XpathValue(pub ::std::string::String);
    pub mod xpath_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::XpathValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::XpathValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::XpathValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug, ::core::cmp::PartialEq, ::core::clone::Clone)]
    pub enum XpathValueParseError {}
    impl ::core::convert::From<XpathValue> for ::std::string::String {
        fn from(value: XpathValue) -> Self {
            value.0
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Selector {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "xpath")]
        pub xpath: XpathValue,
        #[xattribute(name = "xpathDefaultNamespace", optional)]
        pub xpath_default_namespace: ::core::option::Option<
            ::std::boxed::Box<crate::types::XpathDefaultNamespace>,
        >,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<crate::Annotation>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Selector {
    #[xelement(
        name = "selector",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Selector(#[xgroup] selector_items::Selector),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Selector>>,
    ),
}
impl ::core::convert::From<selector_items::Selector> for Selector {
    fn from(value: selector_items::Selector) -> Self {
        Selector::Selector(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Selector>>,
> for Selector {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Selector>>,
    ) -> Self {
        Selector::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Sequence {
    #[xelement(
        name = "sequence",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Sequence(#[xgroup] ::std::boxed::Box<crate::types::ExplicitGroup>),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Sequence>>,
    ),
}
impl ::core::convert::From<crate::types::ExplicitGroup> for Sequence {
    fn from(value: crate::types::ExplicitGroup) -> Self {
        Sequence::Sequence(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Sequence>>,
> for Sequence {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Sequence>>,
    ) -> Self {
        Sequence::SubstitutionGroup(value)
    }
}
pub mod simple_content_items {
    impl ::core::convert::From<crate::types::SimpleRestrictionType> for Child1 {
        fn from(value: crate::types::SimpleRestrictionType) -> Self {
            Child1::Restriction(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::types::SimpleExtensionType> for Child1 {
        fn from(value: crate::types::SimpleExtensionType) -> Self {
            Child1::Extension(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    pub enum Child1 {
        #[xelement(
            name = "restriction",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Restriction(#[xgroup] ::std::boxed::Box<crate::types::SimpleRestrictionType>),
        #[xelement(
            name = "extension",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Extension(#[xgroup] ::std::boxed::Box<crate::types::SimpleExtensionType>),
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct SimpleContent {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<crate::Annotation>,
        pub child_1: Child1,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum SimpleContent {
    #[xelement(
        name = "simpleContent",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    SimpleContent(#[xgroup] simple_content_items::SimpleContent),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::SimpleContent>>,
    ),
}
impl ::core::convert::From<simple_content_items::SimpleContent> for SimpleContent {
    fn from(value: simple_content_items::SimpleContent) -> Self {
        SimpleContent::SimpleContent(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::SimpleContent>>,
> for SimpleContent {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::SimpleContent>>,
    ) -> Self {
        SimpleContent::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum SimpleType {
    #[xelement(
        name = "simpleType",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    SimpleType(#[xgroup] ::std::boxed::Box<crate::types::TopLevelSimpleType>),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::SimpleType>>,
    ),
}
impl ::core::convert::From<crate::types::TopLevelSimpleType> for SimpleType {
    fn from(value: crate::types::TopLevelSimpleType) -> Self {
        SimpleType::SimpleType(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::SimpleType>>,
> for SimpleType {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::SimpleType>>,
    ) -> Self {
        SimpleType::SubstitutionGroup(value)
    }
}
pub mod total_digits_items {
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct TotalDigits {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "value")]
        pub value: ::core::num::NonZeroUsize,
        #[xattribute(name = "fixed", optional)]
        pub fixed: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<crate::Annotation>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum TotalDigits {
    #[xelement(
        name = "totalDigits",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    TotalDigits(#[xgroup] total_digits_items::TotalDigits),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::TotalDigits>>,
    ),
}
impl ::core::convert::From<total_digits_items::TotalDigits> for TotalDigits {
    fn from(value: total_digits_items::TotalDigits) -> Self {
        TotalDigits::TotalDigits(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::TotalDigits>>,
> for TotalDigits {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::TotalDigits>>,
    ) -> Self {
        TotalDigits::SubstitutionGroup(value)
    }
}
pub mod union_items {
    impl ::core::convert::From<crate::types::LocalSimpleType> for SimpleType {
        fn from(value: crate::types::LocalSimpleType) -> Self {
            SimpleType(::std::boxed::Box::new(value))
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xelement(
        name = "simpleType",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    pub struct SimpleType(
        #[xgroup]
        pub ::std::boxed::Box<crate::types::LocalSimpleType>,
    );
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct Union {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "memberTypes", optional)]
        pub member_types: ::core::option::Option<::xmlity_ns::List<crate::types::QName>>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<crate::Annotation>,
        #[xvalue(default)]
        #[builder(default)]
        pub simple_type: ::std::vec::Vec<SimpleType>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Union {
    #[xelement(
        name = "union",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Union(#[xgroup] union_items::Union),
    SubstitutionGroup(::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Union>>),
}
impl ::core::convert::From<union_items::Union> for Union {
    fn from(value: union_items::Union) -> Self {
        Union::Union(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Union>>,
> for Union {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Union>>,
    ) -> Self {
        Union::SubstitutionGroup(value)
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum Unique {
    #[xelement(
        name = "unique",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    Unique(#[xgroup] ::std::boxed::Box<crate::types::Keybase>),
    SubstitutionGroup(::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Unique>>),
}
impl ::core::convert::From<crate::types::Keybase> for Unique {
    fn from(value: crate::types::Keybase) -> Self {
        Unique::Unique(::std::boxed::Box::new(value))
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Unique>>,
> for Unique {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::Unique>>,
    ) -> Self {
        Unique::SubstitutionGroup(value)
    }
}
pub mod white_space_items {
    #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::xmlity::Serialize,
        ::xmlity::Deserialize,
        ::core::cmp::PartialEq
    )]
    #[xvalue(with = value_value_with)]
    pub enum ValueValue {
        Preserve,
        Replace,
        Collapse,
    }
    pub mod value_value_with {
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> ::core::result::Result<super::ValueValue, D::Error>
        where
            D: ::xmlity::Deserializer<'de>,
        {
            let text: ::std::string::String = ::xmlity::Deserialize::deserialize(
                deserializer,
            )?;
            let value: ::std::string::String = text
                .parse()
                .map_err(::xmlity::de::Error::custom)?;
            super::ValueValue::try_from(value).map_err(::xmlity::de::Error::custom)
        }
        pub fn serialize<S>(
            value: &super::ValueValue,
            serializer: S,
        ) -> ::core::result::Result<S::Ok, S::Error>
        where
            S: ::xmlity::Serializer,
        {
            let value: ::std::string::String = ::core::clone::Clone::clone(value).into();
            ::xmlity::Serialize::serialize(
                ::std::string::String::as_str(
                    &::std::string::ToString::to_string(&value),
                ),
                serializer,
            )
        }
    }
    #[derive(::core::fmt::Debug)]
    pub enum ValueValueParseError {
        NonExistent { value: ::std::string::String },
    }
    impl ::core::fmt::Display for ValueValueParseError {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match self {
                ValueValueParseError::NonExistent { value } => {
                    write!(f, "Value '{:?}' does not exist in the enumeration", value)
                }
            }
        }
    }
    impl ::core::convert::TryFrom<::std::string::String> for ValueValue {
        type Error = ValueValueParseError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::core::result::Result<Self, Self::Error> {
            match ::std::string::String::as_str(&value) {
                "preserve" => Ok(ValueValue::Preserve),
                "replace" => Ok(ValueValue::Replace),
                "collapse" => Ok(ValueValue::Collapse),
                _ => {
                    Err(ValueValueParseError::NonExistent {
                        value,
                    })
                }
            }
        }
    }
    impl ::core::convert::From<ValueValue> for ::std::string::String {
        fn from(value: ValueValue) -> Self {
            match value {
                ValueValue::Preserve => ::std::string::String::from("preserve"),
                ValueValue::Replace => ::std::string::String::from("replace"),
                ValueValue::Collapse => ::std::string::String::from("collapse"),
            }
        }
    }
    #[derive(
        ::core::fmt::Debug,
        ::xmlity::SerializationGroup,
        ::xmlity::DeserializationGroup,
        ::bon::Builder,
        ::core::cmp::PartialEq,
        ::core::clone::Clone
    )]
    #[xgroup(children_order = "strict")]
    pub struct WhiteSpace {
        #[xattribute(name = "id", optional)]
        pub id: ::core::option::Option<String>,
        #[xattribute(name = "value")]
        pub value: ValueValue,
        #[xattribute(name = "fixed", optional)]
        pub fixed: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<crate::Annotation>,
    }
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
pub enum WhiteSpace {
    #[xelement(
        name = "whiteSpace",
        namespace = "http://www.w3.org/2001/XMLSchema",
        allow_unknown_attributes = "any"
    )]
    WhiteSpace(#[xgroup] white_space_items::WhiteSpace),
    SubstitutionGroup(
        ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::WhiteSpace>>,
    ),
}
impl ::core::convert::From<white_space_items::WhiteSpace> for WhiteSpace {
    fn from(value: white_space_items::WhiteSpace) -> Self {
        WhiteSpace::WhiteSpace(value)
    }
}
impl ::core::convert::From<
    ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::WhiteSpace>>,
> for WhiteSpace {
    fn from(
        value: ::xmlity_ns::SubstitutionGroup<::std::boxed::Box<crate::WhiteSpace>>,
    ) -> Self {
        WhiteSpace::SubstitutionGroup(value)
    }
}
