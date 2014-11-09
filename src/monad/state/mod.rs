use free::trampoline::{
    Trampoline,
    done,
    more,
};

pub struct State<'a,S,A>(proc(S):'a -> Trampoline<'a,(A,S)>);

impl<'a,S,A> State<'a,S,A> {
    #[inline]
    fn trampoline(self, s:S) -> Trampoline<'a,(A,S)> {
        let State(state) = self;
        state(s)
    }

    #[inline]
    pub fn run(self, s:S) -> (A,S) {
        self.trampoline(s).run()
    }

    #[inline]
    pub fn bind<B>(self, f:proc(A):'a -> State<'a,S,B>) -> State<'a,S,B> {
        State(proc(s) {
            more(proc() {
                self.trampoline(s).bind(proc((a,s):(A,S)) {
                    more(proc() {
                        f(a).trampoline(s)
                    })
                })
            })
        })
    }

    #[inline]
    pub fn seq<B>(self, m:State<'a,S,B>) -> State<'a,S,B> {
        self.bind(proc(_) m)
    }
}

#[inline]
pub fn point<'a,S,A>(a:A) -> State<'a,S,A> {
    State(proc(s) done((a,s)))
}

#[inline]
pub fn get<'a,S>() -> State<'a,S,S>
    where
        S:Clone,
{
    State(proc(s:S) { done((s.clone(),s)) })
}

#[inline]
pub fn put<'a,S>(s:S) -> State<'a,S,()> {
    State(proc(_) { done(((),s)) })
}
