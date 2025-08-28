const SEQ: [char; 3] = ['R', 'G', 'B'];

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2_q02_p1.txt");
    let mut balloons = BalloonQueue::new(&lines[0]);
    let mut _balloons = BalloonQueue::new("GRBGGGBBBRRRRRRRR");
    let mut bolts = 0;
    while !balloons.popped() {
        let fluffbolt = SEQ[bolts % 3];
        bolts += 1;
        balloons.shoot(fluffbolt);
    }
    println!("part 1 = {bolts}");

    let lines = aoclib::read_lines("input/everybody_codes_e2_q02_p2.txt");
    let mut balloons = BalloonQueue::new(&lines[0]);
    let mut _balloons = BalloonQueue::new("BBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGG");
    balloons.extend_circle(100);
    let mut bolts = 0;
    while !balloons.popped() {
        let fluffbolt = SEQ[bolts % 3];
        bolts += 1;
        balloons.circle_shoot(fluffbolt);
    }
    println!("part 2 = {bolts}");
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

    fn extend_circle(&mut self, times: usize) {
        self.queue = self.queue.repeat(times);
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

    fn circle_shoot(&mut self, fluffbolt: char) {
        let len = self.queue.len();
        if self.queue[0] == fluffbolt && len % 2 == 0 {
            self.queue.remove(len / 2);
        }
        self.queue.remove(0);
    }
}
