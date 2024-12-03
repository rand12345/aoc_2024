use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day3)]
fn parse(input: &str) -> String {
    input.to_string()
}

#[aoc(day3, part1)]
fn part1(input: &str) -> String {
    let acc = check(input);
    format!("{}", acc)
}

fn check(input: &str) -> usize {
    let mut acc = 0usize;
    for split in input.split("mul(") {
        if let Some(end) = split.find(')') {
            let numbers = &split[..end];
            let valid_numbers: Vec<usize> = numbers
                .split(',')
                .filter_map(|s| s.trim().parse::<usize>().ok().filter(|&x| x < 1000))
                .collect();

            if valid_numbers.len() != 2 {
                continue;
            }

            let product = valid_numbers.iter().take(2).product::<usize>();
            acc += product;
        }
    }
    acc
}

#[aoc(day3, part2)]
fn part2(input: &str) -> String {
    let mut acc = 0usize;
    for (idx, line) in input.split("don't()").enumerate() {
        if idx == 0 {
            acc += check(line);
            continue;
        }

        for (j, sec) in line.split("do()").enumerate() {
            if j == 0 {
                continue;
            }
            acc += check(sec);
        }
    }

    format!("{}", acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))mul(32,12,2)"
            )),
            "161"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            )),
            "48"
        );
    }
}
