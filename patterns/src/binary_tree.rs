use std::cell::RefCell;

pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

pub struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

#[derive(Debug)]
pub enum TraverseMethod {
    PreOrder,
    InOrder,
    PostOrder,
}

trait Visitor {
    type Element;
    fn visit(&self, element: Self::Element);
}

impl <T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) -> &mut Self {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }));
            },
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
        self
    }

    /// Returns the number of nodes in a tree.
    fn num_of_nodes(&self) -> usize {
        match self {
            BinaryTree::Empty => 0,
            BinaryTree::NonEmpty(node) => {
                1 + node.left.num_of_nodes() + node.right.num_of_nodes()
            }
        }
    }
}

struct VectorVisitor<T> {
    vec: RefCell<Vec<T>>
}

impl<T> VectorVisitor<T> {
    fn new(capacity: usize) -> Self {
        VectorVisitor { vec: RefCell::new(Vec::with_capacity(capacity)) }
    }
}

impl<T> Visitor for VectorVisitor<T> {
    type Element = T;

    fn visit(&self, element: T) {
        self.vec.borrow_mut().push(element);
    }
}


impl <T: Ord + Clone> BinaryTree<T> {

    /// Transforms a tree into a vector.
    ///
    /// Vector elements are automatically sorted as it is constructed by traversing the
    /// tree in order.
    pub fn to_vec(&self) -> Vec<T> {
        let visitor = VectorVisitor::new(self.num_of_nodes());
        self.traverse(&visitor, &TraverseMethod::InOrder);
        visitor.vec.take()
    }

    fn traverse<V: Visitor<Element = T>>(&self, visitor: &V, order: &TraverseMethod) {
        match self {
            BinaryTree::Empty => {},
            BinaryTree::NonEmpty(node) => {
                match order {
                    TraverseMethod::PreOrder => {
                        visitor.visit(node.element.clone());
                        node.left.traverse(visitor, order);
                        node.right.traverse(visitor, order);
                    },
                    TraverseMethod::InOrder => {
                        node.left.traverse(visitor, order);
                        visitor.visit(node.element.clone());
                        node.right.traverse(visitor, order);
                    },
                    TraverseMethod::PostOrder => {
                        node.left.traverse(visitor, order);
                        node.right.traverse(visitor, order);
                        visitor.visit(node.element.clone());
                    }
                }
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node() -> Result<(), String> {
        let mut tree = BinaryTree::Empty;

        tree.add(1);

        match tree {
            BinaryTree::NonEmpty(_) => Ok(()),
            _ => Err("tree shouldn't be empty!".to_string()),
        }
    }

    #[test]
    fn test_num_of_nodes() {
        let mut tree = BinaryTree::Empty;

        tree.add(1);
        tree.add(2);
        tree.add(3);

        assert_eq!(tree.num_of_nodes(), 3);
    }

    #[test]
    fn test_tree_to_vec() {
        let mut tree = BinaryTree::Empty;

        tree.add(5);
        tree.add(1);
        tree.add(7);
        tree.add(3);
        tree.add(9);

        assert_eq!(tree.to_vec(), vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_tree_traverse() -> Result<(), String> {
        let mut tree = BinaryTree::Empty;

        tree.add(5).add(1).add(3).add(2).add(4);

        for (order, expected) in vec![
            (TraverseMethod::PreOrder, vec![5, 1, 3, 2, 4]),
            (TraverseMethod::InOrder, vec![1, 2, 3, 4, 5]),
            (TraverseMethod::PostOrder, vec![2, 4, 3, 1, 5])
        ] {
            let visitor = VectorVisitor::new(tree.num_of_nodes());
            tree.traverse(&visitor, &order);
            let actual = visitor.vec.take();

            if actual != expected {
                return Err(format!("failed traverse order {:?}: {:?} != {:?}", order, actual, expected));
            }
        }

        Ok(())
    }
}