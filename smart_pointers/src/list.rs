use std::rc::Rc;

enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

impl<T: Copy> List<T> {
    fn new(vec: Vec<T>) -> List<T> {
        let mut list = List::Nil;
        for x in vec.iter().rev() {
            list = List::Cons(*x, Rc::new(list));
        }
        list
    }

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

    #[test]
    fn test_list_from_vec() {
        let vec = vec![1, 2, 3];

        let list = List::new(vec);

        assert_eq!(list.vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_lists_share_same_tails() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let count_before = Rc::strong_count(&a);
        let count_in;
        {
            let _b = Cons(3, Rc::clone(&a));
            let _c = Cons(4, Rc::clone(&a));
            count_in = Rc::strong_count(&a);
        }
        let count_after = Rc::strong_count(&a);

        assert_eq!(count_before, 1);
        assert_eq!(count_in, 3);
        assert_eq!(count_after, 1);
    }
}
