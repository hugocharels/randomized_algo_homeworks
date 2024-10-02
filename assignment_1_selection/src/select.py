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
	sample = random.sample(arr, math.floor(n ** (3 / 4)))
	# sort the sample using an optimal sorting algorithm
	sample.sort()
	# setting boundaries
	x = k * (n ** (-1 / 4))
	l = max(math.floor(x - math.sqrt(n)), 1)
	h = min(math.ceil(x + math.sqrt(n)), math.floor(n ** (3 / 4)))
	a = sample[l]
	b = sample[h]
	# determine r_s(a) and r_s(b)
	rank_a = sum(1 for y in arr if y <= a)

	# do the partitioning
	partition = []
	if k < n ** (1 / 4):
		partition = [y for y in arr if y <= b]
	elif k > n - n ** (1 / 4):
		partition = [y for y in arr if y >= a]
	else:
		partition = [y for y in arr if a <= y <= b]
	# check if the partition is correct
	if not (arr[k] in partition and len(partition) <= 4 * n ** (3 / 4) + 2):
		return lazy_select(arr, k)

	partition.sort()

	return partition[k - rank_a + 1]