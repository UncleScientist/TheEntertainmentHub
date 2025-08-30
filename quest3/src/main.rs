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
}

#[derive(Debug)]
struct Die {
    _id: usize,
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
            _id,
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
