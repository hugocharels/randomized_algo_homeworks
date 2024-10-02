import select


def test_basic():
	arr = [1, 2, 3]
	k = 2
	assert select.quick_select(arr, k) == 2
	assert select.lazy_select(arr, k) == 2
