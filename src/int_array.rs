use std::ops::Index;

use crate::InPlaceSliceOperations;

pub trait NumArrayInPlaceOperations {
	fn move_zeros_to_end(&mut self);
}

impl NumArrayInPlaceOperations for [u32] {
	// FIXME: THIS HAS THE WORST CASE TIME COMPLEXITY OF O(N^2)
	// FIXME: USE OPTIMAL APPROACH
	fn move_zeros_to_end(&mut self) {
		let len = self.len();
		// OLD IMPLEMENTATION
		// let mut current = 0;

		// for _ in 0..len {
		// 	println!("=>> current for {current}");
		// 	if self[current] == 0 {
		// 		self[current..].rotate_slice_in_place_left(1);
		// 	} else {
		// 		current += 1;
		// 	}
		// }
		let mut j = 0;
		let mut i = 0;

		while i <= len {
			while j < len && self[j] != 0 {
				j += 1;
				println!("=>> j is {j}");
			}

			if j == len && self[j] == 0 {
				return;
			} else {
				self.swap_in_place(i, j);
				return;
			}

			if self[i] != 0 {
				self.swap_in_place(i, j);
			}
		}

		todo!();
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_moved_zeros_to_end() {
		let mut arr = [2, 3, 5, 1, 2, 1, 1, 1, 1, 1, 4, 5];

		println!("=>> before {arr:?}");

		arr.move_zeros_to_end();

		println!("=>> after  {arr:?}");
	}
}
