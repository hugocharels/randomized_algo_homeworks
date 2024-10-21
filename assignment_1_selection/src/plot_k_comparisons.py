import matplotlib.pyplot as plt
import pandas as pd
from sys import argv

# Read the CSV file with the additional 'k' column
data = pd.read_csv(argv[-1])

# Separate the data by algorithm
quickselect_data = data[data["Algorithm"] == "QuickSelect"]
lazyselect_data = data[data["Algorithm"] == "LazySelect"]

# Group by 'K' and compute the average number of comparisons and standard deviation for each k
quickselect_avg_by_k = quickselect_data.groupby("K")["Comparisons"].mean()
quickselect_std_by_k = quickselect_data.groupby("K")["Comparisons"].std()
lazyselect_avg_by_k = lazyselect_data.groupby("K")["Comparisons"].mean()
lazyselect_std_by_k = lazyselect_data.groupby("K")["Comparisons"].std()

# Generate expected comparisons for each k
k_values = quickselect_avg_by_k.index

# Calculate the upper and lower bounds for standard deviation
quickselect_lower_bound = quickselect_avg_by_k - quickselect_std_by_k
quickselect_upper_bound = quickselect_avg_by_k + quickselect_std_by_k
lazyselect_lower_bound = lazyselect_avg_by_k - lazyselect_std_by_k
lazyselect_upper_bound = lazyselect_avg_by_k + lazyselect_std_by_k

# Plot 1: Mean of comparisons per k with standard deviation area
plt.figure(figsize=(10, 6))

# QuickSelect area for standard deviation
plt.fill_between(
	k_values,
	quickselect_lower_bound,
	quickselect_upper_bound,
	color="red",
	alpha=0.2,
	label="QuickSelect Standard Deviation Area",
)

# LazySelect area for standard deviation
plt.fill_between(
	k_values,
	lazyselect_lower_bound,
	lazyselect_upper_bound,
	color="orange",
	alpha=0.2,
	label="LazySelect Standard Deviation Area",
)

# Plot mean comparisons for QuickSelect and LazySelect
plt.plot(
	k_values,
	quickselect_avg_by_k.values,
	label="QuickSelect Mean Comparisons",
	marker="o",
	color="red",
	markersize=3,
	linestyle="None",
)
plt.plot(
	k_values,
	lazyselect_avg_by_k.values,
	label="LazySelect Mean Comparisons",
	marker="o",
	color="orange",
	markersize=3,
	linestyle="None",
)

plt.xlabel("k")
plt.ylabel("Mean Number of Comparisons")
plt.title(
	"Mean Comparisons per k for QuickSelect and LazySelect with Standard Deviation Area"
)
plt.legend()
plt.grid(True)
plt.savefig("graphics/" + argv[-1].split("/")[1].replace(".csv", ".png"))
