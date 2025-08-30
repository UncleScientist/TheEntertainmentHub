use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2_q03_p1.txt");
    let mut dice: Vec<Die> = lines.iter().map(|line| line.parse().unwrap()).collect();

    let mut score = 0;
    let mut rolls = 0;
    while score < 10_000 {
        rolls += 1;
        score += dice.iter_mut().map(|die| die.roll()).sum::<i64>();
    }
    println!("part 1 = {rolls}");

    let records = aoclib::read_text_records("input/everybody_codes_e2_q03_p2.txt");
    let mut dice: Vec<(usize, Die)> = records[0]
        .split('\n')
        .map(|line| (0, line.parse().unwrap()))
        .collect();
    let track: Vec<i64> = records[1]
        .chars()
        .map(|ch| (ch as u8 - b'0') as i64)
        .collect();
    let mut on_track = dice.len();
    let mut finish = Vec::new();
    while on_track > 0 {
        for die_pos in dice.iter_mut().filter(|(pos, _)| *pos < track.len()) {
            let roll = die_pos.1.roll();
            if roll == track[die_pos.0] {
                die_pos.0 += 1;
                if die_pos.0 >= track.len() {
                    on_track -= 1;
                    finish.push(format!("{}", die_pos.1.id));
                }
            }
        }
    }
    println!("part 2 = {}", finish.join(","));

    // let records = aoclib::read_text_records("input/test-part-3-2.txt");
    let records = aoclib::read_text_records("input/everybody_codes_e2_q03_p3.txt");
    let mut dice: Vec<Die> = records[0]
        .split('\n')
        .map(|line| line.parse().unwrap())
        .collect();

    let mut grid = HashMap::<(i64, i64), i64>::new();
    for (row, line) in records[1].split('\n').enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let val = (ch as u8 - b'0') as i64;
            grid.insert((row as i64, col as i64), val);
        }
    }
    /*
    let rows = records[1].split('\n').count();
    for row in 0..rows {
        for col in 0..100 {
            if let Some(val) = grid.get(&(row as i64, col as i64)) {
                print!("{val}");
            } else {
                break;
            }
        }
        println!();
    }
    */

    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    for die in dice.iter_mut() {
        let next_num = die.roll();
        let mut positions = grid
            .iter()
            .filter(|(_, num)| **num == next_num)
            .map(|(pos, _)| *pos)
            .collect::<Vec<_>>();
        while !positions.is_empty() {
            visited.extend(positions.iter().copied());
            let next_num = die.roll();
            let mut next_positions = Vec::new();
            let mut check_pos = HashSet::new();
            for pos in positions {
                check_pos.extend(&[
                    pos,
                    (pos.0 + 1, pos.1),
                    (pos.0 - 1, pos.1),
                    (pos.0, pos.1 + 1),
                    (pos.0, pos.1 - 1),
                ]);
            }

            for c in check_pos {
                if let Some(val) = grid.get(&c)
                    && next_num == *val
                {
                    next_positions.push(c);
                }
            }
            positions = next_positions;
        }
    }
    println!("part 3 = {}", visited.len());
}

#[derive(Debug)]
struct Die {
    id: usize,
    faces: Vec<i64>,
    seed: usize,
    roll_number: usize,
    pulse: usize,
    face: usize,
}

impl Die {
    fn roll(&mut self) -> i64 {
        let spin = self.roll_number * self.pulse;
        self.face = (self.face + spin) % self.faces.len();

        self.pulse += spin;
        self.pulse %= self.seed;
        self.pulse += 1 + self.roll_number + self.seed;

        self.roll_number += 1;

        self.faces[self.face]
    }
}

impl FromStr for Die {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rest) = s.split_once(": ").unwrap();
        let _id = id.parse().unwrap();

        let (faces, seed) = rest.split_once("] seed=").unwrap();
        let seed = seed.parse().unwrap();
        let open_bracket = faces.chars().position(|ch| ch == '[').unwrap();
        let faces = faces[open_bracket + 1..]
            .split(',')
            .map(|face| face.parse().unwrap())
            .collect::<Vec<_>>();

        let pulse = seed;
        let roll_number = 1;

        Ok(Self {
            id: _id,
            faces,
            seed,
            roll_number,
            pulse,
            face: 0,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_die() {
        let mut die: Die = "1: faces=[1,2,4,-1,5,7,9] seed=3".parse().unwrap();
        assert_eq!(-1, die.roll());
    }

    #[test]
    fn test_example_100_rolls() {
        let mut die: Die = "1: faces=[1,2,4,-1,5,7,9] seed=3".parse().unwrap();
        for _ in 0..99 {
            die.roll();
        }
        let last_roll = die.roll();
        assert_eq!(7, last_roll);
        assert_eq!(106, die.pulse);
    }
}
