extern crate monad;

use monad::monad::state;
use monad::monad::state::{
    State,
};

fn incr<'s>() -> State<'s,int,()>
{
    state::get().bind(proc(a:int) {
    state::put(a + 1i)
    })
}

fn decr<'s>() -> State<'s,int,()>
{
    state::get().bind(proc(a:int) {
    state::put(a - 1i)
    })
}

fn main() {
    let mon =
        incr().seq(
        incr().seq(
        decr().seq(
        decr()
        )));
    let fst = 42i;
    let res = mon.run(fst);
    println!("{} == {} is {}", fst, res.1, fst == res.1);
}
