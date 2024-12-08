use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day8)]
fn parse(input: &str) -> String {
    input.to_string()
}

#[aoc(day8, part1)]
fn part1(input: &str) -> String {
    let mut hsa: HashSet<(usize, usize)> = HashSet::new();
    let mut hm: HashMap<(usize, usize), char> = HashMap::new();
    let mut x_max = 0;
    let y_max = input.lines().count();
    for (y, line) in input.lines().enumerate() {
        x_max = line.chars().count();
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                hm.insert((x, y), c);
            }
        }
    }

    for frequency in hm.values() {
        for ((x, y), c) in hm.iter() {
            if frequency != c {
                continue;
            }
            for ((x1, y1), c1) in hm.iter() {
                if (x1, y1) == (x, y) || c != c1 {
                    continue;
                }
                let xa: isize = *x1 as isize - *x as isize + *x1 as isize;
                let ya: isize = *y1 as isize - *y as isize + *y1 as isize;
                if xa < 0 || ya < 0 {
                    continue;
                }
                if xa >= x_max as isize || ya >= y_max as isize {
                    continue;
                }
                hsa.insert((xa as usize, ya as usize));
            }
        }
    }

    format!("{}", hsa.len())
}

#[aoc(day8, part2)]
fn part2(input: &str) -> String {
    let mut hsa: HashSet<(usize, usize)> = HashSet::new();
    let mut hm: HashMap<(usize, usize), char> = HashMap::new();
    let mut x_max = 0;
    let y_max = input.lines().count();
    for (y, line) in input.lines().enumerate() {
        x_max = line.chars().count();
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                hm.insert((x, y), c);
            }
        }
    }

    for frequency in hm.values() {
        for ((x, y), c) in hm.iter() {
            if frequency != c {
                continue;
            }
            for ((x1, y1), c1) in hm.iter() {
                if (x1, y1) == (x, y) || c != c1 {
                    continue;
                }
                let dx: isize = *x1 as isize - *x as isize;
                let dy: isize = *y1 as isize - *y as isize;

                // Forward propagation
                let mut xa: isize = *x1 as isize + dx;
                let mut ya: isize = *y1 as isize + dy;
                while xa >= 0 && ya >= 0 && xa < x_max as isize && ya < y_max as isize {
                    hsa.insert((xa as usize, ya as usize));
                    xa += dx;
                    ya += dy;
                }

                // Backward propagation
                let mut xa: isize = *x1 as isize - dx;
                let mut ya: isize = *y1 as isize - dy;
                while xa >= 0 && ya >= 0 && xa < x_max as isize && ya < y_max as isize {
                    hsa.insert((xa as usize, ya as usize));
                    xa -= dx;
                    ya -= dy;
                }
            }
        }
    }

    let acc = hsa.len();
    format!("{}", acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let a = "......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....#.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.
";
        assert_eq!(
            part1(&parse(
                "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            )),
            a
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
