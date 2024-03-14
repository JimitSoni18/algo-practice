pub trait SearchSlice {
	type Item;
	fn linear_search(&self, elem: &Self::Item) -> isize
	where
		Self::Item: Eq;
	fn binary_search_recursive(&self, elem: &Self::Item) -> isize
	where
		Self::Item: Ord;
}

impl<T: Eq> SearchSlice for [T] {
	type Item = T;

	fn linear_search(&self, elem: &Self::Item) -> isize {
		let mut index: isize = -1;
		let len = self.len();
		let cur = 0;
		while cur < len {
			if self[cur] == *elem {
				index = cur as isize;
				break;
			}
		}

		return index;
	}

	// FIXME: BOTCHED IMPLEMENTATION, NEED TO REIMPLEMENT,
	// `mid` CALCULATED INCORRECTLY
	fn binary_search_recursive(&self, elem: &Self::Item) -> isize
	where
		Self::Item: Ord,
	{
		fn binary_search_inner<T: Ord>(slice: &[T], start: usize, end: usize, elem: &T) -> isize {
			let len = end - start;
			let mid = (len + (len % 2)) / 2;

			// if len == 1 {
			// 	if self[]
			// }
			// println!("=>> what is mid: {mid}, len: {len}");

			if slice[mid] == *elem {
				return mid as isize;
			} else if slice[mid] < *elem {
				return binary_search_inner(slice, mid + 1, end, elem);
			} else if slice[mid] > *elem {
				return binary_search_inner(slice, start, mid - 1, elem);
			} else {
				panic!("wtf");
			}
		}

		return binary_search_inner(self, 0, self.len(), elem);
	}
}

#[cfg(test)]
mod tests {
	use super::SearchSlice;

	#[test]
	fn test_binary_search_working() {
		let slice = &[1, 3, 5, 7, 8];

		let ind = slice.binary_search_recursive(&7);

		assert_eq!(ind, 3);
	}

	#[test]
	fn test_binary_search_not_found() {
		let slice = &[1, 3, 5, 7, 8];

		let ind = slice.binary_search_recursive(&9);

		println!("=>> got index: {ind}");

		assert_eq!(ind, -1);
	}
}
