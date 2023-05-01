fn modpow(mut x: i64, mut n: i64, m: i64) -> i64 {
    let mut ans = 1;
    while n > 0 {
        if n % 2 == 1 {
            ans = (ans * x) % m;
        }
        x = (x * x) % m;
        n >>= 1;
    }
    return ans % m;
}

fn modpow(mut a: i64, mut p: i64, MOD: i64) -> i64 {
    // 繰り返し2乗法による高速べき乗mod
    let mut ret = 1;
    while p > 0 {
        if p % 2 == 1 {
            ret *= a;
        }
        a *= a;
        p >>= 1;
        a %= MOD;
        ret %= MOD;
    }
    return ret;
}
