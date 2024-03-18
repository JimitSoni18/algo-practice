pub fn max_subarr_sum(arr: &[i32]) -> i32 {
	let mut global_max = 0;
	let mut local_max = 0;
	for i in arr {
		local_max += i;
		if local_max > 0 {
			if local_max > global_max {
				global_max = local_max;
			}
		} else {
			local_max = 0;
		}
	}
	global_max
}

#[cfg(test)]
mod tests {
	use super::max_subarr_sum;

	#[test]
	fn test_kadane() {
		let arr = [2, 3, -6, 1, 2];

		let a = max_subarr_sum(&arr);

		println!("=>> sum = {a}");
	}
}
