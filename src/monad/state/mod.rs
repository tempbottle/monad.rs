pub struct State<'s,S,A>(proc(S):'s -> (A,S));

impl <'s,S,A> State<'s,S,A>
    where
        S:Clone,
{

    pub fn run(self, s:S) -> (A,S) {
        let State(state) = self;
        state(s)
    }

    pub fn point(a:A) -> State<'s,S,A> {
        State(proc(s) (a,s))
    }

    pub fn bind<B>(self, f:proc(A) -> State<'s,S,B>) -> State<'s,S,B>
        where
            B:Clone,
    {
        State(proc(s) {
            let (a,t) = self.run(s);
            f(a).run(t)
        })
    }

    pub fn get() -> State<'s,S,S> {
        State(proc(s:S) { (s.clone(), s) })
    }

    pub fn put(s:S) -> State<'s,S,()> {
        State(proc(_) { ((), s) })
    }

}
