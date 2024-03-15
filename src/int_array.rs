use core::panic;

use crate::InPlaceSliceOperations;

/// Unoptimized array union
pub fn slice_union<T: Ord + Eq + Copy>(arr1: &[T], arr2: &[T]) -> Vec<T> {
	let len1 = arr2.len();
	let len2 = arr1.len();
	let mut union = Vec::with_capacity(len1 + len2);

	let mut i = 0;
	let mut j = 0;

	while i < len1 && j < len2 {
		if arr1[i] < arr2[j] {
			if let Some(&val) = union.last() {
				if val == arr1[i] {
					i += 1;
					continue;
				}
			}
			union.push(arr1[i]);
			i += 1;
		} else if arr1[i] > arr2[j] {
			if let Some(&val) = union.last() {
				if val == arr2[j] {
					continue;
				}
			}
			union.push(arr2[j]);
			j += 1;
		} else if arr1[i] == arr2[j] {
			if let Some(&val) = union.last() {
				if val == arr2[j] {
					continue;
				}
			}
			union.push(arr2[j]);
			j += 1;
			i += 1;
		} else {
			panic!("wtf");
		}
	}

	if i < len1 {
		for &item in &arr1[i..] {
			if let Some(&val) = union.last() {
				if val == arr1[i] {
					continue;
				}
			}
			union.push(item);
		}
	}

	if j < len2 {
		for &item in &arr2[j..] {
			if let Some(&val) = union.last() {
				if val == arr2[j] {
					continue;
				}
			}
			union.push(item);
		}
	}

	return union;
}

// pub fn find_missing_number(arr: &[usize]) -> usize {
// 	let proxy = arr.clone().sort();

// 	fn binary_search_inner(arr: &[usize], start: usize, end: usize) -> usize {
// 		let len = end - start;

// 		todo!()
// 	}
// }

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

		let mut i = 0;
		let mut j = 0;

		while j < len {
			while self[j] != 0 {
				j += 1;
			}
			if j == len - 1 {
				if self[j] != 0 {
					self.swap_in_place(i, j);
				}
				return;
			}
			i = j + 1;
			if self[i] != 0 {
				self.swap_in_place(i, j);
			}
		}
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

	#[test]
	fn test_arr_union() {
		let arr1 = [1, 2, 4, 4, 4, 6, 8];
		let arr2 = [1, 3, 5, 6, 7, 8, 8];

		let union = slice_union(&arr1, &arr2);

		assert!(union.eq(&[1, 2, 3, 4, 5, 6, 7, 8]))
	}
}
