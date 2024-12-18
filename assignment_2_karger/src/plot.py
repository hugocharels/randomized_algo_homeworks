import matplotlib.pyplot as plt
import numpy as np
import os
import pandas as pd

# Use a scientific plotting style
# print(plt.style.available)
plt.style.use("seaborn-v0_8-whitegrid")


def theoretical_success_probability(algo, n):
	"""
	Calculate the theoretical success probability for each algorithm.
	- Contract: 2 / (n * (n - 1))
	- FastCut: 1 / log(n)
	"""
	if algo == 'contract':
		return 2 / (n * (n - 1)) if n > 1 else 0
	elif algo == 'fast_cut':
		return 1 / np.log(n) if n > 1 else 0
	else:
		return 0


def plot_success_rate(csv_file):
	# Load the CSV file
	df = pd.read_csv(csv_file)

	# Check if the necessary columns exist
	required_columns = {'algo', 'len_vertices', 'percentage'}
	if not required_columns.issubset(df.columns):
		raise ValueError(f"CSV must contain the following columns: {required_columns}")

	# Group the data by algorithm and graph size (number of vertices)
	grouped = df.groupby(['algo', 'len_vertices'])['percentage'].mean().reset_index()

	# Set up the plot
	plt.figure(figsize=(8, 6))
	colors = {'contract': 'blue', 'fast_cut': 'orange'}
	markers = {'contract': 'o', 'fast_cut': 's'}

	for algo in grouped['algo'].unique():
		algo_name = "Contract" if algo == 'contract' else "FastCut"
		algo_data = grouped[grouped['algo'] == algo]
		# Plot empirical data
		plt.plot(
			algo_data['len_vertices'],
			algo_data['percentage'],
			label=f"Empirical Bound: {algo_name}",
			marker=markers[algo],
			linestyle='-',
			color=colors[algo],
			linewidth=1.5,
			markersize=6
		)

		# Plot theoretical probabilities
		theoretical_probs = [
			theoretical_success_probability(algo, n) * 100 for n in algo_data['len_vertices']
		]
		plt.plot(
			algo_data['len_vertices'],
			theoretical_probs,
			linestyle='--',
			label=f"Theoretical Bound: {algo_name}",
			color=colors[algo],
			linewidth=1.5
		)

	# Customize the plot
	plt.title(r'Success Probability vs. Graph Size', fontsize=14)
	plt.xlabel(r'Number of Vertices ($n$)', fontsize=12)
	plt.ylabel(r'Success Probability (%)', fontsize=12)
	plt.grid(which='both', linestyle=':', linewidth=0.5)
	plt.legend(fontsize=10, loc='upper center', bbox_to_anchor=(0.5, -0.1), ncol=2)
	plt.tight_layout()

	# Save the figure
	plt.savefig(csv_file.replace(".csv", "_plot.png"), dpi=300, bbox_inches='tight')

	# Show the plot
	plt.show()


# Path to the directory
directory = "generated_data"

# Iterate through all files in the directory
if os.path.exists(directory):
	for filename in os.listdir(directory):
		file_path = os.path.join(directory, filename)
		if os.path.isfile(file_path) and ".csv" in file_path:  # Ensure it's a file
			plot_success_rate(file_path)
else:
	print(f"Directory '{directory}' does not exist.")
