from select import quick_select, lazy_select
from random import shuffle

if __name__ == '__main__':
	arr = [*range(1, 1001)]
	shuffle(arr)
	k = 500
	print(quick_select(arr, k))
	print(lazy_select(arr, k))
