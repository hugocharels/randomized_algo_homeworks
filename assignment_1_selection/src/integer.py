class Integer:

	COMPARISON: {(int, int): int} = dict()
	COMPARISON_COUNT: int = 0

	def __init__(self, value):
		self.__value = int(value)

	def __str__(self):
		return str(self.__value)

	def __update(self, other) -> None:
		Integer.COMPARISON[(self.__value, other.__value)] = Integer.COMPARISON.get((self.__value, other.__value), 0) + 1
		Integer.COMPARISON_COUNT += 1

	@staticmethod
	def clear_comparison() -> None:
		Integer.COMPARISON.clear()
		Integer.COMPARISON_COUNT = 0

	def __eq__(self, other) -> bool:
		self.__update(other)
		return self.__value == other.__value

	def __ne__(self, other) -> bool:
		self.__update(other)
		return self.__value != other.__value

	def __lt__(self, other) -> bool:
		self.__update(other)
		return self.__value < other.__value

	def __le__(self, other) -> bool:
		self.__update(other)
		return self.__value <= other.__value

	def __gt__(self, other) -> bool:
		self.__update(other)
		return self.__value > other.__value

	def __ge__(self, other) -> bool:
		self.__update(other)
		return self.__value >= other.__value
