#![feature(overloaded_calls)]
#![feature(unboxed_closures)]

extern crate monad;

use std::collections::BTreeMap;

use monad::monad::reader::{
    Reader,
    ask,
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

fn eval<'r>(e:Exp) -> Reader<'r,BTreeMap<String,uint>,Option<uint>> {
    match e {
        Add(box e1, box e2) => {
            eval(e1).and_then(proc(o1:Option<uint>) {
            eval(e2).and_then(proc(o2:Option<uint>) {
                let res =
                    o1.and_then(|v1| {
                    o2.and_then(|v2| {
                        Some(v1 + v2)
                    })});
                Reader::ret(res)
            })})
        },
        Let(x, box e1, box e2) => {
            eval(e1).and_then(proc(o1:Option<uint>) {
            eval(e2).local(proc(mut ctx:BTreeMap<String,uint>) {
                o1.map(|v1| { (&mut ctx).insert(x.clone(), v1); });
                ctx
            })})
        },
        Val(n) => {
            Reader::ret(Some(n))
        },
        Var(x) => {
            ask().and_then(proc (ctx:BTreeMap<String,uint>) {
                Reader::ret(ctx.find(&x).map(|x| { *x }))
            })
        },
    }
}

fn main() {
    fn exp_test(var:&str) -> Exp {
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

    println!("{}", (eval(exp_test("x")).run)(BTreeMap::new()));
    println!("{}", (eval(exp_test("y")).run)(BTreeMap::new()));
}
