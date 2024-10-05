from src import select
import pytest
from random import randint

SIZE = 1000
TRY = 1000

PARAMS = [
	([randint(1, 1000) for _ in range(SIZE)], randint(0, SIZE-1))
	# ([randint(1, 1000) for _ in range(SIZE)], randint(0, int((SIZE - 1) / 2)))
	# ([randint(1, 1000) for _ in range(SIZE)], randint(0, 10))
	for _ in range(TRY - 1)
]


def correct_select(arr, k):
	return sorted(arr)[k]


def _test_basic(select: callable):
	arr = [1, 2, 3]
	assert select(arr, 0) == 1
	assert select(arr, 1) == 2
	assert select(arr, 2) == 3


def _test_random_arr_select(select, arr, k):
	found = select(arr, k)
	right = correct_select(arr, k)
	# assert found == right, f"{select.__name__}({arr}, {k}) = {found}, expected {right}"
	assert found == right, f"{select.__name__}(k={k}) = {found}, expected {right}"


def test_basic_quick():
	return _test_basic(select.quick_select)


def test_basic_lazy():
	return _test_basic(select.lazy_select)


@pytest.mark.parametrize("param", PARAMS)
def test_random_arr_quick(param):
	return _test_random_arr_select(select.quick_select, *param)


@pytest.mark.parametrize("param", PARAMS)
def test_random_arr_lazy(param):
	return _test_random_arr_select(select.lazy_select, *param)
