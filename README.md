# boolean-enums

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/domisafonov/boolean-enums/Rust)](https://github.com/domisafonov/boolean-enums/actions/workflows/build.yml)
[![Crates.io Version](https://img.shields.io/crates/v/boolean-enums)](https://crates.io/crates/boolean-enums)
[![docs.rs](https://img.shields.io/docsrs/boolean-enums)](https://docs.rs/boolean-enums/)

Convenient macro to generate enums with Yes and No variants.
Supports `no_std`.

Useful in cases of multiple bool arguments:
```rust
use boolean_enums::gen_boolean_enum;

gen_boolean_enum!(First);
gen_boolean_enum!(Second);
gen_boolean_enum!(Third);

fn do_smth(flag1: First, flag2: Second, flag3: Third) {
    // …
}

fn main() {
    let first = First::Yes;
    let second = Second::No;
    let third = Third::Yes;

    do_smth(first, second, third); // compiles
    do_smth(first, third, second); // fails
}
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
