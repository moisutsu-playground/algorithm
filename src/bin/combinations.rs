use itertools::Itertools;

fn main() {
    for perm in (0..3).combinations_with_replacement(3) {
        println!("{:?}", perm);
    }
    println!("");
    for perm in (0..5).combinations(3) {
        println!("{:?}", perm);
    }
}
