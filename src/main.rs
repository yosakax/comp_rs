use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use std::mem::swap;

use num::{integer::Roots, traits::Pow, ToPrimitive};
use proconio::input;
use proconio::marker::{Chars, Usize1};

// use std::io::*;

fn solve() {
    input! {
        N:usize,
        M:usize,
        K:usize,
        AB:[[f64; 2]; N],
        CD:[[f64; 2]; M],
    }
    println!("{}, {}, {}", N, M, K)
}

fn main() {
    // input! {N:usize}
    // for _ in 0..N {
    //     solve();
    // }
    solve();
}
