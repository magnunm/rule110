use std::{thread, time, env};

const ALIVE: &str = "▒";
const DEAD: &str = " ";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut start_world: Vec<bool> = Vec::new();
    for arg in args.iter() {
        if arg == &"1" {
            start_world.push(true);
        }
        else {
            start_world.push(false);
        }
    }

    let sleep_time = time::Duration::from_millis(50);

    print!("\x1B[2J"); // Clear terminal
    print!("\x1B[H"); // Move to top left
    println!("Simulation:");

    let mut sim = Simulation::new(start_world);
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
    world: Vec<bool>,
}

impl Simulation {
    pub fn new(initial_world: Vec<bool>) -> Simulation {
        return Simulation {
            world: initial_world,
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
        // TODO: For now the 110 rule is hard-coded. Allow arbitrary rules later.
        let current_world = &self.world;
        let mut next_world: Vec<bool> = Vec::new();
        let world_length = current_world.len();

        if world_length < 2 {
            panic!("Too short!");
        }

        next_world.push(rule_110(
            [current_world[world_length - 1], current_world[0], current_world[1]]
        ));

        for i in 1..(world_length - 1) {
            next_world.push(rule_110(
                [current_world[i - 1], current_world[i], current_world[i + 1]]
            ));
        }

        next_world.push(rule_110(
            [current_world[world_length - 2], current_world[world_length - 1], current_world[0]]
        ));

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

fn write_row(content: &str) {
    println!("{}", content);
}
