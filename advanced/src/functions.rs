fn inc(x: i32) -> i32 { x + 1 }

fn twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn create_adder(step: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + step)
}

mod tests {
    use super::*;

    #[test]
    fn test_apply_func_twice() {
        assert_eq!(twice(inc, 0), 2);
    }

    #[test]
    fn test_create_adder_closure() {
        let add_two = create_adder(2);

        assert_eq!(add_two(0), 2);
    }
}
