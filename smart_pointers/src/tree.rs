use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn is_leaf(&self) -> bool {
        self.children.borrow().len() == 0
    }

    fn get_child(&self, index: usize) -> Rc<Node> {
        Rc::clone(&self.children.borrow()[index])
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_node_creation() {
        let leaf = Rc::new(Node {
            value: 3,
            children: RefCell::new(vec![]),
        });
        let branch = Rc::new(Node {
            value: 3,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        let branch_and_leaf = !branch.is_leaf() && leaf.is_leaf();

        assert!(branch_and_leaf);
        assert!(branch.get_child(0).value == leaf.value);
    }
}
