fn main() {
    println!("Hello, world!");
    let a: usize = 0b010101;
    assert_eq!(a.count_ones(), 3);
}
