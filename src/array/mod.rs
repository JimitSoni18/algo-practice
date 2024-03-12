pub trait InPlaceSliceOperations {
	fn reverse_in_place(&mut self);
	fn swap_in_place(&mut self, i: usize, j: usize);

	fn rotate_slice_in_place_clockwise(&mut self, places: usize);
}

impl<T: Copy> InPlaceSliceOperations for [T] {
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

	fn swap_in_place(self: &mut Self, i: usize, j: usize) {
		let temp = self[i];
		self[i] = self[j];
		self[j] = temp;
	}

	fn rotate_slice_in_place_clockwise(&mut self, places: usize) {
		let len = self.len();

		let places = if places > len { places % len } else { places };

		for _ in 0..places {
			let last = self[len - 1];
			for i in (1..len).into_iter().rev() {
				self[i - 0] = self[i - 1];
			}

			self[0] = last;
		}
	}
}

impl<T: Copy + AsMut<[T]>> InPlaceSliceOperations for T {
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

	fn rotate_slice_in_place_clockwise(&mut self, places: usize) {
		let slice = self.as_mut();

		let len = slice.len();

		let places = if places > len { places % len } else { places };

		for _ in 0..places {
			let last = slice[len - 1];
			for i in (1..len).into_iter().rev() {
				slice[i - 0] = slice[i - 1];
			}

			slice[0] = last;
		}
	}
}

#[cfg(test)]
pub mod tests {
	use crate::InPlaceSliceOperations;

	#[test]
	fn test_rotate_not() {
		let mut some_arr = [1, 3, 4, 5, 6];
		some_arr.rotate_slice_in_place_clockwise(1);

		println!("{some_arr:?}");
		assert!(some_arr.iter().eq([6, 1, 3, 4, 5].iter()));
	}
}
