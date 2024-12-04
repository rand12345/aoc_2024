use aoc_runner_derive::{aoc, aoc_generator};

struct Map {
    lines: String,
    raw: String,
    w: usize,
    d: usize,
}
#[aoc_generator(day4)]
fn parse(input: &str) -> Map {
    let w = input.lines().next().map_or(0, |l| l.len() + 2);
    let d = input.lines().count() + 1;
    let mut output = String::with_capacity((w + 1) * (d + 2));

    output.push_str(&"#".repeat(w)); // Top border
    input
        .lines()
        .for_each(|line| output.push_str(&format!("#{}#", line)));
    output.push_str(&"#".repeat(w));

    Map {
        lines: output,
        raw: input.to_string(),
        w,
        d,
    }
}

static COORDS: &[(isize, isize)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[aoc(day4, part1)]
fn part1(map: &Map) -> String {
    let scan = |seek, c| (map.lines.chars().nth(seek) == Some(c));

    let mut acc = 0;
    for (y, x) in (1..=map.d).flat_map(|y| (1..map.w).map(move |x| (y, x))) {
        let offset = y * map.w + x;
        if map.lines.as_bytes().get(offset) == Some(&b'X') {
            acc += COORDS
                .iter()
                .filter_map(|(x, y)| {
                    let seek = offset as isize;
                    ['M', 'A', 'S'].iter().try_fold(seek, |mut seek, &c| {
                        seek += y * map.w as isize + x;
                        scan(seek as usize, c).then_some(seek)
                    })
                })
                .count();
        }
    }

    format!("{}", acc)
}

#[aoc(day4, part2)]
fn part2(map: &Map) -> String {
    let mut acc = 0;
    let lines: Vec<Vec<char>> = map.raw.lines().map(|l| l.chars().collect()).collect();
    for r in 1..lines.len() - 1 {
        for c in 1..lines[0].len() - 1 {
            if lines[r][c] == 'A' {
                let left = c - 1;
                let right = c + 1;
                let check = [
                    lines[r + 1][right],
                    lines[r - 1][left],
                    lines[r - 1][right],
                    lines[r + 1][left],
                ];

                if check[0] != check[1]
                    && check.iter().filter(|v| **v == 'S').count() == 2
                    && check.iter().filter(|v| **v == 'M').count() == 2
                {
                    acc += 1
                }
            }
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
                "....XXMAS.
    .SAMXMS...
    ...S..A...
    ..A.A.MS.X
    XMASAMX.MM
    X.....XA.A
    S.S.S.S.SS
    .A.A.A.A.A
    ..M.M.M.MM
    .X.X.XMASX"
            )),
            "18"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."
            )),
            "9"
        );

        // 134 answer is too low
    }
}
