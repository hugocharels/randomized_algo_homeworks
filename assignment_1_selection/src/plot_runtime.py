import matplotlib.pyplot as plt
import pandas as pd
from sys import argv

# Read the runtime CSV file
data = pd.read_csv(argv[-1])

# Separate the data by algorithm
quickselect_data = data[data["Algorithm"] == "QuickSelect"]
lazyselect_data = data[data["Algorithm"] == "LazySelect"]

# Group by list size and compute the average runtime for each size
quickselect_avg_runtime = quickselect_data.groupby("List Size")["Runtime (ms)"].mean()
lazyselect_avg_runtime = lazyselect_data.groupby("List Size")["Runtime (ms)"].mean()

# Generate the list sizes
list_sizes = quickselect_avg_runtime.index

# Plot the empirical runtime results
plt.figure(figsize=(10, 6))
plt.plot(
    list_sizes,
    quickselect_avg_runtime.values,
    label="QuickSelect Empirical Runtime",
    marker="o",
    color="blue",
)
plt.plot(
    list_sizes,
    lazyselect_avg_runtime.values,
    label="LazySelect Empirical Runtime",
    marker="s",
    color="magenta",
)

# Add labels and title
plt.xlabel("List Size (n)")
plt.ylabel("Average Runtime (ms)")
plt.title("Runtime vs List Size for QuickSelect and LazySelect (Empirical)")
plt.legend()

# Show the plot
plt.grid(True)
plt.savefig("graphics/" + argv[-1].split("/")[1].replace(".csv", ".png"))
