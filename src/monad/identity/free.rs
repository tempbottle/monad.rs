pub struct Sig<'a, X>;

fn absurd<'a, X, A>(_: Sig<'a, X>) -> A {
    unreachable!()
}

fn map<'a, X, Y, F:'a>(m: Sig<'a, X>, _: F) -> Sig<'a, Y>
    where
    F: FnOnce(X) -> Y,
{
    absurd(m)
}

monad!(Identity, Sig, map, [])

impl<'a, A> Identity<'a, A>
{
    #[inline]
    pub fn run(self) -> A {
        match self.resume() {
            Ok(a) => return a,
            Err(sm) => absurd(sm),
        }
    }
}
