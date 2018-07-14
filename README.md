# boolean-enums

Convenient macro to generate enums with Yes and No variants.

Useful in cases of multiple bool arguments:
```rust
#[macro_use] extern crate boolean_enums;

generate_boolean_enum!(First);
generate_boolean_enum!(Second);
generate_boolean_enum!(Third);

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
