#![allow(unknown_lints)]
#![warn(bare_trait_objects)]
#![warn(clippy)]

//! Generate your Yes/No enum with gen_boolean_enum!:
//! ```
//! # #![cfg_attr(feature = "serde", feature(plugin))]
//! # #![cfg_attr(feature = "serde", plugin(interpolate_idents))]
//! #
//! # #[cfg(feature = "serde")]
//! # #[macro_use]
//! # extern crate serde_derive;
//! #
//! # #[macro_use] extern crate boolean_enums;
//! #
//! gen_boolean_enum!(MyEnum);
//! #
//! # fn main() {}
//! ```
//!
//! It's From<bool> and Into<bool> and Not:
//! ```
//! # #![cfg_attr(feature = "serde", feature(plugin))]
//! # #![cfg_attr(feature = "serde", plugin(interpolate_idents))]
//! #
//! # #[cfg(feature = "serde")]
//! # #[macro_use]
//! # extern crate serde_derive;
//! #
//! # #[macro_use] extern crate boolean_enums;
//! #
//! # gen_boolean_enum!(MyEnum);
//! #
//! # fn main() {
//! let flag = MyEnum::Yes;
//! let oflag = true.into();
//! assert_eq!(flag, oflag);
//!
//! if (!flag).into() {
//!     unreachable!()
//! }
//! # }
//! ```
//!
//! To generate a public enum, you need to append **pub** to
//! the macro arguments:
//! ```
//! # #![cfg_attr(feature = "serde", feature(plugin))]
//! # #![cfg_attr(feature = "serde", plugin(interpolate_idents))]
//! #
//! # #[cfg(feature = "serde")]
//! # #[macro_use]
//! # extern crate serde_derive;
//! #
//! # #[macro_use] extern crate boolean_enums;
//! #
//! gen_boolean_enum!(pub MyEnum);
//! #
//! # fn main() {}
//! ```
//!
//! You can serialize and deserialize it with serde like a normal bool
//! (enabled by the "serde" feature).  For that, first add serde_derive
//! and interpolate_idents to your Cargo.toml dependencies.  Then specify
//! **serde** before the enum name in gen_boolean_enum!:
//! ```rust,ignore
//! // required by the macro implementation
//! #![feature(plugin)]
//! #![plugin(interpolate_idents)]
//! #[macro_use] extern crate serde_derive;
//!
//! extern crate toml; // as an example serde format
//!
//! gen_boolean_enum!(serde MyEnum);
//! // or gen_boolean_enum!(pub serde MyEnum);
//!
//! #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
//! struct SomeStruct {
//!     flag: MyEnum
//! }
//!
//! // …
//!
//! let first = SomeStruct {
//!     flag: MyEnum::Yes
//! };
//! let string = toml::ser::to_string(&first).unwrap();
//! let second: SomeStruct = toml::de::from_str(&string).unwrap();
//!
//! assert_eq!(first, second);
//! ```

/// Generates enum with Yes and No variants.
///
/// # Examples
///
/// ```
/// # #![cfg_attr(feature = "serde", feature(plugin))]
/// # #![cfg_attr(feature = "serde", plugin(interpolate_idents))]
/// #
/// # #[cfg(feature = "serde")]
/// # #[macro_use]
/// # extern crate serde_derive;
/// #
/// # #[macro_use] extern crate boolean_enums;
/// #
/// gen_boolean_enum!(DoX);
///
/// // …
///
/// # fn main() {
/// let flag = DoX::Yes;
/// let mut other_flag = DoX::No;
///
/// if flag.into() {
///     other_flag = true.into();
/// }
///
/// assert_eq!(other_flag, DoX::Yes);
/// # }
/// ```
#[macro_export]
macro_rules! gen_boolean_enum {
    ($name:ident) => (
        _gen_boolean_enum_gen_enum!($name);
        _gen_boolean_enum_common!($name);
    );

    (pub $name:ident) => (
        _gen_boolean_enum_gen_enum!(pub $name);
        _gen_boolean_enum_common!($name);
    );

    (serde $name:ident) => (
        _gen_boolean_enum_gen_enum!(pub $name);
        _gen_boolean_enum_common!($name);
        _gen_boolean_enum_serde!($name);
    );

    (pub serde $name:ident) => (
        _gen_boolean_enum_gen_enum!(pub $name);
        _gen_boolean_enum_common!($name);
        _gen_boolean_enum_serde!($name);
    );

    (serde pub $name:ident) => (
        gen_boolean_enum!(pub serde $name);
    )
}

/// Implementation detail
#[macro_export]
macro_rules! _gen_boolean_enum_common {
    ($name:ident) => (
        impl From<bool> for $name {
            fn from(x: bool) -> Self {
                if x {
                    $name::Yes
                } else {
                    $name::No
                }
            }
        }

        impl Into<bool> for $name {
            fn into(self) -> bool {
                match self {
                    $name::Yes => true,
                    $name::No => false
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                $name::No
            }
        }

        impl ::std::ops::Not for $name {
            type Output = Self;
            fn not(self) -> Self::Output {
                if self.into() {
                    $name::No
                } else {
                    $name::Yes
                }
            }
        }
    )
}

/// Implementation detail
#[macro_export]
macro_rules! _gen_boolean_enum_gen_enum {
    ($name:ident) => (
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        enum $name {
            Yes,
            No
        }
    );
    (pub $name:ident) => (
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            Yes,
            No
        }
    );
}

/// Implementation detail
#[cfg(not(feature = "serde"))]
#[macro_export]
macro_rules! _gen_boolean_enum_serde {
    ( $( $t:tt )* ) => (
        compile_error!("the \"serde\" feature is not enabled");
    )
}

/// Implementation detail
#[cfg(feature = "serde")]
#[macro_export]
macro_rules! _gen_boolean_enum_serde {
    ($name:ident) => (
        impl ::serde::Serialize for $name {
            fn serialize<S>(
                &self,
                serializer: S
            ) -> ::std::result::Result<S::Ok, S::Error>
            where S: ::serde::Serializer {
                serializer.serialize_bool((*self).into())
            }
        }

        interpolate_idents! {
            impl<'de> ::serde::Deserialize<'de> for $name {
                fn deserialize<D>(
                    deserializer: D
                ) -> ::std::result::Result<$name, D::Error>
                where D: ::serde::Deserializer<'de> {
                    deserializer.deserialize_bool(
                        [ $name GenBooleanEnumSerde ]
                    )
                }
            }

            struct [ $name GenBooleanEnumSerde ];
            impl<'de> ::serde::de::Visitor<'de>
                    for [ $name GenBooleanEnumSerde ] {
                type Value = $name;

                fn expecting(
                    &self,
                    formatter: &mut ::std::fmt::Formatter
                ) -> ::std::fmt::Result {
                    formatter.write_str("a boolean value")
                }

                fn visit_bool<E>(
                    self,
                    value: bool
                ) -> ::std::result::Result<Self::Value, E>
                where E: ::serde::de::Error {
                    Ok($name::from(value))
                }
            }
        }
    )
}
