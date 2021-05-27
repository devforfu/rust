use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn leaf(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }

    fn new(value: i32, children: Vec<Rc<Node>>) -> Rc<Node> {
        Rc::new(Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(children),
        })
    }

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
        let leaf = Node::leaf(3);
        let branch = Node::new(5, vec![Rc::clone(&leaf)]);

        let branch_and_leaf = !branch.is_leaf() && leaf.is_leaf();

        assert!(branch_and_leaf);
        assert_eq!(branch.get_child(0).value, leaf.value);
    }

    #[test]
    fn test_node_parent_reference_as_weak_reference_added() {
        let child = Node::leaf(3);
        let parent = Node::new(5, vec![Rc::clone(&child)]);

        let none_before = child.parent.borrow().upgrade().is_none();
        *child.parent.borrow_mut() = Rc::downgrade(&parent);
        let some_after = child.parent.borrow().upgrade().is_some();

        assert!(none_before && some_after);
        assert_eq!(Rc::strong_count(&parent), 1);
        assert_eq!(Rc::weak_count(&parent), 1);
    }

    #[test]
    fn test_node_parent_weak_reference_points_to_none_when_leaves_scope() {
        let child = Node::leaf(3);

        let child_strong_count;
        let parent_weak_count;
        {
            let parent = Node::new(5, vec![Rc::clone(&child)]);
            *child.parent.borrow_mut() = Rc::downgrade(&parent);
            child_strong_count = Rc::strong_count(&child);
            parent_weak_count = Rc::weak_count(&parent);
        }

        assert_eq!(child_strong_count, 2);
        assert_eq!(parent_weak_count, 1);
        assert_eq!(Rc::strong_count(&child), 1);
        assert_eq!(Rc::weak_count(&child), 0);
        assert!(child.parent.borrow().upgrade().is_none());
    }
}
