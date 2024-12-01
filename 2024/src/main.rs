use std::{collections::HashMap, sync::LazyLock};

mod day_1;
mod day_2;

static SOLUTIONS: LazyLock<HashMap<usize, fn() -> ()>> =
    LazyLock::new(|| HashMap::from([(1, day_1::run as fn()), (2, day_2::run as fn())]));

// TODO: add CLI for selecting days
fn main() {
    day_2::run();
}
