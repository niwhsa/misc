use std::{cell::RefCell, rc::Rc};

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: TreeLink,
    right: TreeLink,
}

impl TreeNode {
    fn new(val: i32) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }

    fn with_children(val: i32, left: TreeLink, right: TreeLink) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
}

// ============ INORDER (Left → Node → Right) ============

fn inorder_recursive(root: &TreeLink) -> Vec<i32> {
    let mut result = vec![];
    fn traverse(node: &TreeLink, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            traverse(&n.left, result);
            result.push(n.val);
            traverse(&n.right, result);
        }
    }
    traverse(root, &mut result);
    result
}

fn inorder_iterative(root: TreeLink) -> Vec<i32> {
    let mut result = vec![];
    let mut stack = vec![];
    let mut cur = root;

    while cur.is_some() || !stack.is_empty() {
        while let Some(node) = cur {
            stack.push(Rc::clone(&node));
            cur = node.borrow().left.clone();
        }

        if let Some(node) = stack.pop() {
            result.push(node.borrow().val);
            cur = node.borrow().right.clone();
        }
    }
    result
}

// ============ PREORDER (Node → Left → Right) ============

fn preorder_recursive(root: &TreeLink) -> Vec<i32> {
    let mut result = vec![];
    fn traverse(node: &TreeLink, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            result.push(n.val);
            traverse(&n.left, result);
            traverse(&n.right, result);
        }
    }
    traverse(root, &mut result);
    result
}

fn preorder_iterative(root: TreeLink) -> Vec<i32> {
    let mut result = vec![];
    let mut stack = vec![];

    if let Some(node) = root {
        stack.push(node);
    }

    while let Some(node) = stack.pop() {
        result.push(node.borrow().val);

        // Push right first so left is processed first (LIFO)
        if let Some(right) = node.borrow().right.clone() {
            stack.push(right);
        }
        if let Some(left) = node.borrow().left.clone() {
            stack.push(left);
        }
    }
    result
}

// ============ POSTORDER (Left → Right → Node) ============

fn postorder_recursive(root: &TreeLink) -> Vec<i32> {
    let mut result = vec![];
    fn traverse(node: &TreeLink, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            traverse(&n.left, result);
            traverse(&n.right, result);
            result.push(n.val);
        }
    }
    traverse(root, &mut result);
    result
}

// Iterative: "Reverse trick" — do modified preorder (Node → Right → Left), then reverse
// This is elegant because it builds on preorder intuition
fn postorder_iterative(root: TreeLink) -> Vec<i32> {
    let mut result = vec![];
    let mut stack = vec![];

    if let Some(node) = root {
        stack.push(node);
    }

    while let Some(node) = stack.pop() {
        result.push(node.borrow().val);

        // Push left first, then right (opposite of preorder)
        // This gives us: Node → Right → Left
        if let Some(left) = node.borrow().left.clone() {
            stack.push(left);
        }
        if let Some(right) = node.borrow().right.clone() {
            stack.push(right);
        }
    }

    result.reverse(); // Node → Right → Left reversed = Left → Right → Node
    result
}

fn main() {
    //       4
    //      / \
    //     2   6
    //    / \ / \
    //   1  3 5  7
    let root = TreeNode::with_children(
        4,
        TreeNode::with_children(2, TreeNode::new(1), TreeNode::new(3)),
        TreeNode::with_children(6, TreeNode::new(5), TreeNode::new(7)),
    );

    // Inorder: Left → Node → Right
    let inorder_expected = vec![1, 2, 3, 4, 5, 6, 7];
    assert_eq!(inorder_recursive(&root), inorder_expected);
    assert_eq!(inorder_iterative(root.clone()), inorder_expected);
    println!("Inorder:  {:?} ✓", inorder_expected);

    // Preorder: Node → Left → Right
    let preorder_expected = vec![4, 2, 1, 3, 6, 5, 7];
    assert_eq!(preorder_recursive(&root), preorder_expected);
    assert_eq!(preorder_iterative(root.clone()), preorder_expected);
    println!("Preorder:  {:?} ✓", preorder_expected);

    // Postorder: Left → Right → Node
    let postorder_expected = vec![1, 3, 2, 5, 7, 6, 4];
    assert_eq!(postorder_recursive(&root), postorder_expected);
    assert_eq!(postorder_iterative(root.clone()), postorder_expected);
    println!("Postorder: {:?} ✓", postorder_expected);
}
