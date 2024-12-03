use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Lines = Vec<Vec<i32>>;

#[aoc_generator(day2)]
fn parse(input: &str) -> Lines {
    let mut o = vec![];
    for i in input.lines() {
        o.push(i.split(' ').map(|v| v.parse::<i32>().unwrap()).collect());
    }
    o
}

#[aoc(day2, part1)]
fn part1(input: &Lines) -> i32 {
    let mut answer = 0;
    for line in input {
        if check(line) {
            answer += 1
        }
    }
    answer
}

fn check(levels: &[i32]) -> bool {
    let mut levels_iter = levels.iter().tuple_windows().peekable();
    let Some((first, second)) = levels_iter.peek() else {
        return true;
    };
    let is_increasing = second > first;
    levels_iter.all(|(first, second)| {
        let diff = second.abs_diff(*first);
        (1..=3).contains(&diff) && (second > first) == is_increasing
    })
}

#[aoc(day2, part2)]
fn part2(input: &Lines) -> i32 {
    let mut answer = 0;
    for line in input {
        if check(line) {
            answer += 1;
            continue;
        }
        'itr: for ignore in 0..line.len() {
            let newline: Vec<_> = line
                .clone()
                .into_iter()
                .enumerate()
                .filter(|(f, _)| f != &ignore)
                .map(|(_, v)| v)
                .collect();
            if check(newline.as_slice()) {
                answer += 1;
                break 'itr;
            }
        }
    }

    answer
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("9 7 6 2 1")), 0);
        assert_eq!(part1(&parse("1 3 2 4 5")), 0);
        assert_eq!(part1(&parse("8 6 4 4 1")), 0);
        assert_eq!(part1(&parse("1 2 4 7 11")), 0);
        assert_eq!(part1(&parse("11 7 4 2 1")), 0);
        assert_eq!(part1(&parse("1 3 6 7 9")), 1);
        assert_eq!(part1(&parse("6 3 6 7 9")), 0);
        assert_eq!(part1(&parse("3 6 3 7 9")), 0);
    }

    const VALS: &str = indoc::indoc!(
        "48 46 47 49 51 54 56
        1 1 2 3 4 5
        1 2 3 4 5 5
        5 1 2 3 4 5
        1 4 3 2 1
        1 6 7 8 9
        1 2 3 4 3
        9 8 7 6 7
        7 10 8 10 11
        29 28 27 25 26 25 22 20
        95 88 86 84 83 82 79 76
        91 88 86 84 83 82 73 82"
    );

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("5 4 3 2 1")), 1);
        assert_eq!(part2(&parse("7 6 4 2 1")), 1);
        assert_eq!(part2(&parse("1 2 7 8 9")), 0);
        assert_eq!(part2(&parse("9 7 6 2 1")), 0);
        assert_eq!(part2(&parse("1 3 2 4 5")), 1);
        assert_eq!(part2(&parse("8 6 4 4 1")), 1);
        assert_eq!(part2(&parse("1 3 6 7 9")), 1);
        assert_eq!(part2(&parse("4 4 4 4 4")), 0);
        assert_eq!(part2(&parse("1 4 5 9 4")), 0);
        assert_eq!(part2(&parse("1 4 5 9 6")), 1);

        assert_eq!(part2(&parse(VALS)), 11);
    }
}
