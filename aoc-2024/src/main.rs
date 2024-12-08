mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

use std::collections::HashMap;
use utils::{anyhow::Result, runner};

fn main() -> Result<()> {
    runner::run(HashMap::from([
        (1, (day_1::part_1 as fn(bool), day_1::part_2 as fn(bool))),
        (2, (day_2::part_1 as fn(bool), day_2::part_2 as fn(bool))),
        (3, (day_3::part_1 as fn(bool), day_3::part_2 as fn(bool))),
        (4, (day_4::part_1 as fn(bool), day_4::part_2 as fn(bool))),
        (5, (day_5::part_1 as fn(bool), day_5::part_2 as fn(bool))),
        (6, (day_6::part_1 as fn(bool), day_6::part_2 as fn(bool))),
        (7, (day_7::part_1 as fn(bool), day_7::part_2 as fn(bool))),
        // (8, (day_8::part_1 as fn(bool), day_6::part_2 as fn(bool))),
    ]))
}
