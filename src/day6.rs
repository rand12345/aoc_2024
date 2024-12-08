use aoc_runner_derive::{aoc, aoc_generator};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    collections::HashSet,
    io::{self},
    ops::Sub,
    sync::mpsc::{self, Sender},
    thread,
    time::Duration,
};
#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!(),
        }
    }
}

impl From<Direction> for char {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }
}

#[derive(Clone)]
pub struct AsciiGrid {
    grid: Vec<Vec<char>>,
    start: (usize, usize),
}

impl std::fmt::Display for AsciiGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        self.grid.iter().for_each(|row| {
            output.push_str(&row.iter().collect::<String>());
            output.push('\n');
        });
        write!(f, "{}", output)
    }
}

impl AsciiGrid {
    pub fn new(input: &str) -> Self {
        let mut start = (0, 0);

        let grid: Vec<Vec<char>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == '^' {
                            start = (x, y);
                            '.'
                        } else {
                            c
                        }
                    })
                    .collect()
            })
            .collect();
        AsciiGrid { grid, start }
    }

    fn walk_map(
        &mut self,
        mut current_x: usize,
        mut current_y: usize,
        tx: Sender<String>,
        print_map: bool,
    ) -> Option<HashSet<(usize, usize)>> {
        let mut seen = vec![vec![[false; 4]; self.grid[0].len()]; self.grid.len()];
        let mut count: HashSet<(usize, usize)> = HashSet::new();
        let mut direction = Direction::Up;

        loop {
            if seen[current_y][current_x][direction as usize] {
                // println!("Loop detected");
                return None;
            }
            seen[current_y][current_x][direction as usize] = true;

            self.grid[current_y][current_x] = direction.into();
            let (next_x, next_y) = match direction {
                Direction::Up => (current_x, current_y.sub(1)),
                Direction::Down => (current_x, current_y + 1),
                Direction::Left => (current_x.sub(1), current_y),
                Direction::Right => (current_x + 1, current_y),
            };

            if next_y >= self.grid.len() || next_x >= self.grid[0].len() {
                // println!("Off map");
                return Some(count);
            }
            if self.grid[next_y][next_x] == '#' {
                direction = Direction::from((direction as usize + 1) % 4);
            } else {
                (current_x, current_y) = (next_x, next_y);
                count.insert((current_x, current_y));
            }

            if print_map {
                tx.send(self.to_string()).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        }
    }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> String {
    input.to_string()
}

#[aoc(day6, part1)]
fn part1(input: &str) -> String {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut grid = AsciiGrid::new(input);

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        while let Ok(updated_grid) = rx.recv() {
            terminal
                .draw(|f| {
                    let size = f.area();
                    let block = Block::default()
                        .borders(Borders::ALL)
                        .title("Day 6: Guard Gallivant Pt1");
                    let grid_str = updated_grid;
                    let paragraph = Paragraph::new(grid_str).block(block);
                    f.render_widget(paragraph, size);
                })
                .unwrap();
        }
    });

    let count = grid
        .walk_map(grid.start.0, grid.start.1, tx.clone(), true)
        .unwrap();

    format!("{}", count.len())
}

#[aoc(day6, part2)]
fn part2(input: &str) -> String {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut grid = AsciiGrid::new(input);

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        while let Ok(updated_grid) = rx.recv() {
            terminal
                .draw(|f| {
                    let size = f.area();
                    let block = Block::default()
                        .borders(Borders::ALL)
                        .title("Day 6: Guard Gallivant Pt2");
                    let grid_str = updated_grid;
                    let paragraph = Paragraph::new(grid_str).block(block);
                    f.render_widget(paragraph, size);
                })
                .unwrap();
        }
    });

    let count = grid
        .walk_map(grid.start.0, grid.start.1, tx.clone(), false)
        .unwrap();
    let count = count
        .iter()
        .filter(|(x, y)| {
            grid.grid[*y][*x] = '#';
            let this = grid
                .walk_map(grid.start.0, grid.start.1, tx.clone(), true)
                .is_none();
            grid.grid[*y][*x] = '.';
            this
        })
        .count();

    format!("{}", count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            )),
            "41"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            )),
            "6"
        );
    }
}
