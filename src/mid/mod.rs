use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum PredicateResult {
	YES,
	NO,
}

pub fn two_elem(arr: &[i32], target: i32) -> PredicateResult {
	let mut find = HashSet::new();

	for &i in arr {
		if find.contains(&i) {
			return PredicateResult::YES;
		}
		let a = target - i;
		find.insert(a);
	}

	PredicateResult::NO
}

pub fn two_elem_ind(arr: &[i32], target: i32) -> [i32; 2] {
	let mut i_map = HashMap::new();
	for i in 0..arr.len() {
		let a = arr[i];
		if i_map.contains_key(&(target - a)) {
			return [i_map.remove(&(target - a)).unwrap(), i as i32];
		}
		i_map.insert(a, i as i32);
	}

	[-1, -1]
}

#[cfg(test)]
mod tests {
	use crate::mid::two_elem_ind;

	use super::two_elem;

	#[test]
	fn test_has_elem_predicate() {
		let arr = [2, 4, 5, 8, 11];
		let r = two_elem(&arr, 14);

		println!("{r:?}");

		let r2 = two_elem_ind(&arr, 14);

		println!("{r2:?}");
	}
}
