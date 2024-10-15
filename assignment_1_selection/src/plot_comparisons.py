import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

# Constants for expected comparisons
ln2 = np.log(2)  # Natural log of 2
quickselect_factor = 2 * (1 + ln2)

# Read the CSV file
data = pd.read_csv('comparison_results_small.csv')

# Separate the data by algorithm
quickselect_data = data[data['Algorithm'] == 'QuickSelect']
lazyselect_data = data[data['Algorithm'] == 'LazySelect']

# Group by list size and compute the average number of comparisons for each size
quickselect_avg = quickselect_data.groupby('List Size')['Comparisons'].mean()
lazyselect_avg = lazyselect_data.groupby('List Size')['Comparisons'].mean()

# Generate the expected comparisons for each list size
list_sizes = quickselect_avg.index

# QuickSelect expected comparisons: 2(1 - ln(2))n
quickselect_expected = quickselect_factor * list_sizes

# LazySelect expected comparisons: [2n, 3n[
lazyselect_expected_lower = 2 * list_sizes
lazyselect_expected_upper = 3 * list_sizes

# Plot the empirical results
plt.figure(figsize=(10, 6))
plt.plot(list_sizes, quickselect_avg.values, label='QuickSelect Empirical', marker='o', color='red')
plt.plot(list_sizes, lazyselect_avg.values, label='LazySelect Empirical', marker='s', color='orange')

# Plot the expected comparisons for QuickSelect
plt.plot(list_sizes, quickselect_expected, label='QuickSelect Expected 2(1+ln2)n', linestyle='--', color='red')

# Plot the expected comparisons for LazySelect (as a shaded area between 2n and 3n)
plt.fill_between(list_sizes, lazyselect_expected_lower, lazyselect_expected_upper, color='orange', alpha=0.2, label='LazySelect Expected (2n + o(n))')

# Add labels and title
plt.xlabel('List Size (n)')
plt.ylabel('Average Number of Comparisons')
plt.title('Number of Comparisons vs List Size for QuickSelect and LazySelect (Empirical vs Expected)')
plt.legend()

# Show the plot
plt.grid(True)
plt.savefig('comparison_plot_small.png')
