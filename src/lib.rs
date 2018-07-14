#![allow(unknown_lints)]
#![warn(bare_trait_objects)]
#![warn(clippy)]

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

#[macro_export]
macro_rules! _gen_boolean_enum_common {
    ($name:ident) => (
        impl From<bool> for $name {
            fn from(x: bool) -> $name {
                match x {
                    true => $name::Yes,
                    false => $name::No
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
            fn default() -> $name {
                bool::default().into()
            }
        }
    )
}

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

#[cfg(not(feature = "serde_derive"))]
#[macro_export]
macro_rules! _gen_boolean_enum_serde {
    ( $( $t:tt )* ) => (
        compile_error!("the \"serde\" feature is not enabled");
    )
}

#[cfg(feature = "serde_derive")]
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
