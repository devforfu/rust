use std::slice;

fn split_at<T>(slice: &[T], mid: usize) -> (&[T], &[T]) {
    let len = slice.len();

    assert!(mid <= len);

    (&slice[..mid], &slice[mid..])
}

fn split_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_vector_split_is_safe() {
        let arr = vec![1, 2, 3, 4, 5, 6];

        let (l, r) = split_at(&arr, 3);

        assert_eq!(l, &[1, 2, 3]);
        assert_eq!(r, &[4, 5, 6]);
    }

    #[test]
    fn test_mutable_vector_split_requires_unsafe() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];

        let (l, r) = split_at_mut(&mut arr, 3);
        l[0] = 10;
        r[r.len()-1] = 0;

        assert_eq!(l, &mut [10, 2, 3]);
        assert_eq!(r, &mut [4, 5, 0]);
        assert_eq!(arr, &[10, 2, 3, 4, 5, 0]);
    }
}
