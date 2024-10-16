use std::cmp::{max, min};
use std::u32;

struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left  child of the
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert_eq!(
                self.nodes[parent_id].id_left, None,
                "Parent node has the left child already set"
            );
        } else {
            assert_eq!(
                self.nodes[parent_id].id_right, None,
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    /* ---------- Exercise  #1 ---------- */
    /* Write a method to check if the binary tree is a Binary Search Tree. */

    ///return True if the tree is a BST
    pub fn is_bst(&self) -> bool {
        self.rec_is_bst(Some(0)).0
    }

    /// A private recursive function that check if a
    /// subtree rooted at `node_id` is a BST
    fn rec_is_bst(&self, node_id: Option<usize>) -> (bool, u32, u32) {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node: &Node = &self.nodes[id];
            let (ans_l, max_l, min_l) = self.rec_is_bst(node.id_left);
            let (ans_r, max_r, min_r) = self.rec_is_bst(node.id_right);
            let ans_node: bool = ans_l && ans_r && node.key >= max_l && node.key < min_r;
            let max_node = max(node.key, max(max_l, max_r));
            let min_node = min(node.key, min(min_l, min_r));

            return (ans_node, max_node, min_node);
        }

        (true, 0, u32::MAX)
    }

    /* ---------- Exercise  #2 ---------- */
    /* Write a method to solve the Maximum Path Sum problem. The method must return
    the sum of the maximum simple path connecting two leaves. */

    /// return the maximum path sum
    pub fn max_path_sum_backup(&self) -> u32 {
        self.rec_max_path_sum_backup(Some(0)).0
    }

    /// A private recursive function that return the maximum path sum and
    /// the maximum leaf-node path cost for a subtree rooted at `node_id`
    fn rec_max_path_sum_backup(&self, node_id: Option<usize>) -> (u32, u32) {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node: &Node = &self.nodes[id];
            let (best_l, max_l) = self.rec_max_path_sum_backup(node.id_left);
            let (best_r, max_r) = self.rec_max_path_sum_backup(node.id_right);
            let path: u32 = node.key + max_l + max_r;
            let best: u32 = max(path, max(best_l, best_r));
            let max: u32 = max(max_l, max_r) + node.key;

            return (best, max);
        }
        (0, 0)
    }

    pub fn max_path_sum(&self) -> Option<u32> {
        self.rec_max_path_sum(Some(0)).0
    }

    fn rec_max_path_sum(&self, node_id: Option<usize>) -> (Option<u32>, Option<u32>) {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node: &Node = &self.nodes[id];
            let (best_l, max_l) = self.rec_max_path_sum(node.id_left);
            let (best_r, max_r) = self.rec_max_path_sum(node.id_right);

            print!("I'm node: {}, receiving: ", node.key);
            println!("{}, {}, {}, {}", best_l.unwrap_or(0), max_l.unwrap_or(0),
                   best_r.unwrap_or(0), max_r.unwrap_or(0));

            match (best_l, max_l, best_r, max_r) {
                // if everything is defined
                (Some(bl), Some(ml), Some(br), Some(mr)) => {
                    let path: u32 = node.key + ml + mr;
                    let best: u32 = max(path, max(bl, br));
                    let max: u32 = max(ml, mr) + node.key;
                    return (Some(best), Some(max));
                },
                // if i don't found a best path on left subtree
                (None, Some(ml), Some(br), Some(mr)) => {
                    let path: u32 = node.key + ml + mr;
                    let best: u32 = max(path, br);
                    let max: u32 = max(ml, mr) + node.key;
                    return (Some(best), Some(max));
                },
                // if i don't found a best path on right subtree
                (Some(bl), Some(ml), None, Some(mr)) => {
                    let path: u32 = node.key + ml + mr;
                    let best: u32 = max(path, bl);
                    let max: u32 = max(ml, mr) + node.key;
                    return (Some(best), Some(max));
                },
                // if i don't found a best so far
                (None, Some(ml), None, Some(mr)) => {
                    let best: u32 = node.key + ml + mr;
                    let max: u32 = max(ml, mr) + node.key;
                    return (Some(best), Some(max));
                },
                // if i don't have both best so far and max from right child
                (Some(bl), Some(ml), None, None) => {
                    return (Some(bl),Some(ml + node.key));
                }
                // if i don't have both best so far and max from left child
                (None, None, Some(br), Some(mr)) => {
                    return (Some(br),Some(mr + node.key));
                }
                // if i only have max from left
                (None, Some(ml), None, None) => {
                    return (None,Some(ml + node.key));
                }
                // if i only have max from right
                (None, None, None, Some(mr)) => {
                    return (None,Some(mr + node.key));
                }
                // if i'm a leaf
                (None, None, None, None) => {
                    return (None, Some(node.key));
                }
                _ => unreachable!("This code should never be reached"),
            }
        }
        (None, None)
    }
}


/* ---------- Unit Tests ---------- */

#[cfg(test)]
mod tests {
    use super::*;

    /// test for exercise 1
    #[test]
    fn test_is_bst() {
        let mut bst = Tree::with_root(20); // id 0
        assert!(bst.is_bst(), "Tree with only root must be a BST");

        bst.add_node(0, 6, true); // id 1
        bst.add_node(0, 28, false); // id 2

        assert!(bst.is_bst(), "Tree is a BST");

        bst.add_node(1, 3, true); // id 3
        bst.add_node(1, 9, false); // id 4

        assert!(bst.is_bst(), "Tree is a BST");

        bst.add_node(2, 23, true); // id 5
        bst.add_node(2, 37, false); // id 6

        assert!(bst.is_bst(), "Tree is a BST");

        let mut not_bst1 = Tree::with_root(20); // id 0

        not_bst1.add_node(0, 6, true); // id 1
        not_bst1.add_node(0, 5, false); // id 2

        assert!(
            !not_bst1.is_bst(),
            "Tree with value 5 violate the BST property"
        );

        let mut not_bst2 = Tree::with_root(20); // id 0

        not_bst2.add_node(0, 6, true); // id 1
        not_bst2.add_node(0, 21, false); // id 2
        not_bst2.add_node(2, 19, true); // id 3

        assert!(
            !not_bst2.is_bst(),
            "Tree with value 19 violate the BST property"
        );

        let mut not_bst3 = Tree::with_root(20); // id 0

        not_bst3.add_node(0, 6, true); // id 1
        not_bst3.add_node(0, 21, false); // id 2
        not_bst3.add_node(1, 18, false); // id 3
        not_bst3.add_node(1, 19, true); // id 4

        assert!(
            !not_bst3.is_bst(),
            "Tree with value 19 violate the BST property"
        );
    }

    #[test]
    fn test_max_path_sum() {

        // No max path test for the max_path_sum method
        let mut tree = Tree::with_root(20); // id 0
        assert_eq!(
            tree.max_path_sum(),
            None,
            "Tree with only root must return None path"
        );

        tree.add_node(0, 6, true); // id 1
        assert_eq!(
            tree.max_path_sum(),
            None,
            "Tree with only root and one node must return None path"
        );

        tree.add_node(1, 5, true); // id 2
        assert_eq!(
            tree.max_path_sum(),
            None,
            "Tree with only one leaf must return None path"
        );

        tree.add_node(2, 3, false); // id 3
        assert_eq!(
            tree.max_path_sum(),
            None,
            "Tree with only one leaf must return None path"
        );

        // Standard test for the max_path_sum method

        let mut tree = Tree::with_root(20); // id 0
        tree.add_node(0, 6, true); // id 1
        tree.add_node(0, 5, false); // id 2
        assert_eq!(
            tree.max_path_sum().unwrap(),
            31,
            "This tree has max path sum of 31"
        );

        tree.add_node(2, 9, true); // id 3
        tree.add_node(2, 8, false); // id 4
        assert_eq!(
            tree.max_path_sum().unwrap(),
            40,
            "This tree has max path sum of 40"
        );

        tree.add_node(1, 0, true); // id 5
        tree.add_node(1, 2, false); // id 6

        assert_eq!(
            tree.max_path_sum().unwrap(),
            42,
            "This tree has max path sum of 42"
        );

        tree.add_node(3, 55, true); // id 5
        tree.add_node(3, 150, false); // id 6

        assert_eq!(
            tree.max_path_sum().unwrap(),
            214,
            "This tree has max path sum of 214"
        );

    }
}
