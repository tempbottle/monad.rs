use self::StateF::{
    Get,
    Put,
};

pub enum StateF<'a, S, X> {
    Get(Box<FnOnce<(S,), X> + 'a>),
    Put(S, X),
}

pub fn map<'a, S, X, Y, F:'a>(m: StateF<'a, S, X>, f: F) -> StateF<'a, S, Y>
    where
        F: FnOnce(X) -> Y,
{
    match m {
        Get(g) => Get(box move |:s| f.call_once((g.call_once((s,)),))),
        Put(s, a) => Put(s, f.call_once((a,))),
    }
}

free_monad!(State, StateF, map, [ S, ])

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

    #[inline]
    pub fn seq<B:'a>(self, m: State<'a, S, B>) -> State<'a, S, B> {
        self.bind(move |:_| m)
    }
}

#[inline]
pub fn bind<'a, S:'a, A:'a, B:'a, F:'a>(m: State<'a, S, A>, f: F) -> State<'a, S, B>
    where
        F: FnOnce(A) -> State<'a, S, B>,
{
    m.bind(f)
}

#[inline]
pub fn point<'a, S, A:'a>(a: A) -> State<'a, S, A> {
    State::Leaf(a)
}

#[inline]
pub fn get<'a, S>() -> State<'a, S, S>
{
    State::Nest(Get(box |:s| box State::Leaf(s)))
}

#[inline]
pub fn put<'a, S:'a>(s: S) -> State<'a ,S, ()> {
    State::Nest(Put(s, box State::Leaf(())))
}
