use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn parse(input: &str) -> String {
    input.to_string()
}

#[aoc(day8, part1)]
fn part1(input: &str) -> String {
    let mut hsa: HashSet<(usize, usize)> = HashSet::new();
    let x_max = input.lines().map(|l| l.chars().count()).next().unwrap();
    let y_max = input.lines().count();
    let hm: HashMap<(usize, usize), char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| ((x, y), c))
        })
        .collect();

    for ((x, y), c) in hm.iter() {
        for ((x1, y1), _) in hm
            .iter()
            .filter(|&((x1, y1), c1)| (x1 != x) && (y1 != y) && c == c1)
        {
            let dx: isize = *x1 as isize - *x as isize;
            let dy: isize = *y1 as isize - *y as isize;

            // Add one forward
            let xa: isize = *x1 as isize + dx;
            let ya: isize = *y1 as isize + dy;
            if xa < 0 || ya < 0 || xa >= x_max as isize || ya >= y_max as isize {
                continue; // oob
            }
            hsa.insert((xa as usize, ya as usize));
        }
    }

    assert!(hsa.len() == 390);
    format!("{}", hsa.len())
}

#[aoc(day8, part2)]
fn part2(input: &str) -> String {
    let mut hsa: HashSet<(usize, usize)> = HashSet::new();
    let x_max = input.lines().map(|l| l.chars().count()).next().unwrap();
    let y_max = input.lines().count();
    let hm: HashMap<(usize, usize), char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| ((x, y), c))
        })
        .collect();

    for ((x, y), c) in hm.iter() {
        for ((x1, y1), _) in hm
            .iter()
            .filter(|&((x1, y1), c1)| (x1 != x) && (y1 != y) && c == c1)
        {
            let dx: isize = *x1 as isize - *x as isize;
            let dy: isize = *y1 as isize - *y as isize;

            // Forward
            let mut xa: isize = *x1 as isize + dx;
            let mut ya: isize = *y1 as isize + dy;
            while xa >= 0 && ya >= 0 && xa < x_max as isize && ya < y_max as isize {
                hsa.insert((xa as usize, ya as usize));
                xa += dx;
                ya += dy;
            }

            // Backward
            let mut xa: isize = *x1 as isize - dx;
            let mut ya: isize = *y1 as isize - dy;
            while xa >= 0 && ya >= 0 && xa < x_max as isize && ya < y_max as isize {
                hsa.insert((xa as usize, ya as usize));
                xa -= dx;
                ya -= dy;
            }
        }
    }

    format!("{}", hsa.len())
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
