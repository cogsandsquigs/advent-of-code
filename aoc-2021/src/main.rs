mod day_1;
mod day_10;
mod day_11;
mod day_9;

use std::collections::HashMap;
use utils::{anyhow::Result, runner};

fn main() -> Result<()> {
    runner::run(HashMap::from([
        (1, (day_1::part_1 as fn(bool), day_1::part_2 as fn(bool))),
        (9, (day_9::part_1 as fn(bool), day_9::part_2 as fn(bool))),
        (10, (day_10::part_1 as fn(bool), day_10::part_2 as fn(bool))),
        (11, (day_11::part_1 as fn(bool), day_11::part_2 as fn(bool))),
    ]))
}
