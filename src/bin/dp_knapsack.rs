use proconio::*;

use std::cmp::*;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize, W: usize,
        vw: [[usize; 2]; N],
    }
    let mut dp = vec![vec![0; W + 1]; N + 1];
    for i in 0..N {
        for w in 0..W + 1 {
            dp[i + 1][w] = if vw[i][1] <= w {
                max(dp[i][w - vw[i][1]] + vw[i][0], dp[i][w])
            } else {
                dp[i][w]
            };
        }
    }
    println!("{}", dp[N][W]);
}
