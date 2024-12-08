#![feature(type_alias_impl_trait)]
#![feature(slice_split_once)]
#![feature(allocator_api)]
#![feature(iter_map_windows)]
mod day8;
// mod day7;
// mod day6;
// mod day5;
// mod day4;
// mod day3;
// mod day2;
// mod day1;

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

aoc_lib! { year = 2024 }

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
