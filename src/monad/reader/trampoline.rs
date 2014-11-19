use tailrec::trampoline::{
    Trampoline,
    done,
    more,
};

pub struct Reader<'a, R, A>(Box<FnOnce<(R,), Trampoline<'a, A>> + 'a>);

impl <'a, R:'a, A:'a> Reader<'a, R, A>
    where
        R: Clone,
{
    #[inline]
    fn trampoline(self, r: R) -> Trampoline<'a, A> {
        let Reader(f) = self;
        f.call_once((r,))
    }

    #[inline]
    pub fn run(self, r: R) -> A {
        self.trampoline(r).run()
    }

    #[inline]
    pub fn bind<B:'a, F:'a>(self, f: F) -> Reader<'a, R, B>
        where
            F: FnOnce(A) -> Reader<'a, R, B>,
    {
        Reader(box move |:r: R| {
            more(box move |:| {
                self.trampoline(r.clone()).bind(move |:a| {
                    more(box move |:| {
                        f.call_once((a,)).trampoline(r)
                    })
                })
            })
        })
    }

    #[inline]
    pub fn local<F:'a>(self, f: F) -> Reader<'a, R, A>
        where
            F: FnOnce(R) -> R,
    {
        Reader(box move |:r| self.trampoline(f(r)))
    }
}

#[inline]
pub fn bind<'a, R:'a, A:'a, B:'a, F:'a>(m: Reader<'a, R, A>, f: F) -> Reader<'a, R, B>
    where
        F: FnOnce(A) -> Reader<'a, R, B>,
        R: Clone,
{
    m.bind(f)
}

#[inline]
pub fn point<'a, R, A:'a>(a: A) -> Reader<'a, R, A> {
    Reader(box move |:_| done(a))
}

#[inline]
pub fn ask<'a, R>() -> Reader<'a, R, R> {
    Reader(box |:r| done(r))
}
