use std::{thread, time};
use termion::{clear, cursor};

const WORLD_LENGTH: usize = 17;
const ALIVE: &str = "▒";
const DEAD: &str = " ";
const START_WORLD: [bool; WORLD_LENGTH] = [false, false, false, true, true, true, false, false, true, true, false, false, true, true, false, true, false];

fn main() {
    let sleep_time = time::Duration::from_millis(100);

    print!("{}{}Simulation:\n", clear::All, cursor::Goto(1,1));

    let mut sim = Simulation::new(START_WORLD, 2);
    sim.paint();
    thread::sleep(sleep_time);

    for _ in 0..100 {
        sim.next_state();
        sim.paint();
        thread::sleep(sleep_time);
    }

    print!("\n");
    println!("Simulation done.")
}


struct Simulation {
    world: [bool; WORLD_LENGTH],
    offset: usize,
}

impl Simulation {
    pub fn new(initial_world: [bool; WORLD_LENGTH], offset: usize) -> Simulation {
        return Simulation {
            world: initial_world,
            offset,
        }
    }

    // TODO: Replace by implement of fmt display?
    pub fn paint(&mut self) {
        let row: String = self.world.iter().map(|&v| { if v { ALIVE } else { DEAD }}).collect();
        write_row(&row, self.offset);
        self.offset += 1;
    }

    pub fn next_state(&mut self) {
        self.world = self.next_world();
    }

    fn next_world(&self) -> [bool; WORLD_LENGTH] {
        // TODO: For now the 110 rule is hard-coded. Allow arbitrary rules later.
        let current_world = self.world;
        let mut next_world: [bool; WORLD_LENGTH] = [false; WORLD_LENGTH];

        next_world[0] = rule_110(
            [current_world[WORLD_LENGTH - 1], current_world[0], current_world[1]]
        );

        for i in 1..(WORLD_LENGTH - 1) {
            next_world[i] = rule_110(
                [current_world[i - 1], current_world[i], current_world[i + 1]]
            );
        }

        next_world[WORLD_LENGTH - 1] = rule_110(
            [current_world[WORLD_LENGTH - 2], current_world[WORLD_LENGTH - 2], current_world[0]]
        );

        return next_world;
    }
}

fn rule_110(pattern: [bool; 3]) -> bool {
    match pattern {
        [true, true, true] => (),
        [true, true, false] => { return true; },
        [true, false, true] => { return true; },
        [true, false, false] => (),
        [false, true, true] => { return true; },
        [false, true, false] => { return true; },
        [false, false, true] => { return true; },
        [false, false, false] => (),
    }
    return false;
}

fn write_row(content: &str, row: usize) {
    println!("{}", content);
}
