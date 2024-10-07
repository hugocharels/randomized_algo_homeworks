from select import quick_select, lazy_select
from random import shuffle

if __name__ == '__main__':
    arr = [Integer(x) for x in range(1000)]
    shuffle(arr)
    k = 500
    print(quick_select(arr, k))
    print(f"quick : {Integer.COMPARISON_COUNT}")
    Integer.clear(_comparison)
    print(lazy_select(arr, k))
    print(f"lazy : {Integer.COMPARISON_COUNT}")
