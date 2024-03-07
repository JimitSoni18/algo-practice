pub fn reverse_array<T: Copy>(arr: &mut [T]) {
	reverse_array_inner(arr, 0, arr.len() - 1);
}

fn reverse_array_inner<T: Copy>(arr: &mut [T], i: usize, j: usize) {
	if i < j {
		swap(arr, i, j);
		reverse_array_inner(arr, i + 1, j - 1);
	}
}

fn swap<T: Copy>(arr: &mut [T], i: usize, j: usize) {
	let temp = arr[i];
	arr[i] = arr[j];
	arr[j] = temp;
}
