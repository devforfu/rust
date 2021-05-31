use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

trait OutlineFormat: fmt::Display {
    fn outline_format(&self) -> String {
        let basic = String::from(self.to_string());
        let len = basic.len();
        let mut buf = String::new();
        buf.push_str(&format!("{}\n", "*".repeat(len + 4)));
        buf.push_str(&format!("*{}*\n", " ".repeat(len + 2)));
        buf.push_str(&format!("* {} *\n", basic));
        buf.push_str(&format!("*{}*\n", " ".repeat(len + 2)));
        buf.push_str(&format!("{}\n", "*".repeat(len + 4)));
        buf
    }
}

impl OutlineFormat for Point {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_using_supertrait_for_fancy_formatting() {
        let point = Point {x: 0, y: 1};
        let expected = "**********\n\
                             *        *\n\
                             * (0, 1) *\n\
                             *        *\n\
                             **********\n";

        let actual = point.outline_format();
        
        assert_eq!(actual, expected);
    }

}
