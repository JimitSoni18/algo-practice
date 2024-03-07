pub mod reverse_array;

pub trait InPlaceSliceOperations {
	fn reverse_in_place(self: &mut Self);
	fn swap_in_place(self: &mut Self, i: usize, j: usize);
}
