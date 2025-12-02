// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![warn(bare_trait_objects)]
#![warn(clippy::pedantic)]

#![cfg_attr(not(feature = "std"), no_std)]

//! Generate your Yes/No enum with `gen_boolean_enum!`:
//! ```
//! # use boolean_enums::gen_boolean_enum;
//! #
//! gen_boolean_enum!(MyEnum);
//! ```
//!
//! It's From<bool> and Into<bool> and Not:
//! ```
//! # use boolean_enums::gen_boolean_enum;
//! #
//! # gen_boolean_enum!(MyEnum);
//! #
//! let flag = MyEnum::Yes;
//! let oflag = true.into();
//! assert_eq!(flag, oflag);
//!
//! if (!flag).into() {
//!     unreachable!()
//! }
//! ```
//!
//! To generate a public enum, you need to append `pub` to
//! the macro arguments:
//! ```
//! # use boolean_enums::gen_boolean_enum;
//! #
//! gen_boolean_enum!(pub MyEnum);
//! ```
//!
//! You can serialize and deserialize it with serde like a normal bool
//! (enabled by the `serde` feature).  For that, specify `serde`
//! before the enum name in `gen_boolean_enum!`:
//! ```rust
//! # use boolean_enums::gen_boolean_enum;
//! #
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
//! You can use boolean-enums in `no_std` crates by disabling the default `std`
//! feature:
//! ```toml,ignore
//! [dependencies.boolean-enums]
//! version = "^0.4.0"
//! default-features = false
//! ```
//!
//! # Examples
//! ```
//! # use boolean_enums::gen_boolean_enum;
//! #
//! gen_boolean_enum!(First);
//! gen_boolean_enum!(Second);
//! gen_boolean_enum!(Third);
//!
//! fn do_smth(flag1: First, flag2: Second, flag3: Third) {
//!     // …
//! }
//!
//! let first = First::Yes;
//! let second = Second::No;
//! let third = Third::Yes;
//!
//! do_smth(first, second, third);
//! ```
//! That compiles perfectly, but
//! ```rust,compile_fail
//! # use boolean_enums::gen_boolean_enum;
//! #
//! # gen_boolean_enum!(First);
//! # gen_boolean_enum!(Second);
//! # gen_boolean_enum!(Third);
//! #
//! # fn do_smth(flag1: First, flag2: Second, flag3: Third) {
//! #     // …
//! # }
//! #
//! # let first = First::Yes;
//! # let second = Second::No;
//! # let third = Third::Yes;
//! #
//! do_smth(first, third, second);
//! ```
//! fails to compile.

#[cfg(feature = "serde")]
extern crate serde;

#[cfg(feature = "std")]
#[doc(hidden)]
pub mod lstd {
    pub use std::*;
}

#[cfg(not(feature = "std"))]
#[doc(hidden)]
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
/// # use boolean_enums::gen_boolean_enum;
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
        $crate::_gen_boolean_enum_gen_enum!($name);
        $crate::_gen_boolean_enum_common!($name);
    );

    (pub $name:ident) => (
        $crate::_gen_boolean_enum_gen_enum!(pub $name);
        $crate::_gen_boolean_enum_common!($name);
    );

    (serde $name:ident) => (
        $crate::_gen_boolean_enum_gen_enum!($name);
        $crate::_gen_boolean_enum_common!($name);
        $crate::_gen_boolean_enum_serde!($name);
    );

    (pub serde $name:ident) => (
        $crate::_gen_boolean_enum_gen_enum!(pub $name);
        $crate::_gen_boolean_enum_common!($name);
        $crate::_gen_boolean_enum_serde!($name);
    );

    (serde pub $name:ident) => (
        $crate::gen_boolean_enum!(pub serde $name);
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
        #[allow(missing_docs)]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        enum $name {
            Yes,
            No
        }
    );
    (pub $name:ident) => (
        #[allow(missing_docs)]
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
