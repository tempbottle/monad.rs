pub struct Reader<'r,E,A>
    where
        A:'r,
        E:Clone,
{
    pub run: proc(E):'r -> A
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
            run: proc(_) {
                x
            }
        }
    }

    pub fn and_then<B>(self, f:proc(A) -> Reader<'r,E,B>) -> Reader<'r,E,B>
        where
            B:Clone,
    {
        Reader {
            run: proc(e:E) {
                (f((self.run)(e.clone())).run)(e)
            }
        }
    }

    pub fn within(self, f:proc(E) -> E) -> Reader<'r,E,A>
    {
        Reader {
            run: proc(e) {
                (self.run)(f(e))
            }
        }
    }

}

pub fn ask<'r,E>() -> Reader<'r,E,E>
    where
        E:Clone,
{
    Reader {
        run: proc(e) {
            e
        }
    }
}
