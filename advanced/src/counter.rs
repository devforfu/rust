mod counter {
    pub struct Counter {
        current: u32,
        limit: u32,
    }

    impl Counter {
        pub fn new(limit: u32) -> Counter {
            Counter {
                current: 0,
                limit,
            }
        }

        pub fn current(self) -> u32 {
            self.current
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.limit {
                let value = Some(self.current);
                self.current += 1;
                value
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::counter::Counter;

    #[test]
    fn test_new_counter_creation() {
        let counter = Counter::new(5);

        assert_eq!(counter.current(), 0);
    }

    #[test]
    fn test_counter_is_iterable() {
        let mut counter = Counter::new(3);

        assert_eq!(counter.next(), Some(0));
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn test_counter_generates_list_of_values() {
        let counter = Counter::new(3);

        let values: Vec<u32> = counter.into_iter().collect();

        assert_eq!(values, vec![0, 1, 2]);
    }
}