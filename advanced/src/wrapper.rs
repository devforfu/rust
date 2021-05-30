use std::fmt;

struct PrettyVec(Vec<String>);

impl fmt::Display for PrettyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pretty_vec_formatting() {
        let v = PrettyVec(vec![String::from("first"), String::from("second")]);
        
        let s = format!("{}", v);
        
        assert_eq!(s, "[first, second]");
    }
}
