use std::rc::Rc;

enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

impl<T: Copy> List<T> {
    fn vec(&self) -> Vec<T> {
        let mut root = self;
        let mut vector: Vec<T> = Vec::new();
        loop {
            root = match root {
                List::Cons(value, rest) => {
                    vector.push(*value);
                    rest
                },
                List::Nil => break
            };
        }
        vector
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list::List::{Cons, Nil};

    #[test]
    fn test_list_converts_to_vec() {
        let list = Cons(1, Rc::new(Cons(2, Rc::new(Nil))));
        let vector = list.vec();
        assert_eq!(vector, vec![1, 2]);
    }
}
