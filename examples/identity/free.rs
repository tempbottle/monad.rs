#![feature(phase)]
#![feature(unboxed_closures)]

#[phase(link, plugin)]
extern crate monad_macros;
extern crate monad;

use monad::monad::identity::free::{
    bind,
    point,
};

#[allow(dead_code)]
fn main() {
    let mut res = point(0u);
    for _ in range (0u, 100000) {
        res = mdo! {
            x: uint <- res;
            end point(x + 42u)
        };
    }
    println!("{}", res.run());
}
