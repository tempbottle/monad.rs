use self::Sig::{
    Get,
    Put,
};

pub enum Sig<'a, S, X> {
    Get(Box<FnOnce<(S,), X> + 'a>),
    Put(S, X),
}

pub fn map<'a, S, X, Y, F:'a>(m: Sig<'a, S, X>, f: F) -> Sig<'a, S, Y>
    where
        F: FnOnce(X) -> Y,
{
    match m {
        Get(g) => Get(box move |:s| f.call_once((g.call_once((s,)),))),
        Put(s, a) => Put(s, f.call_once((a,))),
    }
}

monad!(State, Sig, map, [ S, ])

impl<'a, S:'a, A:'a> State<'a, S, A>
    where
        S: Clone,
{
    #[inline]
    pub fn run(mut self, mut s: S) -> A {
        loop { match self.resume() {
            Ok(a) => return a,
            Err(Get(f)) => { self = *f.call_once((s.clone(),)) },
            Err(Put(t, a)) => { s = t; self = *a },
        }}
    }
}

#[inline]
pub fn get<'a, S>() -> State<'a, S, S>
{
    wrap(Get(box |:s| box point(s)))
}

#[inline]
pub fn put<'a, S:'a>(s: S) -> State<'a ,S, ()> {
    wrap(Put(s, box point(())))
}
