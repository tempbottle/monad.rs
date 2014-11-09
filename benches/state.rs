#![feature(tuple_indexing)]

// external crates
extern crate quickcheck;
extern crate test;

// local crates
extern crate monad;

// external imports
use quickcheck::{
    Arbitrary,
    StdGen,
};

// local mod imports
use monad::monad::state;

// custom mod imports
#[path="../examples/state.rs"]
mod counter;

#[bench]
fn arith_native(b:&mut test::Bencher) -> () {
    let gen = &mut StdGen::new(std::rand::task_rng(), 100);
    let fst:  int = Arbitrary::arbitrary(gen);
    let bnd: uint = Arbitrary::arbitrary(gen);
    let mut res = fst;
    let task = || {
        for _ in range(0u, bnd) {
            res += 1i;
        }
        for _ in range(0u, bnd) {
            res -= 1i
        };
        fst == res
    };
    b.iter(task)
}

#[bench]
fn arith_state(b:&mut test::Bencher) -> () {
    let gen = &mut StdGen::new(std::rand::task_rng(), 100);
    let fst:  int = Arbitrary::arbitrary(gen);
    let bnd: uint = Arbitrary::arbitrary(gen);
    let task = || {
        let mut mon = state::point(());
        for _ in range(0u, bnd) {
            mon = mon.seq(counter::incr())
        }
        for _ in range(0u, bnd) {
            mon = mon.seq(counter::decr())
        }
        fst == mon.run(fst).1
    };
    b.iter(task)
}
