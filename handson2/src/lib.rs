use std::cmp::max;
struct SegmentTree {
    tree: Vec<u32>,                 // The segment tree stored as a vector
    ranges: Vec<(usize, usize)>,    // Store the range for each node
    lazy_updates: Vec<Option<u32>>, // Store the lazy updates
    size: usize,                    // Size of the original array
}

impl SegmentTree {
    pub fn new(arr: &[u32]) -> Self {
        let n = arr.len();
        let mut tree = vec![0; 4 * n];
        let mut lazy_updates = vec![None; 4 * n];
        let ranges = vec![(0, 0); 4 * n]; // Initialize ranges with dummy values
        let mut segment_tree = SegmentTree {
            tree,
            ranges,
            lazy_updates,
            size: n,
        };
        segment_tree.build(arr, 0, 0, n - 1);
        segment_tree
    }

    fn build(&mut self, arr: &[u32], node_idx: usize, start: usize, end: usize) {
        self.ranges[node_idx] = (start, end); // Store the range
        if start == end {
            self.tree[node_idx] = arr[start];
        } else {
            let mid = (start + end) / 2;

            // Build left and right subtrees
            self.build(arr, self.get_left_child(node_idx), start, mid);
            self.build(arr, self.get_right_child(node_idx), mid + 1, end);

            // Combine results for the current node
            self.tree[node_idx] = std::cmp::max(
                self.tree[self.get_left_child(node_idx)],
                self.tree[self.get_right_child(node_idx)],
            );
        }
    }

    // return left child of a given node index
    pub fn get_left_child(&self, node_idx: usize) -> usize {
        2 * node_idx + 1
    }

    // return right child of a given node index
    pub fn get_right_child(&self, node_idx: usize) -> usize {
        2 * node_idx + 2
    }

    // Get the range covered by a specific node
    pub fn get_range(&self, node_idx: usize) -> (usize, usize) {
        self.ranges[node_idx]
    }

    // Get the maximum value in the range [start, end]
    pub fn range_max_query(&self, start: usize, end: usize) -> u32 {
        self.range_max_query_recursive(0, start, end)
    }

    pub fn range_max_query_recursive(&self, current: usize, start: usize, end: usize) -> u32 {
        let (node_start, node_end) = self.ranges[current];
        if node_start >= start && node_end <= end {
            // Total overlap
            return self.tree[current];
        } else if end < node_start || node_end < start {
            // No overlap
            return 0;
        }
        // Partial overlap
        let mid = (node_end + node_start) / 2;
        let left_max = self.range_max_query_recursive(
            self.get_left_child(current),
            start,
            std::cmp::min(mid, end),
        );
        let right_max = self.range_max_query_recursive(
            self.get_right_child(current),
            std::cmp::max(mid + 1, start),
            end,
        );
        std::cmp::max(right_max, left_max)
    }

    // Range Update Function
    pub fn range_update(&mut self, start: usize, end: usize, value: u32) {
        self.range_update_recursive(0, start, end, value);
    }

    // Recursive Range Update Function
    pub fn range_update_recursive(
        &mut self,
        current: usize,
        start: usize,
        end: usize,
        mut value: u32,
    ) -> u32 {
        let (node_start, node_end) = self.ranges[current];
        let left_child = self.get_left_child(current);
        let right_child = self.get_right_child(current);

        // Handle panding updates on the node
        value = self.handle_pending_update(current, value, node_start, node_end);

        if node_start >= start && node_end <= end {
            // Total Overlap
            self.tree[current] = self.tree[current].min(value);
            self.propagate_lazy_update(current, value, node_start, node_end);
            return self.tree[current];
        }

        if end < node_start || node_end < start {
            // No Overlap
            return self.tree[current];
        }

        // Partial Overlap Recursion
        let mid = (node_start + node_end) / 2;
        let left_result = self.range_update_recursive(left_child, start, mid.min(end), value);
        let right_result = self.range_update_recursive(right_child, (mid + 1).max(start), end, value);

        self.tree[current] = left_result.max(right_result);
        self.tree[current]
    }

    // Range Update Function
    pub fn range_max_query_lazy(&mut self, start: usize, end: usize) -> u32 {
        self.range_max_query_lazy_recursive(0, start, end)
    }

    // Recursive Range Update Function
    pub fn range_max_query_lazy_recursive(
        &mut self,
        current: usize,
        start: usize,
        end: usize,
    ) -> u32 {
        let (node_start, node_end) = self.ranges[current];
        let left_child = self.get_left_child(current);
        let right_child = self.get_right_child(current);

        // Handle panding updates on the node
        self.handle_pending_update(current, std::u32::MAX, node_start, node_end);

        if node_start >= start && node_end <= end {
            // Total Overlap
            return self.tree[current];
        }

        if end < node_start || node_end < start {
            // No Overlap
            return 0;
        }

        // Partial Overlap Recursion
        let mid = (node_start + node_end) / 2;
        let left_result = self.range_max_query_lazy_recursive(left_child, start, mid.min(end));
        let right_result = self.range_max_query_lazy_recursive(right_child, (mid + 1).max(start), end);

        left_result.max(right_result)
    }

    // Support Function: Handle pending updates
    fn handle_pending_update(
        &mut self,
        current: usize,
        mut value: u32,
        node_start: usize,
        node_end: usize,
    ) -> u32 {
        if let Some(update) = self.lazy_updates[current].take() {
            value = value.min(update);
            self.tree[current] = self.tree[current].min(update);
            self.propagate_lazy_update(current, update, node_start, node_end);
        }
        value
    }

    // Support Function: Propagate lazy updates on childs
    fn propagate_lazy_update(
        &mut self,
        current: usize,
        value: u32,
        node_start: usize,
        node_end: usize,
    ) {
        if node_start != node_end {
            let left_child = self.get_left_child(current);
            let right_child = self.get_right_child(current);
            self.lazy_updates[left_child] = Some(value);
            self.lazy_updates[right_child] = Some(value);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_tree() {
        let arr = [5, 4, 3, 2, 9, 1, 7];
        let segment_tree = SegmentTree::new(&arr);

        // The segment tree should look like this:
        //             9
        //        5         9
        //     5    3     9   7
        //    5 4  3 2   9 1
        assert_eq!(
            segment_tree.tree,
            [9, 5, 9, 5, 3, 9, 7, 5, 4, 3, 2, 9, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );

        let arr = [9, 3, 11, 7, 23];
        let segment_tree = SegmentTree::new(&arr);

        // The segment tree should look like this:
        //             23
        //        11       23
        //      9   11   7   23
        //    9  3
        assert_eq!(
            segment_tree.tree,
            [23, 11, 23, 9, 11, 7, 23, 9, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );

        let arr = [9, 7, 8, 3, 12, 11, 7, 25];
        let segment_tree = SegmentTree::new(&arr);

        // The segment tree should look like this:
        //               25
        //         9             25
        //      9     8      12     25
        //    9  7  8  3   12  11  7  25
        assert_eq!(
            segment_tree.tree,
            [
                25, 9, 25, 9, 8, 12, 25, 9, 7, 8, 3, 12, 11, 7, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn test_max_query() {
        let arr = [5, 4, 3, 2, 9, 1, 7];
        let segment_tree = SegmentTree::new(&arr);

        assert_eq!(segment_tree.range_max_query(1, 4), 9);
        assert_eq!(segment_tree.range_max_query(2, 6), 9);
        assert_eq!(segment_tree.range_max_query(0, 6), 9);
        assert_eq!(segment_tree.range_max_query(0, 3), 5);
        assert_eq!(segment_tree.range_max_query(5, 6), 7);

        let arr = [9, 3, 11, 7, 23, 1, 5, 8, 7, 12, 11, 7, 25];
        let segment_tree = SegmentTree::new(&arr);

        assert_eq!(segment_tree.range_max_query(0, 3), 11);
        assert_eq!(segment_tree.range_max_query(1, 4), 23);
        assert_eq!(segment_tree.range_max_query(1, 3), 11);
        assert_eq!(segment_tree.range_max_query(2, 6), 23);
        assert_eq!(segment_tree.range_max_query(0, 6), 23);
        assert_eq!(segment_tree.range_max_query(0, 12), 25);
        assert_eq!(segment_tree.range_max_query(7, 10), 12);
        assert_eq!(segment_tree.range_max_query(7, 9), 12);
        assert_eq!(segment_tree.range_max_query(5, 6), 5);
    }

    #[test]
    fn test_range_update() {
        let arr = [19, 23, 17, 14, 9, 11, 7];
        let mut segment_tree = SegmentTree::new(&arr);
        assert_eq!(
            segment_tree.tree,
            [
                23, 23, 11, 23, 17, 11, 7, 19, 23, 17, 14, 9, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ]
        );
        segment_tree.range_update(0, 3, 22);
        assert_eq!(
            segment_tree.tree,
            [
                22, 22, 11, 23, 17, 11, 7, 19, 23, 17, 14, 9, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ]
        );
        assert_eq!(
            segment_tree.lazy_updates,
            [None, None, None, Some(22), Some(22), None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None]

        );
        segment_tree.range_update(2, 4, 15);
        assert_eq!(
            segment_tree.tree,
            [
                22, 22, 11, 22, 15, 11, 7, 19, 23, 17, 14, 9, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ]
        );
        assert_eq!(
            segment_tree.lazy_updates,
            [None, None, None, None, None, None, None, Some(22), Some(22), Some(15), Some(15),
                None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None]
        );
        segment_tree.range_update(6, 6, 5);
        assert_eq!(
            segment_tree.tree,
            [
                22, 22, 11, 22, 15, 11, 5, 19, 23, 17, 14, 9, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ]
        );
        assert_eq!(
            segment_tree.lazy_updates,
            [None, None, None, None, None, None, None, Some(22), Some(22), Some(15), Some(15),
                None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None]
        );
        segment_tree.range_update(1, 3, 12);
        assert_eq!(
            segment_tree.tree,
            [19, 19, 11, 19, 12, 11, 5, 19, 12, 17, 14, 9, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            segment_tree.lazy_updates,
            [None, None, None, None, None, None, None, None, None, Some(12), Some(12), None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None]
        );
    }

    #[test]
    fn test_lazy_max_query(){
        let arr = [19, 23, 17, 14, 9, 11, 7];
        let mut segment_tree = SegmentTree::new(&arr);
        segment_tree.range_update(0, 3, 22);
        segment_tree.range_update(2, 4, 15);
        segment_tree.range_update(6, 6, 5);
        assert_eq!(segment_tree.range_max_query_lazy(1, 4), 22);
        assert_eq!(
            segment_tree.lazy_updates,
            [None, None, None, None, None, None, None, None, None, Some(15), Some(15), None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None]
        );
        segment_tree.range_update(2, 3, 7);
        segment_tree.range_update(4, 6, 10);
        // Checks if trees are okay after the updates
        assert_eq!(
            segment_tree.tree,
            [22, 22, 10, 22, 7, 11, 5, 19, 22, 17, 14, 9, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            segment_tree.lazy_updates,
            [None, None, None, None, None, Some(10), Some(10), None, None, Some(7), Some(7), None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None]
        );
        assert_eq!(segment_tree.range_max_query_lazy(3, 5), 10);
        assert_eq!(
            segment_tree.tree,
            [22, 22, 10, 22, 7, 10, 5, 19, 22, 7, 7, 9, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            segment_tree.lazy_updates,
            [None, None, None, None, None, None, None, None,None, None, None,Some(10), Some(10),
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None]
        );
    }
}
