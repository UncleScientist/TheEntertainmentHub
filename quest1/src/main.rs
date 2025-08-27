use std::collections::HashSet;

use aoclib::{CombinationsOf, Permutations};

fn main() {
    let records = aoclib::read_text_records("input/everybody_codes_e2_q01_p1.txt");
    // let records = aoclib::read_text_records("input/test-part-1.txt");
    let gridlines = records[0].split('\n').collect::<Vec<_>>();
    let grid = Grid::new(&gridlines);

    println!(
        "part 1 = {}",
        records[1]
            .split('\n')
            .enumerate()
            .map(|(idx, coin)| {
                let seq = Direction::sequence(coin);
                grid.toss_coin(&seq, idx as i64 * 2)
            })
            .sum::<i64>()
    );

    let records = aoclib::read_text_records("input/everybody_codes_e2_q01_p2.txt");
    let gridlines = records[0].split('\n').collect::<Vec<_>>();
    let grid = Grid::new(&gridlines);

    let mut max_total = 0;
    for coin in records[1].split('\n') {
        let mut pos = 0;
        let mut max_score = 0;
        while pos < grid.cols {
            max_score = max_score.max(grid.toss_coin(&Direction::sequence(coin), pos));
            pos += 2;
        }
        max_total += max_score;
    }
    println!("part 2 = {max_total}");

    let records = aoclib::read_text_records("input/everybody_codes_e2_q01_p3.txt");
    // let records = aoclib::read_text_records("input/test-part-3-1.txt");
    let gridlines = records[0].split('\n').collect::<Vec<_>>();
    let grid = Grid::new(&gridlines);

    let slots = (0..(grid.cols + 1) / 2).map(|c| c * 2).collect::<Vec<_>>();
    let mut coins = Vec::new();
    for coin in records[1].split('\n') {
        let seq = Direction::sequence(coin);
        let mut won = Vec::new();
        for slot in &slots {
            won.push(grid.toss_coin(&seq, *slot));
        }
        coins.push(Coin { _seq: seq, won });
    }
    let mut min_score = i64::MAX;
    let mut max_score = 0;
    for combo in slots.combinations_of(coins.len()) {
        for perm in combo.permutations() {
            let score = perm
                .iter()
                .map(|p| (p / 2) as usize)
                .enumerate()
                .map(|(coin, index)| coins[coin].won[index])
                .sum::<i64>();
            min_score = min_score.min(score);
            max_score = max_score.max(score);
        }
    }
    println!("part 3 = {min_score} {max_score}");
}

#[derive(Debug)]
struct Coin {
    _seq: Vec<Direction>,
    won: Vec<i64>,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn adjust(&self, mut pos: i64, low: i64, high: i64) -> i64 {
        match self {
            Self::Left => {
                if pos > low {
                    pos -= 1;
                } else {
                    pos += 1;
                }
            }
            Self::Right => {
                if pos < high - 1 {
                    pos += 1;
                } else {
                    pos -= 1;
                }
            }
        }
        pos
    }
}

impl Direction {
    fn sequence<S: AsRef<str>>(seq: S) -> Vec<Self> {
        seq.as_ref()
            .chars()
            .map(|ch| if ch == 'R' { Self::Right } else { Self::Left })
            .collect()
    }
}

#[derive(Debug)]
struct Grid {
    grid: HashSet<(i64, i64)>,
    rows: i64,
    cols: i64,
}

impl Grid {
    fn new(data: &[&str]) -> Self {
        let mut rows = 0;
        let mut cols = 0;
        let mut grid = HashSet::new();
        for (row, line) in data.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '*' {
                    grid.insert((row as i64, col as i64));
                    rows = rows.max(row as i64 + 1);
                    cols = cols.max(col as i64 + 1);
                }
            }
        }
        Self { grid, rows, cols }
    }

    // pos: actual position in grid hash, not the coin toss slot number
    fn toss_coin(&self, seq: &[Direction], mut pos: i64) -> i64 {
        let toss_slot_number = pos / 2 + 1;
        let mut bounce = seq.iter();

        for row in 0..self.rows {
            if self.grid.contains(&(row, pos)) {
                pos = bounce.next().unwrap().adjust(pos, 0, self.cols);
            }
        }
        let final_slot_number = pos / 2 + 1;
        0.max(final_slot_number * 2 - toss_slot_number)
    }

    fn _print(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.grid.contains(&(row, col)) {
                    print!("*");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
