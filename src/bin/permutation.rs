use itertools::Itertools;

fn main() {
    for perm in (0..3).permutations(3) {
        println!("{:?}", perm);
    }
}
