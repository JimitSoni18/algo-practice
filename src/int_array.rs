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

/// find a missing number from array with 1..N numbers.
/// Works on the guarantee that exactly one number is missing
/// and all the numbers are from 1 to N where N is length of slice
pub fn find_missing_number(arr: &[usize]) -> usize {
	let mut proxy = arr.to_vec();
	proxy.sort();

	if proxy[0] == 2 {
		return 1;
	}

	fn binary_search_inner(arr: Vec<usize>, start: usize, end: usize) -> usize {
		if start > end {
			panic!("wtf")
		}
		let len = end - start + 1;
		let mid = start + ((len - (len % 2)) / 2);
		if len == 2 {
			if arr[mid - 1] + 2 == arr[mid] {
				return arr[mid] - 1;
			} else if arr[mid + 1] - 2 == arr[mid] {
				panic!("wtfff!");
			} else {
				panic!("wtf");
			}
		}

		if mid == arr[mid] - 1 {
			return binary_search_inner(arr, mid, end);
		} else if mid == arr[mid] - 2 {
			return binary_search_inner(arr, start, mid);
		} else {
			let a = format!("wtf! mid: {mid}, arr[mid]: {}", arr[mid]);
			panic!("{a}");
		}

		// todo!()
	}

	let len = proxy.len();

	return binary_search_inner(proxy, 0, len - 1);

	// todo!()
}

pub trait NumArrayInPlaceOperations {
	fn move_zeros_to_end(&mut self);
}

impl NumArrayInPlaceOperations for [u32] {
	// FIXME: OLD IMPLEMENTATION HAS THE WORST CASE TIME COMPLEXITY OF O(N^2)
	// FIXME: USE OPTIMAL APPROACH

	// FIXME: THE NEWER IMPLEMENTATION FAILS TESTS,
	// FIXME: NEED TO REIMPLEMENT WITH FRESH MIND
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

		while j < len && i < len {
			while j < len && self[j] != 0 {
				j += 1;
			}
			if j == len - 1 {
				if self[j] != 0 && self[i] == 0 {
					self.swap_in_place(i, j);
				}
				return;
			}
			i = j + 1;
			if i >= len {
				return;
			}
			if self[i] != 0 {
				self.swap_in_place(i, j);
			} else {
				while i < len && self[i] == 0 {
					i += 1;
					// println!("=>> how many times here: {i}, self[i]: {}", self[i]);
				}
				println!("=>> what is i while exiting: {i}");
				if i == len - 1 {
					if self[i] != 0 {
						self.swap_in_place(i, j);
					}
					println!("=>> i is len-1");
					return;
				}
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

		arr.move_zeros_to_end();
		assert!(arr.eq(&[2, 3, 5, 1, 2, 1, 1, 1, 1, 1, 4, 5]));
		println!("=>> test 1 complete");

		let mut arr = [1, 3, 6, 0, 1, 0, 0, 0, 9];
		arr.move_zeros_to_end();
		assert!(arr.eq(&[1, 3, 6, 1, 9, 0, 0, 0, 0]));
		println!("=>> test 2 complete");

		let mut arr = [0, 3, 6, 0, 1, 0, 0, 9];
		arr.move_zeros_to_end();
		assert!(arr.eq(&[3, 6, 1, 9, 0, 0, 0, 0]));
		println!("=>> test 3 complete");

		let mut arr = [1, 3, 6, 0, 1, 0, 0, 0];
		arr.move_zeros_to_end();
		assert!(arr.eq(&[1, 3, 6, 1, 0, 0, 0, 0]));
		println!("=>> test 4 complete");

		let mut arr = [0, 3, 6, 1, 1, 1, 1, 0];
		arr.move_zeros_to_end();
		assert!(arr.eq(&[3, 6, 1, 1, 1, 1, 0, 0]));
		println!("=>> test 5 complete");

		let mut arr = [0, 0, 0, 0, 0, 0, 0, 0];
		arr.move_zeros_to_end();
		assert!(arr.eq(&[0, 0, 0, 0, 0, 0, 0, 0]));
		println!("=>> test 6 complete");
	}

	#[test]
	fn test_arr_union() {
		let arr1 = [1, 2, 4, 4, 4, 6, 8];
		let arr2 = [1, 3, 5, 6, 7, 8, 8];

		let union = slice_union(&arr1, &arr2);

		assert!(union.eq(&[1, 2, 3, 4, 5, 6, 7, 8]))
	}

	#[test]
	fn test_find_missing_number_found() {
		let arr = [1, 2, 3, 4, 6, 7, 8];
		let missing = find_missing_number(&arr);
		assert_eq!(missing, 5);

		let arr = [2, 3, 4, 5, 6, 7, 8, 9, 10];
		let missing = find_missing_number(&arr);
		assert_eq!(missing, 1);

		let arr = [1, 3, 4, 5, 6, 7, 8];
		let missing = find_missing_number(&arr);
		assert_eq!(missing, 2);

		let arr = [1, 2, 3, 4, 5, 6, 7, 9, 10];
		let missing = find_missing_number(&arr);
		assert_eq!(missing, 8);

		let arr = [1, 2, 4, 5, 6, 7, 8, 9, 10];
		let missing = find_missing_number(&arr);
		assert_eq!(missing, 3);

		let arr = [9, 2, 6, 8, 1, 3, 5, 7, 4, 11];
		let missing = find_missing_number(&arr);
		assert_eq!(missing, 10);
	}
}
