// O(n^2)
use std::fmt::Debug;
pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for p in 0..v.len() {
        let mut sorted = true;
        for i in 0..(v.len() - 1) - p {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        println!("{:?}", v);

        if sorted {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![11, 13, 1, 2, 3, 4, 5, 9];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 9, 11, 13]);
    }
}
