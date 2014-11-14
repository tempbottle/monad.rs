#![feature(phase)]
#![feature(unboxed_closures)]

#[phase(link, plugin)]
extern crate monad_macros;
extern crate monad;

use monad::monad::state::trampoline::{
    State,
    bind,
    get,
    point,
    put,
};

#[inline(always)]
pub fn incr<'a>() -> State<'a, int, ()> {
    mdo! {
        a: int <- get();
        end put(a + 1i)
    }
}

#[inline(always)]
pub fn decr<'a>() -> State<'a, int, ()> {
    mdo! {
        a: int <- get();
        end put(a - 1i)
    }
}

#[allow(dead_code)]
fn main() {
    let mut mon = point(());
    for _ in range(0u, 50000u) { mon = mon.seq(incr()) }
    for _ in range(0u, 50000u) { mon = mon.seq(decr()) }
    let fst = 42i;
    let res = mon.run(fst);
    println!("{} == {} is {}", fst, res.1, fst == res.1);
}
