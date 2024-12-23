use assignment_2_karger::data_generator::*;

fn main() {
	let num_vertices = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
	generate_data_success_bounds(&num_vertices, "data_success_bounds.csv");
	generate_data_same_time(&num_vertices, "data_same_time.csv");
}
