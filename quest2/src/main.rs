fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2_q02_p1.txt");
    let mut balloons = BalloonQueue::new(&lines[0]);
    let mut _balloons = BalloonQueue::new("GRBGGGBBBRRRRRRRR");
    let seq = ['R', 'G', 'B'];
    let mut bolts = 0;
    while !balloons.popped() {
        let fluffbolt = seq[bolts % 3];
        bolts += 1;
        balloons.shoot(fluffbolt);
    }
    println!("part 1 = {bolts}");
}

struct BalloonQueue {
    queue: Vec<char>,
    index: usize,
}

impl BalloonQueue {
    fn new<S: AsRef<str>>(input: S) -> Self {
        Self {
            queue: input.as_ref().chars().collect(),
            index: 0,
        }
    }

    fn popped(&self) -> bool {
        self.index >= self.queue.len()
    }

    fn shoot(&mut self, fluffbolt: char) {
        while self.index < self.queue.len() && self.queue[self.index] == fluffbolt {
            self.index += 1;
        }
        self.index += 1;
    }
}
