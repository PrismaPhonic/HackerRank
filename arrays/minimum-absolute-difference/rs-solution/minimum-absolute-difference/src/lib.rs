fn minimum_absolute_difference(arr: &mut Vec<i32>) -> i32 {
    arr.sort_unstable();
    let mut min = (&arr[0] - &arr[1]).abs();
    for i in 1..arr.len()-1 {
        let diff = (&arr[i] - &arr[i+1]).abs();
        if diff < min { min = diff };
    }
    min
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn test0() {
        assert_eq!(minimum_absolute_difference(&mut vec![-7, 3, 0]), 3);
    }

    #[test]
    fn test1() {
        assert_eq!(minimum_absolute_difference(&mut vec![-59, -36, -13, 1, -53, -92, -2, -96, -54, 75]), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(minimum_absolute_difference(&mut vec![1, -3, 71, 68, 17]), 3);
    }
}
