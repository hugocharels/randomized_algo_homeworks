import math
import random


def quick_select(arr: [int], k: int) -> int:

	# pick a random pivot
	pivot = random.choice(arr)

	# partition the array into three parts
	left = [x for x in arr if x < pivot]
	mid = [x for x in arr if x == pivot]
	right = [x for x in arr if x > pivot]

	# recursively partition the array
	if k < len(left):
		return quick_select(left, k)
	elif k < len(left) + len(mid):
		return mid[0]
	else:
		return quick_select(right, k - len(left) - len(mid))


def lazy_select(arr: [int], k: int) -> int:
	n = len(arr)

	# pick n^3/4 elements from the array chosen independently and uniformly at random with replacement
	sample = random.choices(arr, k=math.floor(n ** (3 / 4)))

	# sort the sample using an optimal sorting algorithm
	sample.sort()

	# setting boundaries
	x = k * (n ** (-1 / 4))
	l = max(math.floor(x - math.sqrt(n)), 0)
	h = min(math.ceil(x + math.sqrt(n)), len(sample) - 1)
	a = sample[l]
	b = sample[h]

	# determine r_s(a) and r_s(b)
	rank_a = sum(1 for y in arr if y < a)  # strictly less than a
	rank_b = sum(1 for y in arr if y <= b)  # less than or equal to b

	# do the partitioning
	partition = []
	if k < n ** (1 / 4):
		partition = [y for y in arr if y <= b]
	elif k > n - n ** (1 / 4):
		partition = [y for y in arr if y >= a]
	else:
		partition = [y for y in arr if a <= y <= b]

	# check if the partition is correct
	adjusted_k = k - rank_a  # Adjust k relative to the partition
	if adjusted_k < 0 or adjusted_k >= len(partition):
		return lazy_select(arr, k)

	partition.sort()

	return partition[k - rank_a]
