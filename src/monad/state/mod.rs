pub struct State<'s,S,A>(proc(S):'s -> (A,S));

impl <'s,S,A> State<'s,S,A>
{
    pub fn run(self, s:S) -> (A,S) {
        let State(state) = self;
        state(s)
    }

    pub fn bind<B>(self, f:proc(A) -> State<'s,S,B>) -> State<'s,S,B> {
        State(proc(s) {
            let (a,t) = self.run(s);
            f(a).run(t)
        })
    }

    pub fn seq<B>(self, m:State<'s,S,B>) -> State<'s,S,B> {
        self.bind(proc(_) m)
    }
}

pub fn point<'s,S,A>(a:A) -> State<'s,S,A> {
    State(proc(s) (a,s))
}

pub fn get<'s,S>() -> State<'s,S,S>
    where
        S:Clone,
{
    State(proc(s:S) { (s.clone(), s) })
}

pub fn put<'s,S>(s:S) -> State<'s,S,()> {
    State(proc(_) { ((), s) })
}
