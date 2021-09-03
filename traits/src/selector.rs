use std::ops::{Deref, DerefMut};

pub struct Selector<T> {
    /// Elements available in this `Selector`.
    elements: Vec<T>,

    /// The index of the "current" element in `elements`. A `selector`
    /// behaves like a pointer to the current element.
    current: usize
}

impl<T> Selector<T> {
    /// Creates a new `Selector`.
    pub fn new(elements: Vec<T>) -> Self {
        Selector { elements, current: 0 }
    }

    pub fn select(&mut self, index: usize) {
        self.current = index;
    }
}

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements[self.current]
    }
}

#[cfg(test)]
mod tests {
    use crate::selector::Selector;

    #[test]
    fn test_selector_dereference() {
        let s = Selector::new(vec![1, 2, 3]);

        assert_eq!(*s, 1);
    }

    #[test]
    fn test_selector_mut_dereference() {
        let mut s = Selector::new(vec!['x', 'y', 'z']);

        s.select(2);
        *s = 'w';

        assert_eq!(s.elements, vec!['x', 'y', 'w']);
    }
}