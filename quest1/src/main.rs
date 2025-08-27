use std::collections::HashSet;

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
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
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
                let dir = bounce.next().unwrap();
                match dir {
                    Direction::Left => {
                        if pos > 0 {
                            pos -= 1;
                        } else {
                            pos += 1;
                        }
                    }
                    Direction::Right => {
                        if pos < self.cols - 1 {
                            pos += 1;
                        } else {
                            pos -= 1;
                        }
                    }
                }
            }
            // println!("row: {row:2} | {pos}");
        }
        let final_slot_number = pos / 2 + 1;
        0.max(final_slot_number * 2 - toss_slot_number)
        // println!("{toss_slot_number:2} | {final_slot_number:2} | {coins_won:2}");
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
