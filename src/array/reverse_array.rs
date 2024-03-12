use super::InPlaceSliceOperations;

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
}
