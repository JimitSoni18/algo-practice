use core::panic;

use crate::InPlaceSliceOperations;

pub fn stock_buy_and_sell(arr: &[isize]) -> isize {
	let mut max = arr[0];
	let mut min = arr[0];
	let mut global_sum = 0;

	for &i in arr {
		if i < min {
			min = i;
			max = i;
		}
		if i > max {
			max = i;
			global_sum = global_sum.max(max - min)
		}
	}
	global_sum
}

pub fn rearrange_alt_sign(nums: Vec<i32>) -> Vec<i32> {
	let len = nums.len();
	let mut arr = vec![0; len];
	let mut pos_index = 0;
	let mut neg_index = 1;
	for i in nums {
		if i < 0 {
			arr[neg_index] = i;
			neg_index += 2;
		} else {
			arr[pos_index] = i;
			pos_index += 2;
		}
	}

	arr
}

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
	fn max_consecutive_ones(&self) -> usize;
	fn length_of_longest_subarray_with_sum(&self, k: u32) -> usize;
	fn dutch_national_flag_problem(&mut self);
	fn find_majority_element(&self) -> usize;
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

	fn max_consecutive_ones(&self) -> usize {
		let mut local_maxima = 0;
		let mut global_maxima = 0;

		let mut i = 0;
		while i < self.len() {
			if self[i] == 1 {
				local_maxima += 1;
				if local_maxima > global_maxima {
					global_maxima = local_maxima;
				}
			} else if self[i] == 0 {
				local_maxima = 0;
			}

			i += 1;
		}

		global_maxima
	}

	// TODO: USE OPTIMAL APPROACH
	fn length_of_longest_subarray_with_sum(&self, k: u32) -> usize {
		let mut global_count = 0;

		let len = self.len();

		for i in 0..len {
			let mut sum = 0;
			let mut j = i;
			while j < len && sum <= k {
				sum += self[j];

				if sum == k {
					let f = j - i + 1;
					if f > global_count {
						global_count = f;
					}
					// skips other unnecessary iterations
					break;
				}

				j += 1;
			}
		}

		global_count
	}

	fn dutch_national_flag_problem(&mut self) {
		let len = self.len();
		let (mut i, mut j, mut k) = (0, 0, len - 1);
		while j <= k {
			// if self[j] == 0 {
			// 	self.swap_in_place(i, j);
			// 	i += 1;
			// 	j += 1;
			// } else if self[j] == 1 {
			// 	j += 1;
			// } else if self[j] == 2 {
			// 	self.swap_in_place(j, k);
			// 	k -= 1;
			// } else {
			// 	panic!("wtf");
			// }

			match self[j] {
				0 => {
					self.swap_in_place(i, j);
					i += 1;
					j += 1;
				}
				1 => {
					j += 1;
				}
				2 => {
					self.swap_in_place(j, k);
					k -= 1;
				}
				_ => {}
			}
		}
	}

	fn find_majority_element(&self) -> usize {
		// let mut hmap = HashMap::new();
		// for item in self {
		// 	if hmap.contains_key(item) {
		// 		if hmap.get(item).unwrap() == self.len() / 2 - 1 {
		// 			return item;
		// 		}
		// 		hmap.insert(item, hmap.get(item).unwrap() + 1);
		// 	} else {
		// 		hmap.insert(item, 1);
		// 	}
		// }

		todo!()
	}
}

// TODO: USE OPTIMAL APPROACH
pub fn length_of_longest_subarray_with_sum_pos_neg(slice: &[i32], k: i32) -> usize {
	let mut global_count = 0;

	let len = slice.len();

	for i in 0..len {
		let mut sum = 0;
		let mut j = i;
		while j < len {
			sum += slice[j];

			if sum == k {
				let f = j - i + 1;
				if f > global_count {
					global_count = f;
				}
			}

			j += 1;
		}
	}

	global_count
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

	#[test]
	fn test_max_consecutive_ones() {
		let arr = [1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1];

		let a = arr.max_consecutive_ones();
		assert_eq!(4, a);

		// Empty array
		let arr = [];
		let expected = 0;
		let actual = arr.max_consecutive_ones();
		assert_eq!(expected, actual);

		// Array with all ones
		let arr = [1, 1, 1, 1, 1];
		let expected = 5;
		let actual = arr.max_consecutive_ones();
		assert_eq!(expected, actual);

		// Array with single one
		let arr = [0, 1, 0];
		let expected = 1;
		let actual = arr.max_consecutive_ones();
		assert_eq!(expected, actual);

		// Array with multiple consecutive ones
		let arr = [1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1];
		let expected = 4;
		let actual = arr.max_consecutive_ones();
		assert_eq!(expected, actual);

		// Array with ones at the end
		let arr = [0, 0, 0, 1, 1, 1];
		let expected = 3;
		let actual = arr.max_consecutive_ones();
		assert_eq!(expected, actual);
	}

	#[test]
	fn test_len_longest_subarr() {
		let arr = &[1, 3, 5, 7, 8, 9, 0, 2, 5, 6, 8, 2, 4, 5];
		let s = arr.length_of_longest_subarray_with_sum(9);
		assert_eq!(3, s);

		let arr = &[2, 3, 1, 1, 0, 4, 1, 6, 8, 9];
		let s = arr.length_of_longest_subarray_with_sum(6);
		assert_eq!(4, s);

		let arr = &[1, 0, 2, 1, 3, 4];
		let s = arr.length_of_longest_subarray_with_sum(7);
		assert_eq!(5, s);

		let arr = &[3, 4, 1, 0, 2, 0, 8, 3];
		let s = arr.length_of_longest_subarray_with_sum(9);
		assert_eq!(0, s);

		let arr = &[3, 4, 1, 0, 2, 9, 8, 3];
		let s = arr.length_of_longest_subarray_with_sum(9);
		assert_eq!(1, s);
	}

	// TODO: test with more cases
	#[test]
	fn test_len_longest_subarr_pos_neg() {
		let arr = &[-1, 3, 5, -1, 8, 9, 0, 2, 5, 6, -8, 2, 4, 5];
		let s = length_of_longest_subarray_with_sum_pos_neg(arr, 7);
		println!("=>> what is s: {s}");
		assert_eq!(6, s);
	}

	#[test]
	fn test_dutch_national_flag() {
		let mut arr = [1, 2, 0, 1, 1, 1, 0, 0, 2, 2, 0, 1];
		arr.dutch_national_flag_problem();

		assert!(arr.eq(&[0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2]));
	}

	#[test]
	fn test_max_profit() {
		let arr = [1, 2, 1, 3, 4, 0, 7, 4];
		let max_profit = stock_buy_and_sell(&arr);

		println!("=>> max profit = {max_profit}");
	}

	#[test]
	fn test_rearrange_sign() {
		let arr = Vec::from([1, -2, -1, 2, 5, -3]);
		let arr2 = rearrange_alt_sign(arr);
		println!("=>> arr = {arr2:?}");
	}
}
