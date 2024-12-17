use assignment_2_karger::data_generator::*;

fn main() {
	generate_data(
		vec![100,200,300,400,500,600,700,800,900,1000],
		|v: usize| 2*v,
		"generated_data/hundred_2v.csv"
	);
	generate_data(
		vec![100,200,300,400,500,600,700,800,900,1000],
		|v: usize| 5*v,
		"generated_data/hundred_5v.csv"
	);
	generate_data(
		vec![100,200,300,400,500,600,700,800,900,1000],
		|v: usize| 10*v,
		"generated_data/hundred_10v.csv"
	);
}
