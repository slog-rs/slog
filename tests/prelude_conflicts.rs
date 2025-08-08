#![allow(unused_variables)]
use self::conflicting::{FnValue, Serde};
// this import is supposed to be unused
#[rustversion::attr(since(1.81), expect(unused_imports))]
#[rustversion::attr(before(1.81), allow(unused_imports))]
use slog::prelude::*;

#[allow(dead_code)]
mod conflicting {
    pub struct Serde(pub i32);
    pub struct FnValue(pub i32);
}

#[test]
fn conflicts_serde() {
    let x = Serde(3);
    let x: conflicting::Serde = x;
    let _ = x;
}

#[test]
fn conflicts_fn_value() {
    // explicit type needed due to dependency_on_unit_never_type_fallback
    let x = slog::FnValue::<(), _>(|x| panic!());
    let x = FnValue(3);
    let x: conflicting::FnValue = x;
}
