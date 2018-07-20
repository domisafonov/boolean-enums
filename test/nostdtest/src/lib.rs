#![no_std]

#[macro_use] extern crate boolean_enums;

gen_boolean_enum!(TestEnum);
gen_boolean_enum!(pub PubTestEnum);
