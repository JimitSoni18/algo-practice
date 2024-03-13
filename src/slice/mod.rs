pub trait InPlaceSliceOperations {
	type Item;
	fn reverse_in_place(&mut self);
	fn swap_in_place(&mut self, i: usize, j: usize);

	fn rotate_slice_in_place_right(&mut self, places: usize);
	fn rotate_slice_in_place_left(&mut self, places: usize);
}

impl<T: Copy> InPlaceSliceOperations for [T] {
	type Item = T;
	/// Reverse slice with the time complexity of O(n)
	/// and space complexity of O(1)
	fn reverse_in_place(self: &mut Self) {
		#[inline]
		fn reverse_inner<T: Copy>(slice: &mut [T], i: usize, j: usize) {
			if i < j {
				slice.swap_in_place(i, j);
				reverse_inner(slice, i + 1, j - 1);
			}
		}
		reverse_inner(self, 0, self.len() - 1);
	}

	/// Swap elements in slice with time and space complexity of O(1)
	fn swap_in_place(self: &mut Self, i: usize, j: usize) {
		let temp = self[i];
		self[i] = self[j];
		self[j] = temp;
	}

	/// Rotate the slice to right with time complexity O(n)
	/// and space complexity O(n)
	fn rotate_slice_in_place_right(&mut self, places: usize) {
		let pivot = self.len() - places;
		self[..pivot].reverse_in_place();
		self[pivot..].reverse_in_place();
		self.reverse_in_place();
	}

	/// Rotate the slice to left with time complexity O(n)
	/// and space complexity O(n)
	fn rotate_slice_in_place_left(&mut self, places: usize) {
		self[..places].reverse_in_place();
		self[places..].reverse_in_place();

		self.reverse_in_place();
	}
}

impl<T: Copy + AsMut<[T]>> InPlaceSliceOperations for T {
	type Item = T;
	fn reverse_in_place(&mut self) {
		#[inline]
		fn reverse_inner<T: Copy>(slice: &mut [T], i: usize, j: usize) {
			if i < j {
				slice.swap_in_place(i, j);
				reverse_inner(slice, i + 1, j - 1);
			}
		}
		let slice = self.as_mut();
		reverse_inner(slice, 0, slice.len() - 1);
	}

	fn swap_in_place(&mut self, i: usize, j: usize) {
		let slice = self.as_mut();
		let temp = slice[i];
		slice[i] = slice[j];
		slice[j] = temp;
	}

	fn rotate_slice_in_place_right(&mut self, places: usize) {
		let slice = self.as_mut();
		let pivot = slice.len() - places;
		slice[..pivot].reverse_in_place();
		slice[pivot..].reverse_in_place();
		slice.reverse_in_place();
	}

	fn rotate_slice_in_place_left(&mut self, places: usize) {
		let slice = self.as_mut();
		slice[..places].reverse_in_place();
		slice[places..].reverse_in_place();

		slice.reverse_in_place();
	}
}

#[cfg(test)]
pub mod tests {
	use crate::InPlaceSliceOperations;

	#[test]
	fn test_rotate_slice_right_by_one_place() {
		let mut some_arr = [1, 3, 4, 5, 6];
		some_arr.rotate_slice_in_place_right(1);

		assert!(some_arr.iter().eq([6, 1, 3, 4, 5].iter()));
	}

	#[test]
	fn test_rotate_slice_right_by_multiple_places() {
		let mut some_arr = [1, 3, 4, 5, 6];
		some_arr.rotate_slice_in_place_right(3);

		assert!(some_arr.iter().eq([4, 5, 6, 1, 3].iter()));
	}

	#[test]
	fn test_rotate_slice_left_by_one_place() {
		let mut some_arr = [1, 3, 4, 5, 6];
		println!("=>> before: {some_arr:?}");
		some_arr.rotate_slice_in_place_left(1);

		println!("=>> after: {some_arr:?}");

		assert!(some_arr.iter().eq([3, 4, 5, 6, 1].iter()));
	}

	#[test]
	fn test_rotate_slice_left_by_multiple_places() {
		let mut some_arr = [1, 3, 4, 5, 6];
		println!("=>> before: {some_arr:?}");
		some_arr.rotate_slice_in_place_left(3);

		println!("=>> after: {some_arr:?}");

		assert!(some_arr.iter().eq([5, 6, 1, 3, 4].iter()));
	}
}
