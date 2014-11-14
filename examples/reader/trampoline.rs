#![feature(phase)]
#![feature(unboxed_closures)]

#[phase(link, plugin)]
extern crate monad_macros;
extern crate monad;

use std::collections::BTreeMap;

use monad::monad::reader::trampoline::{
    Reader,
    ask,
    bind,
    point,
};

#[deriving(Clone)]
#[deriving(Show)]
enum Exp
{
    Add(Box<Exp>, Box<Exp>),
    Val(uint),
    Var(String),
    Let(String, Box<Exp>, Box<Exp>)
}

#[allow(dead_code)]
fn eval<'a>(e: Exp) -> Reader<'a, BTreeMap<String, uint>, Option<uint>> {
    match e {
        Add(box e1, box e2) => {
            mdo! {
                o1: Option<uint> <- eval(e1);
                o2: Option<uint> <- eval(e2);
                let res =
                    o1.and_then(|v1|
                    o2.and_then(|v2|
                        Some(v1 + v2)
                    ));
                end point(res)
            }
        },
        Let(x, box e1, box e2) => {
            mdo! {
                o1: Option<uint> <- eval(e1);
                end eval(e2).local(move |:mut ctx: BTreeMap<String, uint>| {
                    o1.map(|v1| (&mut ctx).insert(x.clone(), v1));
                    ctx
                })
            }
        },
        Val(n) => point(Some(n)),
        Var(x) => {
            mdo! {
                ctx: BTreeMap<String, uint> <- ask();
                end point(ctx.get(&x).map(|x| *x))
            }
        },
    }
}

#[allow(dead_code)]
fn main() {
    fn exp_test(var: &str) -> Exp {
        Let(String::from_str("x"),
            box Val(42u),
            box Add(
                box Let(String::from_str("y"),
                    box Val(43u),
                    box Add(
                        box Var(String::from_str("x")),
                        box Var(String::from_str("y"))
                    )
                ),
                box Var(String::from_str(var))
            )
        )
    }

    println!("{}", eval(exp_test("x")).run(BTreeMap::new()));
    println!("{}", eval(exp_test("y")).run(BTreeMap::new()));
}
