#![feature(unboxed_closures)]

extern crate monad;

use monad::monad::state::{
    State,
    get,
    point,
    put,
};

#[inline(always)]
pub fn incr<'a>() -> State<'a, int, ()> {
    get().bind(|:a: int| {
    put(a + 1i)
    })
}

#[inline(always)]
pub fn decr<'a>() -> State<'a, int, ()> {
    get().bind(|:a: int| {
    put(a - 1i)
    })
}

#[allow(dead_code)]
fn main() {
    let mut mon = point(());
    for _ in range(0u, 5000u) {
        mon = mon.seq(incr())
    }
    for _ in range(0u, 5000u) {
        mon = mon.seq(decr())
    }
    let fst = 42i;
    let res = mon.run(fst);
    println!("{} == {} is {}", fst, res.1, fst == res.1);
}
