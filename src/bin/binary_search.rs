pub trait BinarySearch<T> {
    fn binary_search(&self, value: T) -> Option<usize>;
}

impl<T: std::cmp::Ord + Copy> BinarySearch<T> for Vec<T> {
    fn binary_search(&self, value: T) -> Option<usize> {
        let (mut start, mut end): (usize, usize) = (0, self.len() - 1);
        while start <= end {
            let mid = (start + end) / 2;
            let guess = self[mid];
            if guess == value {
                return Some(mid);
            } else if guess > value {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        return None;
    }
}

fn main() {
    let v: Vec<i32> = vec![0, 3, 6, 9];
    assert_eq!(0, v.binary_search(0).unwrap());
    assert_eq!(2, v.binary_search(6).unwrap());
    assert_eq!(None, v.binary_search(4));
}
