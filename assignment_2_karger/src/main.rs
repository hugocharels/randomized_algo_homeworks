use assignment_2_karger::data_generator::*;
use std::thread;

fn main() {
	// let num_vertices = vec![20, 40, 60, 80, 100, 120, 140, 160, 180, 200];
	// let num_vertices = vec![30, 60, 90, 120, 150, 180, 210, 240, 270, 300];
	// let num_vertices = vec![50, 100, 150, 200, 250, 300, 350, 400, 450, 500];
	
	// Spawn a thread for `generate_data_success_bounds`
	let handle1 = thread::spawn(move || {
		generate_data_success_bounds(&vec![30, 60, 90, 120, 150, 180, 210, 240, 270, 300], "data_success_bounds_300.csv");
	});

	// Spawn a thread for `generate_data_same_time`
	let handle2 = thread::spawn(move || {
		generate_data_same_time(&vec![30, 60, 90, 120, 150, 180, 210, 240, 270, 300], "data_same_time_300.csv");
	});

	// Wait for both threads to complete
	handle1.join().expect("Thread 1 panicked");
	handle2.join().expect("Thread 2 panicked");
}
