#![feature(half_open_range_patterns)]
#![feature(exclusive_range_pattern)]

fn main() {
    match [5..4, 99..105, 43..44] {
        [_, 99..] => {},
        //~^ ERROR pattern requires 2 elements but array has 3
        //~| ERROR mismatched types
        _ => {},
    }
}
