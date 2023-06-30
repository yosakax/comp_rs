fn bisect_left<T: std::cmp::PartialOrd>(lst: &Vec<T>, val: T) -> usize {
    let mut left = 0;
    let mut right = lst.len();
    while right - left > 1 {
        let mid = (left + right) / 2;
        if lst[mid] <= val {
            left = mid;
        } else {
            right = mid;
        }
    }
    return left;
}

fn bisect_right<T: std::cmp::PartialOrd>(lst: &Vec<T>, val: T) -> usize {
    let mut left = 0;
    let mut right = lst.len();
    while right - left > 1 {
        let mid = (left + right) / 2;
        if lst[mid] < val {
            left = mid;
        } else {
            right = mid;
        }
    }
    return left;
}
