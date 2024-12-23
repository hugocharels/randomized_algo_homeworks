pub fn generate_data_success_bounds(num_vertices: &Vec<usize>, csv_file_name: &str) {
	// TODO:
	// For each graph size, build a complete and a planar graph
	// Run FastCut and Contract 100 times for each graph
	// Compute the success rate of each algorithm
	// Write the results to a CSV file

}

pub fn generate_data_same_time(num_vertices: &Vec<usize>, csv_file_name: &str) {
	// TODO:

	// For each graph size, build a complete and a planar graph
	// Do that 100 times
	//      Run FastCut one time and measure the time
	//      Run Contract repeatedly until the elapsed time matches that of FastCut
	//      Compute the success rate of each algorithm
	// Write the results to a CSV file
}
