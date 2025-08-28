use std::collections::VecDeque;

const SEQ: [char; 3] = ['R', 'G', 'B'];

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2_q02_p1.txt");
    let mut balloons = BalloonQueue::new(&lines[0]);
    let mut _balloons = BalloonQueue::new("GRBGGGBBBRRRRRRRR");
    let mut bolts = 0;
    while !balloons.all_popped() {
        let fluffbolt = SEQ[bolts % 3];
        bolts += 1;
        balloons.shoot(fluffbolt);
    }
    println!("part 1 = {bolts}");

    let lines = aoclib::read_lines("input/everybody_codes_e2_q02_p2.txt");
    let mut balloons = BalloonQueue::new(&lines[0]);
    let mut _balloons = BalloonQueue::new("BBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGG");
    balloons.extend_circle(100);
    println!("part 2 = {}", balloons.play());

    let lines = aoclib::read_lines("input/everybody_codes_e2_q02_p3.txt");
    let mut balloons = BalloonDeque::new(&lines[0], 100000);
    println!("part 3 = {}", balloons.play());
}

struct BalloonDeque {
    first: VecDeque<char>,
    second: VecDeque<char>,
}

impl BalloonDeque {
    fn new<S: AsRef<str>>(input: S, repitition: usize) -> Self {
        let base = input.as_ref().chars().collect::<Vec<_>>();
        assert!(repitition % 2 == 0);
        let first = base.repeat(repitition / 2);
        let second = base.repeat(repitition / 2);
        Self {
            first: first.into(),
            second: second.into(),
        }
    }

    fn all_popped(&self) -> bool {
        self.first.is_empty() && self.second.is_empty()
    }

    fn play(&mut self) -> usize {
        let mut bolts = 0;
        while !self.all_popped() {
            let fluffbolt = SEQ[bolts % 3];
            bolts += 1;
            self.circle_shoot(fluffbolt);
        }
        bolts
    }

    fn circle_shoot(&mut self, fluffbolt: char) {
        let len = self.first.len() + self.second.len();
        let balloon = self.first.pop_front().unwrap();
        if len % 2 == 0 && fluffbolt == balloon {
            self.second.pop_front();
        }
        if self.first.len() < self.second.len() {
            self.first.push_back(self.second.pop_front().unwrap());
        }
    }
}

struct BalloonQueue {
    queue: Vec<char>,
}

impl BalloonQueue {
    fn new<S: AsRef<str>>(input: S) -> Self {
        Self {
            queue: input.as_ref().chars().rev().collect(),
        }
    }

    fn extend_circle(&mut self, times: usize) {
        self.queue = self.queue.repeat(times);
    }

    fn all_popped(&self) -> bool {
        self.queue.is_empty()
    }

    fn play(&mut self) -> usize {
        let mut bolts = 0;
        while !self.all_popped() {
            let fluffbolt = SEQ[bolts % 3];
            bolts += 1;
            self.circle_shoot(fluffbolt);
        }
        bolts
    }

    fn shoot(&mut self, fluffbolt: char) {
        let mut last = self.queue.pop().unwrap();
        while last == fluffbolt {
            if let Some(next) = self.queue.pop() {
                last = next;
            } else {
                return;
            }
        }
    }

    fn circle_shoot(&mut self, fluffbolt: char) {
        let len = self.queue.len();
        let last = self.queue.pop().unwrap();
        if last == fluffbolt && len % 2 == 0 {
            self.queue.remove(len / 2 - 1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2_10() {
        let mut balloons = BalloonQueue::new("BBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGG");
        balloons.extend_circle(10);
        assert_eq!(304, balloons.play());
    }

    #[test]
    fn test_part2_50() {
        let mut balloons = BalloonQueue::new("BBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGG");
        balloons.extend_circle(50);
        assert_eq!(1464, balloons.play());
    }

    #[test]
    fn test_part2_100() {
        let mut balloons = BalloonQueue::new("BBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGG");
        balloons.extend_circle(100);
        assert_eq!(2955, balloons.play());
    }

    #[test]
    fn test_part2deque_10() {
        let mut balloons = BalloonDeque::new("BBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGG", 10);
        assert_eq!(304, balloons.play());
    }

    #[test]
    fn test_part2deque_50() {
        let mut balloons = BalloonDeque::new("BBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGG", 50);
        assert_eq!(1464, balloons.play());
    }

    #[test]
    fn test_part2deque_100() {
        let mut balloons = BalloonDeque::new("BBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGG", 100);
        assert_eq!(2955, balloons.play());
    }
}
