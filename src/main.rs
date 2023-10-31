#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
use amplify::confinement::Collection;
use lazy_static::lazy_static;
use num_bigint::BigInt;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use std::hash::Hash;
use std::mem::swap;
use std::ops::Index;
use std::ops::Mul;

use itertools::{CombinationsWithReplacement, Itertools};
use libm::{ceil, log};
use num::{integer::Roots, traits::Pow, ToPrimitive};
use num_integer::{div_ceil, Integer};
use num_traits::{abs_sub, AsPrimitive, Float};
use permutohedron::{Heap, LexicalPermutation};
use proconio::marker::{Chars, Usize1};
use proconio::{input, source::line::LineSource};
use std::convert::From;
use std::{io::*, string};

fn solve() {
    #[rustfmt::skip]
    input! {
    }
}

fn main() {
    // input! {N:usize}
    // for _ in 0..N {
    //     solve();
    // }
    solve();
}
