// A class that represents an integer value.
// But used to count the number of compaisons
// in the selection sort algorithm.

use std::cmp::Ordering;
use std::fmt;

// The Integer struct
#[derive(Debug)]
pub(crate) struct Integer {
	value: i32,
}

// Static variable to store the number of comparisons
static mut COMPARISONS: usize = 0;

impl Integer {
	// Constructor
	pub(crate) fn new(value: i32) -> Self {
		Integer { value }
	}

	// Method to clear the comparison count
	pub(crate) fn clear_comparisons() {
		unsafe {
			COMPARISONS = 0;
		}
	}

	// Helper method to update the comparison count
	fn update_comparisons() {
		unsafe {
			COMPARISONS += 1;
		}
	}

	// Function to get the current comparison count
	pub(crate) fn get_comparisons() -> usize {
		unsafe { COMPARISONS }
	}
}

// Implement the Eq, PartialEq, PartialOrd, and Ord traits for Integer
impl Eq for Integer {}

impl PartialEq<Self> for Integer {
	fn eq(&self, other: &Self) -> bool {
		Integer::update_comparisons();
		self.value == other.value
	}
}

impl PartialOrd<Self> for Integer {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Integer::update_comparisons();
		Some(self.value.cmp(&other.value))
	}
}

impl Ord for Integer {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		Integer::update_comparisons();
		self.value.cmp(&other.value)
	}
}

impl Clone for Integer {
	fn clone(&self) -> Self {
		Integer { value: self.value }
	}
}

// Implement the Copy trait for Integer
impl Copy for Integer {}


// Implement the Display trait to allow printing the value of Integer
impl fmt::Display for Integer {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.value)
	}
}