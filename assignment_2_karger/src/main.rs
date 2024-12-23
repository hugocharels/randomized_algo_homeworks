use assignment_2_karger::data_generator::*;

fn main() {
	// let num_vertices = vec![20, 40, 60, 80, 100, 120, 140, 160, 180, 200];
	let num_vertices = vec![50, 100, 150, 200, 250, 300, 350, 400, 450, 500];
	generate_data_success_bounds(&num_vertices, "data_success_bounds_500.csv");
	generate_data_same_time(&num_vertices, "data_same_time_500.csv");
}
