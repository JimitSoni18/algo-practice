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

	fn binary_search_recursive(&self, elem: &Self::Item) -> isize
	where
		Self::Item: Ord,
	{
		fn binary_search_inner<T: Ord>(slice: &[T], start: usize, end: usize, elem: &T) -> isize {
			if start > end {
				return -1;
			} else if start == end {
				if *elem == slice[start] {
					return start as isize;
				}
				return -1;
			}
			let len = end - start + 1;
			let mid = start + (len - (len % 2)) / 2;

			if *elem == slice[mid] {
				return mid as isize;
			} else if *elem > slice[mid] {
				return binary_search_inner(slice, mid + 1, end, elem);
			} else if *elem < slice[mid] {
				return binary_search_inner(slice, start, mid - 1, elem);
			} else {
				panic!("wtf");
			}
		}

		binary_search_inner(self, 0, self.len() - 1, elem)
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

		let ind = slice.binary_search_recursive(&8);
		assert_eq!(ind, 4);

		let ind = slice.binary_search_recursive(&1);
		assert_eq!(ind, 0);

		let slice = &[1, 4, 6, 7, 8, 9];

		let ind = slice.binary_search_recursive(&9);
		assert_eq!(ind, 5);

		let ind = slice.binary_search_recursive(&1);
		assert_eq!(ind, 0);

		let ind = slice.binary_search_recursive(&8);
		assert_eq!(ind, 4);
	}

	#[test]
	fn test_binary_search_not_found() {
		let slice = &[1, 3, 5, 7, 8];

		let ind = slice.binary_search_recursive(&9);
		assert_eq!(ind, -1);

		let ind = slice.binary_search_recursive(&0);
		assert_eq!(ind, -1);

		let ind = slice.binary_search_recursive(&2);
		assert_eq!(ind, -1);

		let slice = &[2, 6, 7, 8, 23, 24, 26];

		let ind = slice.binary_search_recursive(&0);
		assert_eq!(ind, -1);

		let ind = slice.binary_search_recursive(&5);
		assert_eq!(ind, -1);

		let ind = slice.binary_search_recursive(&9);
		assert_eq!(ind, -1);

		let ind = slice.binary_search_recursive(&25);
		assert_eq!(ind, -1);

		let ind = slice.binary_search_recursive(&33);
		assert_eq!(ind, -1);
	}
}
