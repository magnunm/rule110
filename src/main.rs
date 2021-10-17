#![feature(destructuring_assignment)]
use std::{thread, time, env, fmt};

const ALIVE: &str = "▒";
const DEAD: &str = " ";
const SLEEP_MILLIS: u64 = 50;
const RULE_110_WOLFRAM_CODE: [bool; 8] = [false, true, true, false, true, true, true, false];

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut start_world: Vec<bool> = Vec::new();
    for arg in args[1..].iter() { // First arg is command
        start_world.push(arg == &"1");
    }

    print!("\x1B[2J\x1B[H"); // Clear terminal and move to top left

    let rule = Rule::new(RULE_110_WOLFRAM_CODE);
    let mut sim = Simulation::new(start_world, &rule);
    let sleep_time = time::Duration::from_millis(SLEEP_MILLIS);
    println!("{}", sim);
    thread::sleep(sleep_time);

    loop {
        sim.advance();
        println!("{}", sim);
        thread::sleep(sleep_time);
    }
}

struct Simulation<'a> {
    world: Vec<bool>,
    rule: &'a Rule,
}

impl<'a> Simulation<'a> {
    pub fn new(initial_world: Vec<bool>, rule: &Rule) -> Simulation {
        if initial_world.len() < 2 {
            panic!("World is too short!");
        }
        return Simulation {
            world: initial_world,
            rule,
        }
    }

    pub fn advance(&mut self) {
        self.world = self.next_world();
    }

    fn next_world(&self) -> Vec<bool> {
        let mut next_world: Vec<bool> = Vec::new();

        let world_start = [self.world[0]];
        let world_end = [self.world[self.world.len() - 1]];
        let mut cyclic_world = world_end.iter()
            .chain(self.world.iter())
            .chain(world_start.iter());

        let (mut prev, mut current, mut next) =
            (cyclic_world.next(), cyclic_world.next(), cyclic_world.next());

        while next.is_some() {
            next_world.push(
                self.rule.apply([*prev.unwrap(), *current.unwrap(), *next.unwrap()])
            );
            (prev, current, next) = (current, next, cyclic_world.next());
        }
        return next_world;
    }
}

impl fmt::Display for Simulation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let row: String = self.world.iter().map(|&v| { if v { ALIVE } else { DEAD }}).collect();
        write!(f, "{}", row)
    }
}

struct Rule {
    wolfram_code: [bool; 8]
}

impl Rule {
    pub fn new(wolfram_code: [bool; 8]) -> Rule {
        Rule { wolfram_code }
    }

    pub fn apply(&self, pattern: [bool; 3]) -> bool {
        match pattern {
            [true, true, true] => { return self.wolfram_code[0] },
            [true, true, false] => { return self.wolfram_code[1] },
            [true, false, true] => { return self.wolfram_code[2] },
            [true, false, false] => { return self.wolfram_code[3] },
            [false, true, true] => { return self.wolfram_code[4] },
            [false, true, false] => { return self.wolfram_code[5] },
            [false, false, true] => { return self.wolfram_code[6] },
            [false, false, false] => { return self.wolfram_code[7] },
        }
    }
}
