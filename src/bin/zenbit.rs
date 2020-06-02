#[allow(non_snake_case)]
fn main() {
    let N = 3;
    for bit in 0..1 << N {
        let mut array: Vec<i32> = Vec::new();
        for i in 0..N {
            if bit & (1 << i) != 0 {
                array.push(i)
            }
        }
        println!("{:?}", array);
    }
}
