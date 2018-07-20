#![allow(unknown_lints)]
#![warn(bare_trait_objects)]
#![warn(clippy)]

#![cfg_attr(not(feature = "std"), no_std)]

//! Generate your Yes/No enum with gen_boolean_enum!:
//! ```
//! # #[macro_use] extern crate boolean_enums;
//! #
//! gen_boolean_enum!(MyEnum);
//! #
//! # fn main() {}
//! ```
//!
//! It's From<bool> and Into<bool> and Not:
//! ```
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
//! # #[macro_use] extern crate boolean_enums;
//! #
//! gen_boolean_enum!(pub MyEnum);
//! #
//! # fn main() {}
//! ```
//!
//! You can serialize and deserialize it with serde like a normal bool
//! (enabled by the "serde" feature).  For that, specify **serde**
//! before the enum name in gen_boolean_enum!:
//! ```rust
//! #[macro_use] extern crate boolean_enums;
//!
//! # #[cfg(feature = "serde")]
//! # {
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
//! # }
//! ```
//!
//! You can use boolean-enums in no_std crates by disabling the default "std"
//! feature:
//! ```toml,ignore
//! [dependencies.boolean-enums]
//! version = "^0.3.0"
//! default-features = false
//! ```
//!
//! # Examples
//! ```
//! #[macro_use] extern crate boolean_enums;
//!
//! gen_boolean_enum!(First);
//! gen_boolean_enum!(Second);
//! gen_boolean_enum!(Third);
//!
//! fn do_smth(flag1: First, flag2: Second, flag3: Third) {
//!     // …
//! }
//!
//! fn main() {
//!     let first = First::Yes;
//!     let second = Second::No;
//!     let third = Third::Yes;
//!
//!     do_smth(first, second, third);
//! }
//! ```
//! That compiles perfectly, but
//! ```rust,compile_fail
//! # #[macro_use] extern crate boolean_enums;
//! #
//! # gen_boolean_enum!(First);
//! # gen_boolean_enum!(Second);
//! # gen_boolean_enum!(Third);
//! #
//! # fn do_smth(flag1: First, flag2: Second, flag3: Third) {
//! #     // …
//! # }
//! #
//! # fn main() {
//! #     let first = First::Yes;
//! #     let second = Second::No;
//! #     let third = Third::Yes;
//! #
//! do_smth(first, third, second);
//! # }
//! ```
//! fails to compile.

#[cfg(feature = "serde")]
extern crate serde;

#[cfg(feature = "std")]
pub mod lstd {
    pub use std::*;
}

#[cfg(not(feature = "std"))]
pub mod lstd {
    pub use core::*;
}

#[cfg(feature = "serde")]
#[doc(hidden)]
pub use serde::*;

/// Generates enum with Yes and No variants.
///
/// # Examples
///
/// ```
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

#[doc(hidden)]
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

        impl $crate::lstd::ops::Not for $name {
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

#[doc(hidden)]
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

#[cfg(not(feature = "serde"))]
#[doc(hidden)]
#[macro_export]
macro_rules! _gen_boolean_enum_serde {
    ( $( $t:tt )* ) => (
        compile_error!("the \"serde\" feature is not enabled");
    )
}

#[cfg(feature = "serde")]
#[doc(hidden)]
#[macro_export]
macro_rules! _gen_boolean_enum_serde {
    ($name:ident) => (
        impl $crate::Serialize for $name {
            fn serialize<S>(
                &self,
                serializer: S
            ) -> $crate::lstd::result::Result<S::Ok, S::Error>
            where S: $crate::Serializer {
                serializer.serialize_bool((*self).into())
            }
        }

        impl<'de> $crate::Deserialize<'de> for $name {
            fn deserialize<D>(
                deserializer: D
            ) -> $crate::lstd::result::Result<$name, D::Error>
            where D: $crate::Deserializer<'de> {
                struct BooleanEnumVisitor;

                impl<'de> $crate::de::Visitor<'de>
                        for BooleanEnumVisitor {
                    type Value = $name;

                    fn expecting(
                        &self,
                        formatter: &mut $crate::lstd::fmt::Formatter
                    ) -> $crate::lstd::fmt::Result {
                        formatter.write_str("a boolean value")
                    }

                    fn visit_bool<E>(
                        self,
                        value: bool
                    ) -> $crate::lstd::result::Result<Self::Value, E>
                    where E: $crate::de::Error {
                        Ok($name::from(value))
                    }
                }

                deserializer.deserialize_bool(BooleanEnumVisitor)
            }
        }
    )
}
