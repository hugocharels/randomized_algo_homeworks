import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import seaborn as sns


def plot_success_rates(csv_file):
	# Set up Seaborn for scientific-quality plots
	sns.set_context("paper", font_scale=1.4)
	sns.set_style("whitegrid")

	# Load the CSV file
	data = pd.read_csv(csv_file)

	# Get unique graph types
	graph_types = data["Graph Type"].unique()

	# Create separate plots for each graph type
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
			marker="o",
			label="FastCut Success Rate"
		)
		plt.plot(
			graph_size,
			fastcut_theoretical_bound,
			linestyle="--",
			label="FastCut Theoretical Bound"
		)

		# Plot Contract data
		plt.plot(
			graph_size,
			contract_rate,
			marker="s",
			label="Contract Success Rate"
		)
		plt.plot(
			graph_size,
			contract_theoretical_bound,
			linestyle="--",
			label="Contract Theoretical Bound"
		)

		plt.xlabel("Graph Size")
		plt.ylabel("Success Rate (%)")
		plt.title(f"Success Rates for {graph_type} Graphs")
		plt.legend()
		plt.tight_layout()
		plt.savefig(f"{csv_file.replace("data_", "").replace(".csv", "")}_{graph_type.lower()}_plot.png", dpi=300)
		plt.show()


if __name__ == "__main__":
	plot_success_rates("data_success_bounds_200.csv")
	plot_success_rates("data_same_time_200.csv")
