use tailrec::trampoline::{
    Trampoline,
    done,
    more,
};

pub struct State<'a, S, A>(Box<FnOnce<(S,), Trampoline<'a, (A, S)>> + 'a>);

impl<'a, S:'a, A:'a> State<'a, S, A> {
    #[inline]
    fn trampoline(self, s: S) -> Trampoline<'a, (A, S)> {
        let State(state) = self;
        state.call_once((s,))
    }

    #[inline]
    pub fn run(self, s: S) -> (A, S) {
        self.trampoline(s).run()
    }

    #[inline]
    pub fn bind<B:'a, F:'a>(self, f: F) -> State<'a, S, B>
        where
            F: FnOnce(A) -> State<'a, S, B>,
    {
        State(box move |:s| {
            more(box move |:| {
                self.trampoline(s).bind(move |:(a, s):(A, S)| {
                    more(box move |:| {
                        f.call_once((a,)).trampoline(s)
                    })
                })
            })
        })
    }

    #[inline]
    pub fn seq<B:'a>(self, m: State<'a, S, B>) -> State<'a, S, B> {
        self.bind(move |:_| m)
    }
}

#[inline]
pub fn point<'a, S, A:'a>(a: A) -> State<'a, S, A> {
    State(box move |:s| done((a, s)))
}

#[inline]
pub fn get<'a, S>() -> State<'a, S, S>
    where
        S: Clone,
{
    State(box |:s: S| { done((s.clone(), s)) })
}

#[inline]
pub fn put<'a, S:'a>(s: S) -> State<'a ,S, ()> {
    State(box move |:_| { done(((), s)) })
}
