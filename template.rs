/// lowprime sieve: linear, sieve, primes, divisor
/// computes lp[i] where lp[i] is the lowest prime divisor of i and pr, where pr[i] is the i-th prime number
/// C: O(n), R: nothing
fn lp(n: U) {
    let mut lp = vec![0; n + 1];
    let mut pr = vec![];
    for i in 2..=n {
        if lp[i] == 0 {
            lp[i] = i;
            pr.push(i);
        }
        let mut j = 0;
        while i * pr[j] <= n {
            lp[i * pr[j]] = pr[j];
            if pr[j] == lp[i] {
                break;
            }
            j += 1;
        }
    }
}

/// parse graph, input parsing, undirected
/// Parses the size and edges for an UNDIRECTED graph and generates the edge list
/// assumes that indexes are 1-based
/// R: io: Io
let (n, m) = r!(io, U, U);
let mut e = vec![vec![]; n];
for _ in 0..m {
    let (mut f, mut t) = r!(io, U, U);
    f -= 1;
    t -= 1;
    e[f].push(t);
    e[t].push(f);
}

/// DFS, graph, depth first search
/// C: O(n + |e|), R: n: U, e: &Vec<Vec<U>>
let start = 0;
let mut v = vec![false; n];
let mut p = vec![start; n];
let mut s = vec![start];
while let Some(i) = s.pop() {
    if v[i] {
        continue;
    }
    v[i] = true;
    for &j in e[i].iter() {
        if !v[j] {
            p[j] = i;
            s.push(j);
        }
    }
}

/// BFS, graph, breadth first search
/// C: O(n + |e|), R: n: U, e: &Vec<Vec<U>>
let start = 0;
let mut v = vec![false; n];
let mut p = vec![start; n];
let mut s = std::collections::VecDeque::<U>::new();
s.push_front(start);
while let Some(i) = s.pop_front() {
    if v[i] {
        continue;
    }
    v[i] = true;
    for &j in e[i].iter() {
        if !v[j] {
            p[j] = i;
            s.push_back(j);
        }
    }
}

/// GCD, greatest common denominator
/// C: O(log(min(a, b))), R: nothing
fn gcd(mut a: U, mut b: U) -> U {
    assert!(a != 0 && b != 0);
    if b > a {
        core::mem::swap(&mut a, &mut b);
    }
    let mut r = a % b;
    while r > 0 {
        a = b;
        b = r;
        r = a % b;
    }
    return b;
}

/// segment tree, range queries, configured for max
/// can do: max, min, plus, times, xor, and, or
/// C: O(n*log n), R: nothing
#[inline(always)]
fn id() -> U {
    0
}
#[inline(always)]
fn comb(a: U, b: U) -> U {
    a.min(b)
}
struct SegTree {
    buf: Vec<U>,
}
impl SegTree {
    fn build(mut buf: Vec<U>) -> Self {
        let n = buf.len();
        buf.reserve_exact(n);
        for i in 0..n {
            buf.push(unsafe { *buf.get_unchecked(i) })
        }
        for i in (0..n).rev() {
            buf[i] = comb(buf[i << 1], buf[i << 1 | 1]);
        }
        Self { buf }
    }
    fn query(&self, mut l: U, mut r: U) -> U {
        let mut res = id();
        l += self.buf.len() >> 1;
        r += (self.buf.len() >> 1) + 1;
        while l < r {
            if l & 1 == 1 {
                res = comb(res, self.buf[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res = comb(res, self.buf[r]);
            }
            l >>= 1;
            r >>= 1;
        }
        res
    }
    fn modify(&mut self, mut idx: U, val: U) -> U {
        idx += self.buf.len() >> 1;
        let res = core::mem::replace(&mut self.buf[idx], val);
        while {
            idx >>= 1;
            idx > 0
        } {
            self.buf[idx] = comb(self.buf[idx << 1], self.buf[idx << 1 | 1]);
        }
        res
    }
}
