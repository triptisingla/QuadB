use std::cmp;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}


fn maxDepth(root: Option<&Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = maxDepth(node.left.as_ref());
            let right_depth = maxDepth(node.right.as_ref());
            cmp::max(left_depth, right_depth) + 1
        }
        None => 0,
    }
}



fn main() {
    let mut root = TreeNode::new(3);
    root.left = Some(Box::new(TreeNode::new(9)));
    root.right = Some(Box::new(TreeNode::new(20)));
    // root.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(15)));
    root.right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(7)));

    println!("Maximum depth of the binary tree: {}", maxDepth(Some(&Box::new(root))));
}
