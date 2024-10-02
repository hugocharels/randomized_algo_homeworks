from src import select
import pytest
from random import randint

SIZE = 1000
TRY = 0


PARAMS = [
	([randint(1, 1000) for _ in range(SIZE)], randint(0, 999))
	for _ in range(TRY)
]


def correct_select(arr, k):
	return sorted(arr)[k]


def test_basic_quick():
	arr = [1, 2, 3]
	k = 1
	assert select.quick_select(arr, k) == 2


def test_basic_lazy():
	arr = [1, 2, 3]
	k = 1
	assert select.lazy_select(arr, k) == 2


@pytest.mark.parametrize("param", PARAMS)
def test_random_arr_quick(param):
	arr, k = param
	assert select.quick_select(arr, k) == correct_select(arr, k)


@pytest.mark.parametrize("param", PARAMS)
def test_random_arr_lazy(param):
	arr, k = param
	assert select.lazy_select(arr, k) == correct_select(arr, k)
