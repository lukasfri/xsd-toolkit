pub mod types {
    pub mod all_nni_items {
        pub mod variant_0_variants {
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
                AllNNI::Variant0(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<variant_0_variants::Variant0> for AllNNI {
            fn from(value: variant_0_variants::Variant0) -> Self {
                AllNNI::Variant0_0(::std::boxed::Box::new(value))
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
            Variant0(::std::boxed::Box<usize>),
            Variant0_0(::std::boxed::Box<variant_0_variants::Variant0>),
        }
    }
    pub type AllNNI = all_nni_items::AllNNI;
    pub mod basic_namespace_list_items {
        pub mod variant_0_variants {
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
        impl ::core::convert::From<crate::xs::types::TargetNamespace>
        for BasicNamespaceList {
            fn from(value: crate::xs::types::TargetNamespace) -> Self {
                BasicNamespaceList::Variant0(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<variant_0_variants::Variant0> for BasicNamespaceList {
            fn from(value: variant_0_variants::Variant0) -> Self {
                BasicNamespaceList::Variant0_0(::std::boxed::Box::new(value))
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
            Variant0(::std::boxed::Box<crate::xs::types::TargetNamespace>),
            Variant0_0(::std::boxed::Box<variant_0_variants::Variant0>),
        }
    }
    pub type BasicNamespaceList = crate::xs::types::List<
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
        }
        impl ::core::convert::From<variant_variants::Variant0> for BlockSet {
            fn from(value: variant_variants::Variant0) -> Self {
                BlockSet::Variant0(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::types::List<::std::string::String>>
        for BlockSet {
            fn from(value: crate::xs::types::List<::std::string::String>) -> Self {
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
            Variant1(::std::boxed::Box<crate::xs::types::List<::std::string::String>>),
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
            crate::xs::types::List<
                ::std::boxed::Box<crate::xs::types::ReducedDerivationControl>,
            >,
        > for DerivationSet {
            fn from(
                value: crate::xs::types::List<
                    ::std::boxed::Box<crate::xs::types::ReducedDerivationControl>,
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
                    crate::xs::types::List<
                        ::std::boxed::Box<crate::xs::types::ReducedDerivationControl>,
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
            crate::xs::types::List<
                ::std::boxed::Box<crate::xs::types::TypeDerivationControl>,
            >,
        > for FullDerivationSet {
            fn from(
                value: crate::xs::types::List<
                    ::std::boxed::Box<crate::xs::types::TypeDerivationControl>,
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
                    crate::xs::types::List<
                        ::std::boxed::Box<crate::xs::types::TypeDerivationControl>,
                    >,
                >,
            ),
        }
    }
    pub type FullDerivationSet = full_derivation_set_items::FullDerivationSet;
    pub mod namespace_list_items {
        impl ::core::convert::From<crate::xs::types::SpecialNamespaceList>
        for NamespaceList {
            fn from(value: crate::xs::types::SpecialNamespaceList) -> Self {
                NamespaceList::Variant0(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::types::BasicNamespaceList>
        for NamespaceList {
            fn from(value: crate::xs::types::BasicNamespaceList) -> Self {
                NamespaceList::Variant1(::std::boxed::Box::new(value))
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
            Variant0(::std::boxed::Box<crate::xs::types::SpecialNamespaceList>),
            Variant1(::std::boxed::Box<crate::xs::types::BasicNamespaceList>),
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
        pub mod variant_0_variants {
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
        impl ::core::convert::From<crate::xs::types::QName> for QnameList {
            fn from(value: crate::xs::types::QName) -> Self {
                QnameList::Variant0(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<variant_0_variants::Variant0> for QnameList {
            fn from(value: variant_0_variants::Variant0) -> Self {
                QnameList::Variant0_0(::std::boxed::Box::new(value))
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
            Variant0(::std::boxed::Box<crate::xs::types::QName>),
            Variant0_0(::std::boxed::Box<variant_0_variants::Variant0>),
        }
    }
    pub type QnameList = crate::xs::types::List<qname_list_items::QnameList>;
    pub mod qname_list_a_items {
        pub mod variant_0_variants {
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
        impl ::core::convert::From<crate::xs::types::QName> for QnameListA {
            fn from(value: crate::xs::types::QName) -> Self {
                QnameListA::Variant0(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<variant_0_variants::Variant0> for QnameListA {
            fn from(value: variant_0_variants::Variant0) -> Self {
                QnameListA::Variant0_0(::std::boxed::Box::new(value))
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
            Variant0(::std::boxed::Box<crate::xs::types::QName>),
            Variant0_0(::std::boxed::Box<variant_0_variants::Variant0>),
        }
    }
    pub type QnameListA = crate::xs::types::List<qname_list_a_items::QnameListA>;
    pub type ReducedDerivationControl = ::std::string::String;
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
        }
        impl ::core::convert::From<variant_variants::Variant0> for SimpleDerivationSet {
            fn from(value: variant_variants::Variant0) -> Self {
                SimpleDerivationSet::Variant0(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::types::List<::std::string::String>>
        for SimpleDerivationSet {
            fn from(value: crate::xs::types::List<::std::string::String>) -> Self {
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
            Variant1(::std::boxed::Box<crate::xs::types::List<::std::string::String>>),
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
    pub type TypeDerivationControl = ::std::string::String;
    pub mod xpath_default_namespace_items {
        pub mod variant_0_variants {
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
        impl ::core::convert::From<crate::xs::types::TargetNamespace>
        for XpathDefaultNamespace {
            fn from(value: crate::xs::types::TargetNamespace) -> Self {
                XpathDefaultNamespace::Variant0(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<variant_0_variants::Variant0>
        for XpathDefaultNamespace {
            fn from(value: variant_0_variants::Variant0) -> Self {
                XpathDefaultNamespace::Variant0_0(::std::boxed::Box::new(value))
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
            Variant0(::std::boxed::Box<crate::xs::types::TargetNamespace>),
            Variant0_0(::std::boxed::Box<variant_0_variants::Variant0>),
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
        pub max_occurs: ::core::option::Option<::std::string::String>,
        pub all_model: ::std::boxed::Box<crate::xs::groups::AllModel>,
    }
    pub mod alt_type_items {
        impl ::core::convert::From<crate::xs::types::LocalSimpleType> for Type {
            fn from(value: crate::xs::types::LocalSimpleType) -> Self {
                Type::SimpleType(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::types::LocalComplexType> for Type {
            fn from(value: crate::xs::types::LocalComplexType) -> Self {
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
            SimpleType(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalSimpleType>),
            #[xelement(
                name = "complexType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            ComplexType(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalComplexType>),
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
        pub type_attribute: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "xpathDefaultNamespace", optional)]
        pub xpath_default_namespace: ::core::option::Option<
            ::std::boxed::Box<crate::xs::types::XpathDefaultNamespace>,
        >,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
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
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
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
            ::std::boxed::Box<crate::xs::types::XpathDefaultNamespace>,
        >,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
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
        pub ref_: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "type", optional)]
        pub type_: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "use", optional)]
        pub use_: ::core::option::Option<attribute_items::UseValue>,
        #[xattribute(name = "default", optional)]
        pub default: ::core::option::Option<String>,
        #[xattribute(name = "fixed", optional)]
        pub fixed: ::core::option::Option<String>,
        #[xattribute(name = "form", optional)]
        pub form: ::core::option::Option<
            ::std::boxed::Box<crate::xs::types::FormChoice>,
        >,
        #[xattribute(name = "targetNamespace", optional)]
        pub target_namespace: ::core::option::Option<crate::xs::types::TargetNamespace>,
        #[xattribute(name = "inheritable", optional)]
        pub inheritable: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xelement(
            name = "simpleType",
            namespace = "http://www.w3.org/2001/XMLSchema",
            group,
            optional
        )]
        pub simple_type: ::core::option::Option<
            ::std::boxed::Box<crate::xs::types::LocalSimpleType>,
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
        pub ref_: ::core::option::Option<crate::xs::types::QName>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        pub attr_decls: ::std::boxed::Box<crate::xs::groups::AttrDecls>,
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
        pub ref_: crate::xs::types::QName,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
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
            ::std::boxed::Box<crate::xs::types::DerivationSet>,
        >,
        #[xattribute(name = "block", optional)]
        pub block: ::core::option::Option<
            ::std::boxed::Box<crate::xs::types::DerivationSet>,
        >,
        #[xattribute(name = "defaultAttributesApply", optional)]
        pub default_attributes_apply: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        pub complex_type_model: ::std::boxed::Box<crate::xs::groups::ComplexTypeModel>,
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
                ::std::boxed::Box<crate::xs::OpenContent>,
            >,
            pub type_def_particle: ::std::boxed::Box<crate::xs::groups::TypeDefParticle>,
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
        pub base: crate::xs::types::QName,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        pub child_1: ::core::option::Option<complex_restriction_type_items::Child1>,
        pub attr_decls: ::std::boxed::Box<crate::xs::groups::AttrDecls>,
        pub assertions: ::std::boxed::Box<crate::xs::groups::Assertions>,
    }
    pub mod element_items {
        impl ::core::convert::From<crate::xs::types::LocalSimpleType> for Type {
            fn from(value: crate::xs::types::LocalSimpleType) -> Self {
                Type::SimpleType(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::types::LocalComplexType> for Type {
            fn from(value: crate::xs::types::LocalComplexType) -> Self {
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
            SimpleType(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalSimpleType>),
            #[xelement(
                name = "complexType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            ComplexType(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalComplexType>),
        }
        impl ::core::convert::From<crate::xs::types::AltType> for Alternative {
            fn from(value: crate::xs::types::AltType) -> Self {
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
        pub struct Alternative(
            #[xgroup]
            pub ::std::boxed::Box<crate::xs::types::AltType>,
        );
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
        pub ref_: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "type", optional)]
        pub type_attribute: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "substitutionGroup", optional)]
        pub substitution_group: ::core::option::Option<
            crate::xs::types::List<crate::xs::types::QName>,
        >,
        #[xattribute(name = "minOccurs", optional)]
        pub min_occurs: ::core::option::Option<usize>,
        #[xattribute(name = "maxOccurs", optional)]
        pub max_occurs: ::core::option::Option<
            ::std::boxed::Box<crate::xs::types::AllNNI>,
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
            ::std::boxed::Box<crate::xs::types::DerivationSet>,
        >,
        #[xattribute(name = "block", optional)]
        pub block: ::core::option::Option<::std::boxed::Box<crate::xs::types::BlockSet>>,
        #[xattribute(name = "form", optional)]
        pub form: ::core::option::Option<
            ::std::boxed::Box<crate::xs::types::FormChoice>,
        >,
        #[xattribute(name = "targetNamespace", optional)]
        pub target_namespace: ::core::option::Option<crate::xs::types::TargetNamespace>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        pub type_: ::core::option::Option<element_items::Type>,
        #[xvalue(default)]
        #[builder(default)]
        pub alternative: ::std::vec::Vec<element_items::Alternative>,
        #[xvalue(default)]
        #[builder(default)]
        pub identity_constraint: ::std::vec::Vec<crate::xs::groups::IdentityConstraint>,
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
        pub max_occurs: ::core::option::Option<
            ::std::boxed::Box<crate::xs::types::AllNNI>,
        >,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        #[builder(default)]
        pub nested_particle: ::std::vec::Vec<crate::xs::groups::NestedParticle>,
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
        pub base: crate::xs::types::QName,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        pub open_content: ::core::option::Option<
            ::std::boxed::Box<crate::xs::OpenContent>,
        >,
        #[xvalue(default)]
        pub type_def_particle: ::core::option::Option<
            ::std::boxed::Box<crate::xs::groups::TypeDefParticle>,
        >,
        pub attr_decls: ::std::boxed::Box<crate::xs::groups::AttrDecls>,
        pub assertions: ::std::boxed::Box<crate::xs::groups::Assertions>,
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
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
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
        pub ref_: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "minOccurs", optional)]
        pub min_occurs: ::core::option::Option<usize>,
        #[xattribute(name = "maxOccurs", optional)]
        pub max_occurs: ::core::option::Option<
            ::std::boxed::Box<crate::xs::types::AllNNI>,
        >,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        #[builder(default)]
        pub particle: ::std::vec::Vec<crate::xs::groups::Particle>,
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
        pub ref_: crate::xs::types::QName,
        #[xattribute(name = "minOccurs", optional)]
        pub min_occurs: ::core::option::Option<usize>,
        #[xattribute(name = "maxOccurs", optional)]
        pub max_occurs: ::core::option::Option<
            ::std::boxed::Box<crate::xs::types::AllNNI>,
        >,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
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
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
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
            pub selector: ::std::boxed::Box<crate::xs::Selector>,
            #[xvalue(default)]
            #[builder(default)]
            pub field: ::std::vec::Vec<crate::xs::Field>,
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
        pub ref_: ::core::option::Option<crate::xs::types::QName>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
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
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        pub complex_type_model: ::std::boxed::Box<crate::xs::groups::ComplexTypeModel>,
    }
    pub mod local_element_items {
        impl ::core::convert::From<crate::xs::types::LocalSimpleType> for Type {
            fn from(value: crate::xs::types::LocalSimpleType) -> Self {
                Type::SimpleType(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::types::LocalComplexType> for Type {
            fn from(value: crate::xs::types::LocalComplexType) -> Self {
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
            SimpleType(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalSimpleType>),
            #[xelement(
                name = "complexType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            ComplexType(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalComplexType>),
        }
        impl ::core::convert::From<crate::xs::types::AltType> for Alternative {
            fn from(value: crate::xs::types::AltType) -> Self {
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
        pub struct Alternative(
            #[xgroup]
            pub ::std::boxed::Box<crate::xs::types::AltType>,
        );
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
        pub ref_: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "type", optional)]
        pub type_attribute: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "minOccurs", optional)]
        pub min_occurs: ::core::option::Option<usize>,
        #[xattribute(name = "maxOccurs", optional)]
        pub max_occurs: ::core::option::Option<
            ::std::boxed::Box<crate::xs::types::AllNNI>,
        >,
        #[xattribute(name = "default", optional)]
        pub default: ::core::option::Option<String>,
        #[xattribute(name = "fixed", optional)]
        pub fixed: ::core::option::Option<String>,
        #[xattribute(name = "nillable", optional)]
        pub nillable: ::core::option::Option<bool>,
        #[xattribute(name = "block", optional)]
        pub block: ::core::option::Option<::std::boxed::Box<crate::xs::types::BlockSet>>,
        #[xattribute(name = "form", optional)]
        pub form: ::core::option::Option<
            ::std::boxed::Box<crate::xs::types::FormChoice>,
        >,
        #[xattribute(name = "targetNamespace", optional)]
        pub target_namespace: ::core::option::Option<crate::xs::types::TargetNamespace>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        pub type_: ::core::option::Option<local_element_items::Type>,
        #[xvalue(default)]
        #[builder(default)]
        pub alternative: ::std::vec::Vec<local_element_items::Alternative>,
        #[xvalue(default)]
        #[builder(default)]
        pub identity_constraint: ::std::vec::Vec<crate::xs::groups::IdentityConstraint>,
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
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        pub simple_derivation: ::std::boxed::Box<crate::xs::groups::SimpleDerivation>,
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
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        pub attr_decls: ::std::boxed::Box<crate::xs::groups::AttrDecls>,
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
                    ::std::boxed::Box<crate::xs::types::AllNNI>,
                >,
                all_model: ::std::boxed::Box<crate::xs::groups::AllModel>,
            },
            #[xelement(
                name = "choice",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            Choice(#[xgroup] ::std::boxed::Box<crate::xs::types::SimpleExplicitGroup>),
            #[xelement(
                name = "sequence",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            Sequence(#[xgroup] ::std::boxed::Box<crate::xs::types::SimpleExplicitGroup>),
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
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
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
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
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
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
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
        impl ::core::convert::From<crate::xs::All> for Child1 {
            fn from(value: crate::xs::All) -> Self {
                Child1::All(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::Choice> for Child1 {
            fn from(value: crate::xs::Choice) -> Self {
                Child1::Choice(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::Sequence> for Child1 {
            fn from(value: crate::xs::Sequence) -> Self {
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
            All(::std::boxed::Box<crate::xs::All>),
            Choice(::std::boxed::Box<crate::xs::Choice>),
            Sequence(::std::boxed::Box<crate::xs::Sequence>),
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
        pub ref_: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "minOccurs", optional)]
        pub min_occurs: ::core::option::Option<usize>,
        #[xattribute(name = "maxOccurs", optional)]
        pub max_occurs: ::core::option::Option<
            ::std::boxed::Box<crate::xs::types::AllNNI>,
        >,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
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
                    ::std::boxed::Box<crate::xs::OpenContent>,
                >,
                pub type_def_particle: ::std::boxed::Box<
                    crate::xs::groups::TypeDefParticle,
                >,
            }
        }
        impl ::core::convert::From<child_1_variants::Variant0> for Child1 {
            fn from(value: child_1_variants::Variant0) -> Self {
                Child1::Variant0(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::groups::SimpleRestrictionModel>
        for Child1 {
            fn from(value: crate::xs::groups::SimpleRestrictionModel) -> Self {
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
                ::std::boxed::Box<crate::xs::groups::SimpleRestrictionModel>,
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
        pub base: crate::xs::types::QName,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        pub child_1: ::core::option::Option<restriction_type_items::Child1>,
        pub attr_decls: ::std::boxed::Box<crate::xs::groups::AttrDecls>,
        pub assertions: ::std::boxed::Box<crate::xs::groups::Assertions>,
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
            ::std::boxed::Box<crate::xs::types::SimpleDerivationSet>,
        >,
        #[xattribute(name = "name", optional)]
        pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        pub simple_derivation: ::std::boxed::Box<crate::xs::groups::SimpleDerivation>,
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
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        #[builder(default)]
        pub nested_particle: ::std::vec::Vec<crate::xs::groups::NestedParticle>,
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
        pub base: crate::xs::types::QName,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        pub attr_decls: ::std::boxed::Box<crate::xs::groups::AttrDecls>,
        pub assertions: ::std::boxed::Box<crate::xs::groups::Assertions>,
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
                crate::xs::groups::SimpleRestrictionModel,
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
        pub base: crate::xs::types::QName,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        pub simple_restriction_model: ::core::option::Option<
            simple_restriction_type_items::SimpleRestrictionModel,
        >,
        pub attr_decls: ::std::boxed::Box<crate::xs::groups::AttrDecls>,
        pub assertions: ::std::boxed::Box<crate::xs::groups::Assertions>,
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
        pub type_: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "default", optional)]
        pub default: ::core::option::Option<String>,
        #[xattribute(name = "fixed", optional)]
        pub fixed: ::core::option::Option<String>,
        #[xattribute(name = "inheritable", optional)]
        pub inheritable: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xelement(
            name = "simpleType",
            namespace = "http://www.w3.org/2001/XMLSchema",
            group,
            optional
        )]
        pub simple_type: ::core::option::Option<
            ::std::boxed::Box<crate::xs::types::LocalSimpleType>,
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
            ::std::boxed::Box<crate::xs::types::DerivationSet>,
        >,
        #[xattribute(name = "block", optional)]
        pub block: ::core::option::Option<
            ::std::boxed::Box<crate::xs::types::DerivationSet>,
        >,
        #[xattribute(name = "defaultAttributesApply", optional)]
        pub default_attributes_apply: ::core::option::Option<bool>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        pub complex_type_model: ::std::boxed::Box<crate::xs::groups::ComplexTypeModel>,
    }
    pub mod top_level_element_items {
        impl ::core::convert::From<crate::xs::types::LocalSimpleType> for Type {
            fn from(value: crate::xs::types::LocalSimpleType) -> Self {
                Type::SimpleType(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::types::LocalComplexType> for Type {
            fn from(value: crate::xs::types::LocalComplexType) -> Self {
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
            SimpleType(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalSimpleType>),
            #[xelement(
                name = "complexType",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            ComplexType(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalComplexType>),
        }
        impl ::core::convert::From<crate::xs::types::AltType> for Alternative {
            fn from(value: crate::xs::types::AltType) -> Self {
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
        pub struct Alternative(
            #[xgroup]
            pub ::std::boxed::Box<crate::xs::types::AltType>,
        );
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
        pub type_attribute: ::core::option::Option<crate::xs::types::QName>,
        #[xattribute(name = "substitutionGroup", optional)]
        pub substitution_group: ::core::option::Option<
            crate::xs::types::List<crate::xs::types::QName>,
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
            ::std::boxed::Box<crate::xs::types::DerivationSet>,
        >,
        #[xattribute(name = "block", optional)]
        pub block: ::core::option::Option<::std::boxed::Box<crate::xs::types::BlockSet>>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        pub type_: ::core::option::Option<top_level_element_items::Type>,
        #[xvalue(default)]
        #[builder(default)]
        pub alternative: ::std::vec::Vec<top_level_element_items::Alternative>,
        #[xvalue(default)]
        #[builder(default)]
        pub identity_constraint: ::std::vec::Vec<crate::xs::groups::IdentityConstraint>,
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
            ::std::boxed::Box<crate::xs::types::SimpleDerivationSet>,
        >,
        #[xattribute(name = "name")]
        pub name: ::xmlity::LocalName<'static>,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        pub simple_derivation: ::std::boxed::Box<crate::xs::groups::SimpleDerivation>,
    }
    pub mod wildcard_items {
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
            ::std::boxed::Box<crate::xs::types::NamespaceList>,
        >,
        #[xattribute(name = "notNamespace", optional)]
        pub not_namespace: ::core::option::Option<::std::string::String>,
        #[xattribute(name = "processContents", optional)]
        pub process_contents: ::core::option::Option<
            wildcard_items::ProcessContentsValue,
        >,
        #[xvalue(default)]
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
    }
}
pub mod attributes {}
pub mod groups {
    pub mod all_model_items {
        impl ::core::convert::From<crate::xs::types::LocalElement> for Child1 {
            fn from(value: crate::xs::types::LocalElement) -> Self {
                Child1::Element(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::Any> for Child1 {
            fn from(value: crate::xs::Any) -> Self {
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
            Element(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalElement>),
            Any(::std::boxed::Box<crate::xs::Any>),
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
                ref_: crate::xs::types::QName,
                #[xattribute(name = "minOccurs", optional)]
                min_occurs: ::core::option::Option<usize>,
                #[xattribute(name = "maxOccurs", optional)]
                max_occurs: ::core::option::Option<usize>,
                #[xvalue(default)]
                annotation: ::core::option::Option<
                    ::std::boxed::Box<crate::xs::Annotation>,
                >,
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
        pub annotation: ::core::option::Option<::std::boxed::Box<crate::xs::Annotation>>,
        #[xvalue(default)]
        #[builder(default)]
        pub child_1: ::std::vec::Vec<all_model_items::Child1>,
    }
    pub mod assertions_items {
        impl ::core::convert::From<crate::xs::types::Assertion> for Assert {
            fn from(value: crate::xs::types::Assertion) -> Self {
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
        pub struct Assert(#[xgroup] pub ::std::boxed::Box<crate::xs::types::Assertion>);
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
        impl ::core::convert::From<crate::xs::types::Attribute> for Attribute {
            fn from(value: crate::xs::types::Attribute) -> Self {
                Attribute::Attribute(::std::boxed::Box::new(value))
            }
        }
        impl ::core::convert::From<crate::xs::types::AttributeGroupRef> for Attribute {
            fn from(value: crate::xs::types::AttributeGroupRef) -> Self {
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
            Attribute(#[xgroup] ::std::boxed::Box<crate::xs::types::Attribute>),
            #[xelement(
                name = "attributeGroup",
                namespace = "http://www.w3.org/2001/XMLSchema",
                allow_unknown_attributes = "any"
            )]
            AttributeGroup(
                #[xgroup]
                ::std::boxed::Box<crate::xs::types::AttributeGroupRef>,
            ),
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
            ::std::boxed::Box<crate::xs::AnyAttribute>,
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
                    ::std::boxed::Box<crate::xs::OpenContent>,
                >,
                #[xvalue(default)]
                pub type_def_particle: ::core::option::Option<
                    ::std::boxed::Box<crate::xs::groups::TypeDefParticle>,
                >,
                pub attr_decls: crate::xs::groups::AttrDecls,
                pub assertions: crate::xs::groups::Assertions,
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
        SimpleContent(::std::boxed::Box<crate::xs::SimpleContent>),
        ComplexContent(::std::boxed::Box<crate::xs::ComplexContent>),
        Variant2(
            ::std::boxed::Box<
                complex_type_model_items::complex_type_model_variants::Variant2,
            >,
        ),
    }
    impl ::core::convert::From<crate::xs::SimpleContent> for ComplexTypeModel {
        fn from(value: crate::xs::SimpleContent) -> Self {
            ComplexTypeModel::SimpleContent(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::ComplexContent> for ComplexTypeModel {
        fn from(value: crate::xs::ComplexContent) -> Self {
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
        Include(::std::boxed::Box<crate::xs::Include>),
        Import(::std::boxed::Box<crate::xs::Import>),
        Redefine(::std::boxed::Box<crate::xs::Redefine>),
        Override(::std::boxed::Box<crate::xs::Override>),
        Annotation(::std::boxed::Box<crate::xs::Annotation>),
    }
    impl ::core::convert::From<crate::xs::Include> for Composition {
        fn from(value: crate::xs::Include) -> Self {
            Composition::Include(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Import> for Composition {
        fn from(value: crate::xs::Import) -> Self {
            Composition::Import(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Redefine> for Composition {
        fn from(value: crate::xs::Redefine) -> Self {
            Composition::Redefine(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Override> for Composition {
        fn from(value: crate::xs::Override) -> Self {
            Composition::Override(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Annotation> for Composition {
        fn from(value: crate::xs::Annotation) -> Self {
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
        Unique(::std::boxed::Box<crate::xs::Unique>),
        Key(::std::boxed::Box<crate::xs::Key>),
        Keyref(::std::boxed::Box<crate::xs::Keyref>),
    }
    impl ::core::convert::From<crate::xs::Unique> for IdentityConstraint {
        fn from(value: crate::xs::Unique) -> Self {
            IdentityConstraint::Unique(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Key> for IdentityConstraint {
        fn from(value: crate::xs::Key) -> Self {
            IdentityConstraint::Key(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Keyref> for IdentityConstraint {
        fn from(value: crate::xs::Keyref) -> Self {
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
        Element(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalElement>),
        #[xelement(
            name = "group",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Group(#[xgroup] ::std::boxed::Box<crate::xs::types::GroupRef>),
        Choice(::std::boxed::Box<crate::xs::Choice>),
        Sequence(::std::boxed::Box<crate::xs::Sequence>),
        Any(::std::boxed::Box<crate::xs::Any>),
    }
    impl ::core::convert::From<crate::xs::types::LocalElement> for NestedParticle {
        fn from(value: crate::xs::types::LocalElement) -> Self {
            NestedParticle::Element(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::types::GroupRef> for NestedParticle {
        fn from(value: crate::xs::types::GroupRef) -> Self {
            NestedParticle::Group(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Choice> for NestedParticle {
        fn from(value: crate::xs::Choice) -> Self {
            NestedParticle::Choice(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Sequence> for NestedParticle {
        fn from(value: crate::xs::Sequence) -> Self {
            NestedParticle::Sequence(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Any> for NestedParticle {
        fn from(value: crate::xs::Any) -> Self {
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
        Element(#[xgroup] ::std::boxed::Box<crate::xs::types::LocalElement>),
        #[xelement(
            name = "group",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Group(#[xgroup] ::std::boxed::Box<crate::xs::types::GroupRef>),
        All(::std::boxed::Box<crate::xs::All>),
        Choice(::std::boxed::Box<crate::xs::Choice>),
        Sequence(::std::boxed::Box<crate::xs::Sequence>),
        Any(::std::boxed::Box<crate::xs::Any>),
    }
    impl ::core::convert::From<crate::xs::types::LocalElement> for Particle {
        fn from(value: crate::xs::types::LocalElement) -> Self {
            Particle::Element(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::types::GroupRef> for Particle {
        fn from(value: crate::xs::types::GroupRef) -> Self {
            Particle::Group(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::All> for Particle {
        fn from(value: crate::xs::All) -> Self {
            Particle::All(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Choice> for Particle {
        fn from(value: crate::xs::Choice) -> Self {
            Particle::Choice(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Sequence> for Particle {
        fn from(value: crate::xs::Sequence) -> Self {
            Particle::Sequence(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Any> for Particle {
        fn from(value: crate::xs::Any) -> Self {
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
        SimpleType(::std::boxed::Box<crate::xs::SimpleType>),
        ComplexType(::std::boxed::Box<crate::xs::ComplexType>),
        Group(::std::boxed::Box<crate::xs::Group>),
        AttributeGroup(::std::boxed::Box<crate::xs::AttributeGroup>),
    }
    impl ::core::convert::From<crate::xs::SimpleType> for Redefinable {
        fn from(value: crate::xs::SimpleType) -> Self {
            Redefinable::SimpleType(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::ComplexType> for Redefinable {
        fn from(value: crate::xs::ComplexType) -> Self {
            Redefinable::ComplexType(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Group> for Redefinable {
        fn from(value: crate::xs::Group) -> Self {
            Redefinable::Group(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::AttributeGroup> for Redefinable {
        fn from(value: crate::xs::AttributeGroup) -> Self {
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
        Redefinable(::std::boxed::Box<crate::xs::groups::Redefinable>),
        Element(::std::boxed::Box<crate::xs::Element>),
        Attribute(::std::boxed::Box<crate::xs::Attribute>),
        Notation(::std::boxed::Box<crate::xs::Notation>),
    }
    impl ::core::convert::From<crate::xs::groups::Redefinable> for SchemaTop {
        fn from(value: crate::xs::groups::Redefinable) -> Self {
            SchemaTop::Redefinable(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Element> for SchemaTop {
        fn from(value: crate::xs::Element) -> Self {
            SchemaTop::Element(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Attribute> for SchemaTop {
        fn from(value: crate::xs::Attribute) -> Self {
            SchemaTop::Attribute(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Notation> for SchemaTop {
        fn from(value: crate::xs::Notation) -> Self {
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
        Restriction(::std::boxed::Box<crate::xs::Restriction>),
        List(::std::boxed::Box<crate::xs::List>),
        Union(::std::boxed::Box<crate::xs::Union>),
    }
    impl ::core::convert::From<crate::xs::Restriction> for SimpleDerivation {
        fn from(value: crate::xs::Restriction) -> Self {
            SimpleDerivation::Restriction(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::List> for SimpleDerivation {
        fn from(value: crate::xs::List) -> Self {
            SimpleDerivation::List(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Union> for SimpleDerivation {
        fn from(value: crate::xs::Union) -> Self {
            SimpleDerivation::Union(::std::boxed::Box::new(value))
        }
    }
    pub mod simple_restriction_model_items {
        impl ::core::convert::From<crate::xs::Facet> for Child1 {
            fn from(value: crate::xs::Facet) -> Self {
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
            Facet(::std::boxed::Box<crate::xs::Facet>),
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
            ::std::boxed::Box<crate::xs::types::LocalSimpleType>,
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
        Group(#[xgroup] ::std::boxed::Box<crate::xs::types::GroupRef>),
        All(::std::boxed::Box<crate::xs::All>),
        Choice(::std::boxed::Box<crate::xs::Choice>),
        Sequence(::std::boxed::Box<crate::xs::Sequence>),
    }
    impl ::core::convert::From<crate::xs::types::GroupRef> for TypeDefParticle {
        fn from(value: crate::xs::types::GroupRef) -> Self {
            TypeDefParticle::Group(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::All> for TypeDefParticle {
        fn from(value: crate::xs::All) -> Self {
            TypeDefParticle::All(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Choice> for TypeDefParticle {
        fn from(value: crate::xs::Choice) -> Self {
            TypeDefParticle::Choice(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Sequence> for TypeDefParticle {
        fn from(value: crate::xs::Sequence) -> Self {
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
#[xelement(
    name = "all",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct All(#[xgroup] pub ::std::boxed::Box<crate::xs::types::All>);
impl ::core::convert::From<crate::xs::types::All> for All {
    fn from(value: crate::xs::types::All) -> Self {
        All(::std::boxed::Box::new(value))
    }
}
pub mod annotation_items {
    impl ::core::convert::From<crate::xs::Appinfo> for Annotation {
        fn from(value: crate::xs::Appinfo) -> Self {
            Annotation::Appinfo(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::Documentation> for Annotation {
        fn from(value: crate::xs::Documentation) -> Self {
            Annotation::Documentation(::std::boxed::Box::new(value))
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
        Appinfo(::std::boxed::Box<crate::xs::Appinfo>),
        Documentation(::std::boxed::Box<crate::xs::Documentation>),
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
#[xelement(
    name = "annotation",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Annotation {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xvalue(default)]
    #[builder(default)]
    pub annotation: ::std::vec::Vec<annotation_items::Annotation>,
}
pub mod any_items {
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
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "any",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Any {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "namespace", optional)]
    pub namespace: ::core::option::Option<
        ::std::boxed::Box<crate::xs::types::NamespaceList>,
    >,
    #[xattribute(name = "notNamespace", optional)]
    pub not_namespace: ::core::option::Option<::std::string::String>,
    #[xattribute(name = "processContents", optional)]
    pub process_contents: ::core::option::Option<any_items::ProcessContentsValue>,
    #[xattribute(name = "notQName", optional)]
    pub not_q_name: ::core::option::Option<
        ::std::boxed::Box<crate::xs::types::QnameList>,
    >,
    #[xattribute(name = "minOccurs", optional)]
    pub min_occurs: ::core::option::Option<usize>,
    #[xattribute(name = "maxOccurs", optional)]
    pub max_occurs: ::core::option::Option<::std::boxed::Box<crate::xs::types::AllNNI>>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
}
pub mod any_attribute_items {
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
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "anyAttribute",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct AnyAttribute {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "namespace", optional)]
    pub namespace: ::core::option::Option<
        ::std::boxed::Box<crate::xs::types::NamespaceList>,
    >,
    #[xattribute(name = "notNamespace", optional)]
    pub not_namespace: ::core::option::Option<::std::string::String>,
    #[xattribute(name = "processContents", optional)]
    pub process_contents: ::core::option::Option<
        any_attribute_items::ProcessContentsValue,
    >,
    #[xattribute(name = "notQName", optional)]
    pub not_q_name: ::core::option::Option<
        ::std::boxed::Box<crate::xs::types::QnameListA>,
    >,
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
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "appinfo",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Appinfo {
    #[xattribute(name = "source", optional)]
    pub source: ::core::option::Option<crate::xs::types::TargetNamespace>,
    #[xvalue(default)]
    #[builder(default)]
    pub child_0: ::std::vec::Vec<appinfo_items::Child0>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "assertion",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Assertion(#[xgroup] pub ::std::boxed::Box<crate::xs::types::Assertion>);
impl ::core::convert::From<crate::xs::types::Assertion> for Assertion {
    fn from(value: crate::xs::types::Assertion) -> Self {
        Assertion(::std::boxed::Box::new(value))
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
    name = "attribute",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Attribute(
    #[xgroup]
    pub ::std::boxed::Box<crate::xs::types::TopLevelAttribute>,
);
impl ::core::convert::From<crate::xs::types::TopLevelAttribute> for Attribute {
    fn from(value: crate::xs::types::TopLevelAttribute) -> Self {
        Attribute(::std::boxed::Box::new(value))
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
    name = "attributeGroup",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct AttributeGroup(
    #[xgroup]
    pub ::std::boxed::Box<crate::xs::types::NamedAttributeGroup>,
);
impl ::core::convert::From<crate::xs::types::NamedAttributeGroup> for AttributeGroup {
    fn from(value: crate::xs::types::NamedAttributeGroup) -> Self {
        AttributeGroup(::std::boxed::Box::new(value))
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
    name = "choice",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Choice(#[xgroup] pub ::std::boxed::Box<crate::xs::types::ExplicitGroup>);
impl ::core::convert::From<crate::xs::types::ExplicitGroup> for Choice {
    fn from(value: crate::xs::types::ExplicitGroup) -> Self {
        Choice(::std::boxed::Box::new(value))
    }
}
pub mod complex_content_items {
    impl ::core::convert::From<crate::xs::types::ComplexRestrictionType> for Child1 {
        fn from(value: crate::xs::types::ComplexRestrictionType) -> Self {
            Child1::Restriction(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::types::ExtensionType> for Child1 {
        fn from(value: crate::xs::types::ExtensionType) -> Self {
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
        Restriction(
            #[xgroup]
            ::std::boxed::Box<crate::xs::types::ComplexRestrictionType>,
        ),
        #[xelement(
            name = "extension",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Extension(#[xgroup] ::std::boxed::Box<crate::xs::types::ExtensionType>),
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
#[xelement(
    name = "complexContent",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct ComplexContent {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "mixed", optional)]
    pub mixed: ::core::option::Option<bool>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
    pub child_1: complex_content_items::Child1,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "complexType",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct ComplexType(
    #[xgroup]
    pub ::std::boxed::Box<crate::xs::types::TopLevelComplexType>,
);
impl ::core::convert::From<crate::xs::types::TopLevelComplexType> for ComplexType {
    fn from(value: crate::xs::types::TopLevelComplexType) -> Self {
        ComplexType(::std::boxed::Box::new(value))
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
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "defaultOpenContent",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct DefaultOpenContent {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "appliesToEmpty", optional)]
    pub applies_to_empty: ::core::option::Option<bool>,
    #[xattribute(name = "mode", optional)]
    pub mode: ::core::option::Option<default_open_content_items::ModeValue>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
    #[xelement(name = "any", namespace = "http://www.w3.org/2001/XMLSchema", group)]
    pub any: ::std::boxed::Box<crate::xs::types::Wildcard>,
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
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "documentation",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Documentation {
    #[xattribute(name = "source", optional)]
    pub source: ::core::option::Option<crate::xs::types::TargetNamespace>,
    #[xattribute(deferred = true, optional)]
    pub attribute_1: ::core::option::Option<
        ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
    >,
    #[xvalue(default)]
    #[builder(default)]
    pub child_0: ::std::vec::Vec<documentation_items::Child0>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "element",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Element(#[xgroup] pub ::std::boxed::Box<crate::xs::types::TopLevelElement>);
impl ::core::convert::From<crate::xs::types::TopLevelElement> for Element {
    fn from(value: crate::xs::types::TopLevelElement) -> Self {
        Element(::std::boxed::Box::new(value))
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
    name = "enumeration",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Enumeration(#[xgroup] pub ::std::boxed::Box<crate::xs::types::NoFixedFacet>);
impl ::core::convert::From<crate::xs::types::NoFixedFacet> for Enumeration {
    fn from(value: crate::xs::types::NoFixedFacet) -> Self {
        Enumeration(::std::boxed::Box::new(value))
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
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "explicitTimezone",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct ExplicitTimezone {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "value")]
    pub value: explicit_timezone_items::ValueValue,
    #[xattribute(name = "fixed", optional)]
    pub fixed: ::core::option::Option<bool>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
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
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "field",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Field {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "xpath")]
    pub xpath: field_items::XpathValue,
    #[xattribute(name = "xpathDefaultNamespace", optional)]
    pub xpath_default_namespace: ::core::option::Option<
        ::std::boxed::Box<crate::xs::types::XpathDefaultNamespace>,
    >,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "fractionDigits",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct FractionDigits(#[xgroup] pub ::std::boxed::Box<crate::xs::types::NumFacet>);
impl ::core::convert::From<crate::xs::types::NumFacet> for FractionDigits {
    fn from(value: crate::xs::types::NumFacet) -> Self {
        FractionDigits(::std::boxed::Box::new(value))
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
    name = "group",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Group(#[xgroup] pub ::std::boxed::Box<crate::xs::types::NamedGroup>);
impl ::core::convert::From<crate::xs::types::NamedGroup> for Group {
    fn from(value: crate::xs::types::NamedGroup) -> Self {
        Group(::std::boxed::Box::new(value))
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
#[xelement(
    name = "import",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Import {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "namespace", optional)]
    pub namespace: ::core::option::Option<crate::xs::types::TargetNamespace>,
    #[xattribute(name = "schemaLocation", optional)]
    pub schema_location: ::core::option::Option<crate::xs::types::TargetNamespace>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "include",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Include {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "schemaLocation")]
    pub schema_location: crate::xs::types::TargetNamespace,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "key",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Key(#[xgroup] pub ::std::boxed::Box<crate::xs::types::Keybase>);
impl ::core::convert::From<crate::xs::types::Keybase> for Key {
    fn from(value: crate::xs::types::Keybase) -> Self {
        Key(::std::boxed::Box::new(value))
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
        pub selector: ::std::boxed::Box<crate::xs::Selector>,
        #[xvalue(default)]
        #[builder(default)]
        pub field: ::std::vec::Vec<crate::xs::Field>,
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
#[xelement(
    name = "keyref",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Keyref {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "name", optional)]
    pub name: ::core::option::Option<::xmlity::LocalName<'static>>,
    #[xattribute(name = "ref", optional)]
    pub ref_: ::core::option::Option<crate::xs::types::QName>,
    #[xattribute(name = "refer", optional)]
    pub refer: ::core::option::Option<crate::xs::types::QName>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
    #[xvalue(default)]
    pub child_1: ::core::option::Option<keyref_items::Child1>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "length",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Length(#[xgroup] pub ::std::boxed::Box<crate::xs::types::NumFacet>);
impl ::core::convert::From<crate::xs::types::NumFacet> for Length {
    fn from(value: crate::xs::types::NumFacet) -> Self {
        Length(::std::boxed::Box::new(value))
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
#[xelement(
    name = "list",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct List {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "itemType", optional)]
    pub item_type: ::core::option::Option<crate::xs::types::QName>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
    #[xelement(
        name = "simpleType",
        namespace = "http://www.w3.org/2001/XMLSchema",
        group,
        optional
    )]
    pub simple_type: ::core::option::Option<
        ::std::boxed::Box<crate::xs::types::LocalSimpleType>,
    >,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "maxExclusive",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct MaxExclusive(#[xgroup] pub ::std::boxed::Box<crate::xs::types::Facet>);
impl ::core::convert::From<crate::xs::types::Facet> for MaxExclusive {
    fn from(value: crate::xs::types::Facet) -> Self {
        MaxExclusive(::std::boxed::Box::new(value))
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
    name = "maxInclusive",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct MaxInclusive(#[xgroup] pub ::std::boxed::Box<crate::xs::types::Facet>);
impl ::core::convert::From<crate::xs::types::Facet> for MaxInclusive {
    fn from(value: crate::xs::types::Facet) -> Self {
        MaxInclusive(::std::boxed::Box::new(value))
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
    name = "maxLength",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct MaxLength(#[xgroup] pub ::std::boxed::Box<crate::xs::types::NumFacet>);
impl ::core::convert::From<crate::xs::types::NumFacet> for MaxLength {
    fn from(value: crate::xs::types::NumFacet) -> Self {
        MaxLength(::std::boxed::Box::new(value))
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
    name = "minExclusive",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct MinExclusive(#[xgroup] pub ::std::boxed::Box<crate::xs::types::Facet>);
impl ::core::convert::From<crate::xs::types::Facet> for MinExclusive {
    fn from(value: crate::xs::types::Facet) -> Self {
        MinExclusive(::std::boxed::Box::new(value))
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
    name = "minInclusive",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct MinInclusive(#[xgroup] pub ::std::boxed::Box<crate::xs::types::Facet>);
impl ::core::convert::From<crate::xs::types::Facet> for MinInclusive {
    fn from(value: crate::xs::types::Facet) -> Self {
        MinInclusive(::std::boxed::Box::new(value))
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
    name = "minLength",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct MinLength(#[xgroup] pub ::std::boxed::Box<crate::xs::types::NumFacet>);
impl ::core::convert::From<crate::xs::types::NumFacet> for MinLength {
    fn from(value: crate::xs::types::NumFacet) -> Self {
        MinLength(::std::boxed::Box::new(value))
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
#[xelement(
    name = "notation",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Notation {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "name")]
    pub name: ::xmlity::LocalName<'static>,
    #[xattribute(name = "public", optional)]
    pub public: ::core::option::Option<::std::boxed::Box<crate::xs::types::Public>>,
    #[xattribute(name = "system", optional)]
    pub system: ::core::option::Option<crate::xs::types::TargetNamespace>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
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
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "openContent",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct OpenContent {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "mode", optional)]
    pub mode: ::core::option::Option<open_content_items::ModeValue>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
    #[xelement(
        name = "any",
        namespace = "http://www.w3.org/2001/XMLSchema",
        group,
        optional
    )]
    pub any: ::core::option::Option<::std::boxed::Box<crate::xs::types::Wildcard>>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "override",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Override {
    #[xattribute(name = "schemaLocation")]
    pub schema_location: crate::xs::types::TargetNamespace,
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
    #[xvalue(default)]
    #[builder(default)]
    pub schema_top: ::std::vec::Vec<crate::xs::groups::SchemaTop>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "pattern",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Pattern {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "value")]
    pub value: String,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
}
pub mod redefine_items {
    impl ::core::convert::From<crate::xs::Annotation> for Redefine {
        fn from(value: crate::xs::Annotation) -> Self {
            Redefine::Annotation(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::groups::Redefinable> for Redefine {
        fn from(value: crate::xs::groups::Redefinable) -> Self {
            Redefine::Redefinable(::std::boxed::Box::new(value))
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
        Annotation(::std::boxed::Box<crate::xs::Annotation>),
        Redefinable(::std::boxed::Box<crate::xs::groups::Redefinable>),
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
#[xelement(
    name = "redefine",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Redefine {
    #[xattribute(name = "schemaLocation")]
    pub schema_location: crate::xs::types::TargetNamespace,
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xvalue(default)]
    #[builder(default)]
    pub redefine: ::std::vec::Vec<redefine_items::Redefine>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "restriction",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Restriction {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "base", optional)]
    pub base: ::core::option::Option<crate::xs::types::QName>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
    pub simple_restriction_model: crate::xs::groups::SimpleRestrictionModel,
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
        pub default_open_content: crate::xs::DefaultOpenContent,
        #[xvalue(default)]
        #[builder(default)]
        pub annotation: ::std::vec::Vec<crate::xs::Annotation>,
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
        pub schema_top: crate::xs::groups::SchemaTop,
        #[xvalue(default)]
        #[builder(default)]
        pub annotation: ::std::vec::Vec<crate::xs::Annotation>,
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
#[xelement(
    name = "schema",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Schema {
    #[xattribute(name = "targetNamespace", optional)]
    pub target_namespace: ::core::option::Option<crate::xs::types::TargetNamespace>,
    #[xattribute(name = "version", optional)]
    pub version: ::core::option::Option<String>,
    #[xattribute(name = "finalDefault", optional)]
    pub final_default: ::core::option::Option<
        ::std::boxed::Box<crate::xs::types::FullDerivationSet>,
    >,
    #[xattribute(name = "blockDefault", optional)]
    pub block_default: ::core::option::Option<
        ::std::boxed::Box<crate::xs::types::BlockSet>,
    >,
    #[xattribute(name = "attributeFormDefault", optional)]
    pub attribute_form_default: ::core::option::Option<
        ::std::boxed::Box<crate::xs::types::FormChoice>,
    >,
    #[xattribute(name = "elementFormDefault", optional)]
    pub element_form_default: ::core::option::Option<
        ::std::boxed::Box<crate::xs::types::FormChoice>,
    >,
    #[xattribute(name = "defaultAttributes", optional)]
    pub default_attributes: ::core::option::Option<crate::xs::types::QName>,
    #[xattribute(name = "xpathDefaultNamespace", optional)]
    pub xpath_default_namespace: ::core::option::Option<
        ::std::boxed::Box<crate::xs::types::XpathDefaultNamespace>,
    >,
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xattribute(deferred = true, optional)]
    pub attribute_9: ::core::option::Option<
        ::std::boxed::Box<xmlity_ns_xml::attributes::Lang>,
    >,
    #[xvalue(default)]
    #[builder(default)]
    pub composition: ::std::vec::Vec<crate::xs::groups::Composition>,
    #[xvalue(default)]
    pub child_1: ::core::option::Option<schema_items::Child1>,
    #[xvalue(default)]
    #[builder(default)]
    pub child_2: ::std::vec::Vec<schema_items::Child2>,
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
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "selector",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Selector {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "xpath")]
    pub xpath: selector_items::XpathValue,
    #[xattribute(name = "xpathDefaultNamespace", optional)]
    pub xpath_default_namespace: ::core::option::Option<
        ::std::boxed::Box<crate::xs::types::XpathDefaultNamespace>,
    >,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "sequence",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Sequence(#[xgroup] pub ::std::boxed::Box<crate::xs::types::ExplicitGroup>);
impl ::core::convert::From<crate::xs::types::ExplicitGroup> for Sequence {
    fn from(value: crate::xs::types::ExplicitGroup) -> Self {
        Sequence(::std::boxed::Box::new(value))
    }
}
pub mod simple_content_items {
    impl ::core::convert::From<crate::xs::types::SimpleRestrictionType> for Child1 {
        fn from(value: crate::xs::types::SimpleRestrictionType) -> Self {
            Child1::Restriction(::std::boxed::Box::new(value))
        }
    }
    impl ::core::convert::From<crate::xs::types::SimpleExtensionType> for Child1 {
        fn from(value: crate::xs::types::SimpleExtensionType) -> Self {
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
        Restriction(
            #[xgroup]
            ::std::boxed::Box<crate::xs::types::SimpleRestrictionType>,
        ),
        #[xelement(
            name = "extension",
            namespace = "http://www.w3.org/2001/XMLSchema",
            allow_unknown_attributes = "any"
        )]
        Extension(#[xgroup] ::std::boxed::Box<crate::xs::types::SimpleExtensionType>),
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
#[xelement(
    name = "simpleContent",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct SimpleContent {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
    pub child_1: simple_content_items::Child1,
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
    pub ::std::boxed::Box<crate::xs::types::TopLevelSimpleType>,
);
impl ::core::convert::From<crate::xs::types::TopLevelSimpleType> for SimpleType {
    fn from(value: crate::xs::types::TopLevelSimpleType) -> Self {
        SimpleType(::std::boxed::Box::new(value))
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
#[xelement(
    name = "totalDigits",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct TotalDigits {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "value")]
    pub value: ::core::num::NonZeroUsize,
    #[xattribute(name = "fixed", optional)]
    pub fixed: ::core::option::Option<bool>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
}
pub mod union_items {
    impl ::core::convert::From<crate::xs::types::LocalSimpleType> for SimpleType {
        fn from(value: crate::xs::types::LocalSimpleType) -> Self {
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
        pub ::std::boxed::Box<crate::xs::types::LocalSimpleType>,
    );
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "union",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct Union {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "memberTypes", optional)]
    pub member_types: ::core::option::Option<
        crate::xs::types::List<crate::xs::types::QName>,
    >,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
    #[xvalue(default)]
    #[builder(default)]
    pub simple_type: ::std::vec::Vec<union_items::SimpleType>,
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "unique",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any"
)]
pub struct Unique(#[xgroup] pub ::std::boxed::Box<crate::xs::types::Keybase>);
impl ::core::convert::From<crate::xs::types::Keybase> for Unique {
    fn from(value: crate::xs::types::Keybase) -> Self {
        Unique(::std::boxed::Box::new(value))
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
}
#[derive(
    ::core::fmt::Debug,
    ::xmlity::Serialize,
    ::xmlity::Deserialize,
    ::bon::Builder,
    ::core::cmp::PartialEq,
    ::core::clone::Clone
)]
#[xelement(
    name = "whiteSpace",
    namespace = "http://www.w3.org/2001/XMLSchema",
    allow_unknown_attributes = "any",
    children_order = "strict"
)]
pub struct WhiteSpace {
    #[xattribute(name = "id", optional)]
    pub id: ::core::option::Option<String>,
    #[xattribute(name = "value")]
    pub value: white_space_items::ValueValue,
    #[xattribute(name = "fixed", optional)]
    pub fixed: ::core::option::Option<bool>,
    #[xvalue(default)]
    pub annotation: ::core::option::Option<crate::xs::Annotation>,
}
