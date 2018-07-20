#![no_std]

#[macro_use] extern crate boolean_enums;

gen_boolean_enum!(serde SerdeTestEnum);
gen_boolean_enum!(pub serde PubSerdeTestEnum);
gen_boolean_enum!(serde pub SerdePubTestEnum);
