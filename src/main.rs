use std::{thread, time, env};

const ALIVE: &str = "▒";
const DEAD: &str = " ";
const SLEEP_MILLIS: u64 = 50;
const RULE_110_WOLFRAM_CODE: [bool; 8] = [false, true, true, false, true, true, true, false];

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut start_world: Vec<bool> = Vec::new();
    for arg in args[1..].iter() { // First arg is command
        if arg == &"1" {
            start_world.push(true);
        }
        else {
            start_world.push(false);
        }
    }

    print!("\x1B[2J"); // Clear terminal
    print!("\x1B[H"); // Move to top left
    println!("Simulation:");

    let rule = Rule::new(RULE_110_WOLFRAM_CODE);
    let mut sim = Simulation::new(start_world, &rule);
    let sleep_time = time::Duration::from_millis(SLEEP_MILLIS);
    sim.paint();
    thread::sleep(sleep_time);

    loop {
        sim.next_state();
        sim.paint();
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

    // TODO: Replace by implement of fmt display?
    pub fn paint(&mut self) {
        let row: String = self.world.iter().map(|&v| { if v { ALIVE } else { DEAD }}).collect();
        write_row(&row);
    }

    pub fn next_state(&mut self) {
        self.world = self.next_world();
    }

    fn next_world(&self) -> Vec<bool> {
        let current_world = &self.world;
        let mut next_world: Vec<bool> = Vec::new();
        let world_length = current_world.len();

        next_world.push(
            self.rule.apply(
                [current_world[world_length - 1], current_world[0], current_world[1]]
            )
        );

        for i in 1..(world_length - 1) {
            next_world.push(
                self.rule.apply(
                    [current_world[i - 1], current_world[i], current_world[i + 1]]
                )
            );
        }

        next_world.push(
            self.rule.apply(
                [current_world[world_length - 2], current_world[world_length - 1], current_world[0]]
            )
        );

        return next_world;
    }
}

    }
}

fn write_row(content: &str) {
    println!("{}", content);
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
