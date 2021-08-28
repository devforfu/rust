pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

pub struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
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

}