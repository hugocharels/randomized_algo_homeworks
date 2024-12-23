import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import seaborn as sns


# Function to plot success rate with theoretical bounds
def plot_success_rates(csv_file):
	# Set up Seaborn for scientific-quality plots
	sns.set_context("paper", font_scale=1.4)
	sns.set_style("whitegrid")

	# Load the CSV file
	data = pd.read_csv(csv_file)

	# Get unique graph types
	graph_types = data["Graph Type"].unique()

	# Define colors and markers for consistent plotting
	fastcut_color = "red"
	contract_color = "orange"

	for graph_type in graph_types:
		subset = data[data["Graph Type"] == graph_type]
		graph_size = subset["Graph Size"]

		# FastCut data
		fastcut_rate = subset["FastCut Success Rate"]
		fastcut_theoretical_bound = [100 / np.log(n) if n > 1 else 0 for n in graph_size]

		# Contract data
		contract_rate = subset["Contract Success Rate"]
		contract_theoretical_bound = [200 / (n * (n - 1)) if n > 1 else 0 for n in graph_size]

		# Combined plot
		plt.figure(figsize=(10, 6))

		# Plot FastCut data
		plt.plot(
			graph_size,
			fastcut_rate,
			marker="*",
			markersize=10,
			color=fastcut_color,
			label="FastCut Empirical"
		)
		plt.plot(
			graph_size,
			fastcut_theoretical_bound,
			linestyle="--",
			color=fastcut_color,
			label="FastCut Theoretical"
		)

		# Plot Contract data
		plt.plot(
			graph_size,
			contract_rate,
			marker="s",
			markersize=3,
			color=contract_color,
			label="Contract Empirical"
		)
		plt.plot(
			graph_size,
			contract_theoretical_bound,
			linestyle="--",
			color=contract_color,
			label="Contract Theoretical"
		)

		plt.ylim(-10, 110)  # Adjust the range as needed

		# Axes labels and title
		plt.xlabel("Graph Size", fontsize=14)
		plt.ylabel("Success Rate (%)", fontsize=14)
		plt.title(f"Success Rates for {graph_type} Graphs", fontsize=16)

		# Improved legend
		plt.legend(loc="lower right", fontsize=12, frameon=True)
		plt.tight_layout()

		# Save and display the plot
		filename = f"{csv_file.replace('data_', '').replace('.csv', '')}_{graph_type.lower()}_success_plot.png"
		plt.savefig(filename, dpi=300)
		# plt.show()


# Function to plot same-time metrics without theoretical bounds
def plot_same_time(csv_file):
	# Set up Seaborn for scientific-quality plots
	sns.set_context("paper", font_scale=1.4)
	sns.set_style("whitegrid")

	# Load the CSV file
	data = pd.read_csv(csv_file)

	# Get unique graph types
	graph_types = data["Graph Type"].unique()

	# Define colors and markers for consistent plotting
	fastcut_color = "red"
	contract_color = "orange"

	for graph_type in graph_types:
		subset = data[data["Graph Type"] == graph_type]
		graph_size = subset["Graph Size"]

		# FastCut data
		fastcut_rate = subset["FastCut Success Rate"]

		# Contract data
		contract_rate = subset["Contract Success Rate"]

		# Combined plot
		plt.figure(figsize=(10, 6))

		# Plot FastCut data
		plt.plot(
			graph_size,
			fastcut_rate,
			marker="*",
			markersize=10,
			color=fastcut_color,
			label="FastCut"
		)

		# Plot Contract data
		plt.plot(
			graph_size,
			contract_rate,
			marker="s",
			markersize=3,
			color=contract_color,
			label="Contract"
		)

		plt.ylim(50, 105)  # Adjust the range as needed

		# Axes labels and title
		plt.xlabel("Graph Size", fontsize=14)
		plt.ylabel("Success Rate (%)", fontsize=14)
		plt.title(f"Success Rate for {graph_type} Graphs (Same Time)", fontsize=16)

		# Improved legend
		plt.legend(loc="lower right", fontsize=12, frameon=True)
		plt.tight_layout()

		# Save and display the plot
		filename = f"{csv_file.replace('data_', '').replace('.csv', '')}_{graph_type.lower()}_same_time_plot.png"
		plt.savefig(filename, dpi=300)
		# plt.show()


if __name__ == "__main__":
	# Generate plots
	plot_success_rates("data_success_bounds_200.csv")  # Includes theoretical bounds
	plot_same_time("data_same_time_200.csv")          # Excludes theoretical bounds
