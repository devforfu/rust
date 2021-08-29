pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

pub struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

pub enum TraverseMethod {
    InOrder,
}

impl <T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            },
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value)
                } else {
                    node.right.add(value)
                }
            }
        }
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

impl <T: Ord + Clone> BinaryTree<T> {

    /// Transforms a tree into a vector.
    ///
    /// Vector elements are automatically sorted as it is constructed by traversing the
    /// tree in order.
    pub fn to_vec(&self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.num_of_nodes());
        self.travers_tree(&mut vec, &TraverseMethod::InOrder);
        vec
    }

    fn travers_tree(&self, vec: &mut Vec<T>, order: &TraverseMethod) {
        match self {
            BinaryTree::Empty => {},
            BinaryTree::NonEmpty(node) => {
                match order {
                    // TraverseMethod::PreOrder => {
                    //     vec.push(node.element.clone());
                    //     node.left.travers_tree(vec, order);
                    //     node.right.travers_tree(vec, order);
                    // }
                    TraverseMethod::InOrder => {
                        node.left.travers_tree(vec, order);
                        vec.push(node.element.clone());
                        node.right.travers_tree(vec, order);
                    },
                    // TraverseMethod::PostOrder => {
                    //     node.left.travers_tree(vec, order);
                    //     node.right.travers_tree(vec, order);
                    //     vec.push(node.element.clone());
                    // }
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

}