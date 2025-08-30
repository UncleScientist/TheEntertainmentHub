use std::str::FromStr;

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
