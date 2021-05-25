use std::ops::Deref;

struct Item<T>(T);

impl<T> Item<T> {
    fn new(x: T) -> Item<T> {
        Item(x)
    }
}

impl<T> Deref for Item<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_item() {
        let item = Item::new(1);
        assert_eq!(1, *item);
    }
}
