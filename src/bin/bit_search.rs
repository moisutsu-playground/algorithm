pub trait BitFullSearch<T> {
    fn sorted(&self) -> Vec<T>;
    fn bit_full_search(&self) -> Vec<Vec<T>>;
}

impl<T: Copy + std::cmp::Ord> BitFullSearch<T> for Vec<T> {
    // selfを引数に入れることで,生やせるようになる
    fn sorted(&self) -> Vec<T> {
        let mut tmp_v = self.clone();
        tmp_v.sort();
        tmp_v
    }
    fn bit_full_search(&self) -> Vec<Vec<T>> {
        let mut ret_v: Vec<Vec<T>> = Vec::new();
        for bit in 0..1 << self.len() {
            let mut tmp_v: Vec<T> = Vec::new();
            for i in 0..self.len() {
                if bit & (1 << i) != 0 {
                    tmp_v.push(self[i]);
                }
            }
            ret_v.push(tmp_v);
        }
        ret_v
    }
}

fn main() {
    println!("{:?}", vec![0, 2, 1].sorted().bit_full_search());
}
