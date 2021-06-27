use std::io;
use rand::Rng;

enum State {
	Closed,
	Open
}

struct Door {
	state: State,
	has_prize: bool,
	is_selected: bool
}

struct Contestant {
	choice: usize,
	changes_door: bool
}

impl Door {
	fn new() -> Door {
		Door {
			state: State::Closed,
			has_prize: false,
			is_selected: false
		}
	}
	fn is_open(&self) -> bool {
		match self.state {
			State::Open		=> true,
			State::Closed	=> false
		}
	}
	fn open(&mut self) {
		self.state = State::Open;
	}
}

impl Contestant {
	fn new_alice() -> Contestant {
		Contestant {
			choice: 1,
			changes_door: false
		}
	}

	fn new_bob() -> Contestant {
		Contestant {
			choice: 1,
			changes_door: true
		}
	}
}

fn main() {
	loop {
		let mut choice: usize = 0;
		println!("Do you want to run the simulation or play the game? (sim = 0, play = 1)");
		
		read_input(&mut choice);

		if check_if_valid(&choice, &[0, 1]) {
			if choice == 0 {
				let sim = simulation(&[Contestant::new_alice(), Contestant::new_bob()], &1000);

				println!("Alice: {:.3}%\nBob: {:.3}%", sim[0] * 100f64, sim[1] * 100f64);
			} else {
				game();
			}

			break;
		}

		println!("Invalid number entered! Retry...");
	}
}

fn read_input(data: &mut usize) {
	let mut raw_data = String::new();
	io::stdin().read_line(&mut raw_data).expect("Error while reading from stdin!");

	*data = raw_data.trim()
				.parse()
				.expect("Unable to parse inputed data!");
}

fn check_if_valid(data: &usize, range: &[usize]) -> bool {
	let mut out: bool = false;

	for element in range {
		if *data == *element {
			out = true;
		}
	}
	out
}

fn select_door(max_index: usize) -> usize {
	rand::thread_rng().gen_range(0..max_index) as usize
}

fn monty_open_door(doors: &mut [Door]) {
	loop {
		let index = select_door(doors.len());
		
		if !doors[index].is_open() && !doors[index].has_prize && !doors[index].is_selected {
			doors[index].open();
			break;
		}
	}
}

fn change_door(doors: &mut [Door]) {
	for door in doors {
		if door.is_selected {
			door.is_selected = false;
		} else if !door.is_selected && !door.is_open() {
			door.is_selected = true;
		}
	}
}

fn chek_if_won(doors: &mut [Door]) -> bool {
	let mut out = false;
	for door in doors {
		if door.is_selected && door.has_prize {
			out = true;
		}
	}
	out
}

fn game() {
	let mut doors: [Door; 3] = [Door::new(), Door::new(), Door::new()];
	
	let mut choice: usize = 0;
	
	// put the prize in one door
	doors[select_door(doors.len())].has_prize = true;

	// Let the user choose their door
	loop {
		println!("Which door do you choose? (1, 2, 3)");
		read_input(&mut choice);
		
		if check_if_valid(&choice, &[1, 2, 3]) {
			break;
		}
		println!("Invalid number entered!");
	}
	doors[choice - 1].is_selected = true;

	// Open 1 door
	monty_open_door(&mut doors);

	loop {
		println!("Do you want to change the door? (yes = 1, no = 0)");
		read_input(&mut choice);
		
		if check_if_valid(&choice, &[0, 1]) {
			break;
		}
		println!("Invalid number entered!");
	}

	if choice == 1 {
		change_door(&mut doors);
	}

	if chek_if_won(&mut doors) {
		println!("Congrats! You won the prize!!! 🎉");
	} else {
		println!("Sorry, you lost! 😢")
	}
}

fn simulation(contestants: &[Contestant], simulate_contestant: &usize) -> Box<[f64]> {
	let mut simulation_output = vec![0.0f64; contestants.len()];
	
	let mut i = 0;
	for contestant in contestants {
		let mut won: usize = 0;
		for _i in 0..*simulate_contestant {
			let mut doors: [Door; 3] = [Door::new(), Door::new(), Door::new()];

			// put the prize in one door
			doors[select_door(doors.len())].has_prize = true;

			// Let the user choose their door
			doors[contestant.choice - 1].is_selected = true;

			// Open 1 door
			monty_open_door(&mut doors);

			// Change door if contestant chooses to change
			if contestant.changes_door {
				change_door(&mut doors);
			}

			if chek_if_won(&mut doors) {
				won += 1;
			}
		}
		simulation_output[i] = won as f64 / *simulate_contestant as f64;
		i += 1;
	}

	simulation_output.into_boxed_slice()
}