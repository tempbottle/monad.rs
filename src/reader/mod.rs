pub struct Reader<'r,E,A>
    where
        A:'r,
        E:Clone,
{
    pub run: Box<FnOnce<(E,), A> + 'r>
}

// impl<'r,E,A> FnOnce<(E,),A> for Reader<'r,E,A>
//     where
//         E:Clone,
// {
//     extern "rust-call" fn call_once(self, (e,):(E,)) -> A {
//         self.run.call_once((e,))
//     }
// }

impl <'r,E,A> Reader<'r,E,A>
    where
        E:Clone,
{

    pub fn ret(x:A) -> Reader<'r,E,A>
    {
        Reader {
            run: box move |:_| {
                x
            }
        }
    }

    pub fn and_then<'s,B:'s,F:FnOnce<(A,),Reader<'s,E,B>> + 'r + 's>(self, f:F) -> Reader<'r,E,B>
        where
            B:Clone,
    {
        Reader {
            run: box move |:e:E| {
                f.call_once((self.run.call_once((e.clone(),)),)).run.call_once((e,))
            }
        }
    }

    pub fn within<F:FnOnce<(E,),E> + 'r>(self, f:F) -> Reader<'r,E,A>
    {
        Reader { run: box move |:e| { self.run.call_once((f.call_once((e,)),)) } }
    }

}

pub fn ask<'r,E>() -> Reader<'r,E,E>
    where
        E:Clone,
{
    Reader { run: box move |:e| { e } }
}
