use std::rc::Rc;

#[derive(Debug)]
enum InternalList<T> {
    Nil,
    Cons(Rc<T>, Rc<InternalList<T>>)
}

#[derive(Debug)]
pub struct List<T> {
    xs: Rc<InternalList<T>>
}

fn count<T> (xs:Rc<InternalList<T>>) -> usize
{
    match *xs.clone() {
        InternalList::Nil => 0,
        InternalList::Cons(_, ref cdr) => 1 + count (cdr.clone())
    }
}

fn map<F,T> (f:F, xs:Rc<InternalList<T>>) -> Rc<InternalList<T>> where F:Fn(&T)->T
{
    match *xs.clone() {
        InternalList::Nil => xs.clone(),
        InternalList::Cons(ref car, ref cdr) => Rc::new(InternalList::Cons (Rc::new(f(car)), map (f, cdr.clone())))
    }
}

impl <T> List<T> {
    pub fn new () -> List<T> {
        let n:InternalList<T> = InternalList::Nil;
        List {xs: Rc::new(n)}
    }
    
    pub fn cons (&self, x:T) -> List<T> {
        List {xs: Rc::new(InternalList::Cons(Rc::new(x),self.xs.clone()))}
    }
    
    pub fn car (&self) -> Option<Rc<T>> {
        match *self.xs {
            InternalList::Nil => None,
            InternalList::Cons(ref car,_) => Some(car.clone())
        }
    }

    pub fn cdr (&self) -> List<T> {
        match *self.xs.clone() {
            InternalList::Nil => List {xs: self.xs.clone()},
            InternalList::Cons(_, ref cdr) => List{xs: cdr.clone ()}
        }
    }

    pub fn count (&self) -> usize {
        count (self.xs.clone())
    }

    pub fn map<F> (&self, f:F) -> List<T> where F:Fn(&T)->T {
        List {xs: map(f,self.xs.clone())}
    }
}
