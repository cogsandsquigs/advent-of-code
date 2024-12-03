mod day_1;
mod day_2;
mod day_3;

use std::collections::HashMap;
use utils::{anyhow::Result, runner};

fn main() -> Result<()> {
    runner::run(HashMap::from([
        (1, (day_1::part_1 as fn(bool), day_1::part_2 as fn(bool))),
        (2, (day_2::part_1 as fn(bool), day_2::part_2 as fn(bool))),
        (3, (day_3::part_1 as fn(bool), day_3::part_2 as fn(bool))),
    ]))
}
