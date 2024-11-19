// ---------------------- HANDSON 2 ----------------------
// Author: Aliprandi Francesco

// ------- MAX SEGMENT TREE -------
pub struct MaxSegmentTree {
    tree: Vec<u32>,                 // The segment tree stored as a vector
    ranges: Vec<(usize, usize)>,    // Store the range for each node
    lazy_updates: Vec<Option<u32>>, // Store the lazy updates
}

impl MaxSegmentTree {
    pub fn new(arr: &[u32]) -> Self {
        let n = arr.len();
        let tree = vec![0; 4 * n];
        let lazy_updates = vec![None; 4 * n];
        let ranges = vec![(0, 0); 4 * n];
        let mut max_segment_tree = MaxSegmentTree {
            tree,
            ranges,
            lazy_updates,
        };
        max_segment_tree.build(arr, 0, 0, n - 1);
        max_segment_tree
    }

    // Build the segment tree recursively, starting from the root node
    // splitting the range [start, end] in half at each step
    fn build(&mut self, arr: &[u32], node_idx: usize, start: usize, end: usize) {
        self.ranges[node_idx] = (start, end);
        if start == end {
            // Leaf nodes
            self.tree[node_idx] = arr[start];
        } else {
            // Internal nodes
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

    // Return 1 if k is in interval [start,end], 0 otherwise
    pub fn is_there(&self, start: usize, end: usize, k: u32) -> u32 {
        (self.is_there_recursive(0, start, end, k) >= 1) as u32
    }

    // Recursive is_there function, it scan recursively the tree
    // and return the number of occurrences of k value in the range [start, end]
    pub fn is_there_recursive(&self, current: usize, start: usize, end: usize, k: u32) -> u32 {
        let (node_start, node_end) = self.ranges[current];
        if node_start >= start && node_end <= end {
            // Total overlap
            if self.tree[current] < k {
                return 0;
            }
            return self.check_total_overlap(current, node_start, node_end, k);
        } else if end < node_start || node_end < start {
            // No overlap
            return 0;
        }
        // Partial overlap
        let mid = (node_end + node_start) / 2;
        // Recursively query the left and right children
        let left_count = self.is_there_recursive(
            self.get_left_child(current),
            start,
            std::cmp::min(mid, end),
            k,
        );
        let right_count = self.is_there_recursive(
            self.get_right_child(current),
            std::cmp::max(mid + 1, start),
            end,
            k,
        );
        // Combine the results
        left_count + right_count
    }

    // if total overlap in is_there function it checks if the searched value k is present in
    // this interval.
    pub fn check_total_overlap(&self, current: usize, start: usize, end: usize, k: u32) -> u32 {
        if start == end {
            // leaf node
            return (self.tree[current] == k) as u32;
        }
        if self.tree[current] < k {
            // max in subtree < k, k cannot be in that subtree
            return 0;
        }
        if self.tree[current] == k {
            // if current value = k return 1
            return 1;
        }
        // recursive iteration, return sum from left and right child
        let mid = (start + end) / 2;
        let left_child = self.get_left_child(current);
        let right_child = self.get_right_child(current);

        let left_count = self.check_total_overlap(left_child, start, mid, k);
        let right_count = self.check_total_overlap(right_child, mid + 1, end, k);
        left_count + right_count
    }

    // Range Update Function: this function updates the range [start, end] with
    // the minimum between the value passed and the current value stored
    pub fn range_update(&mut self, start: usize, end: usize, value: u32) {
        self.range_update_recursive(0, start - 1, end - 1, value);
    }

    // Recursive Range Update Function: this function scans recursively the
    // tree and updates value in a lazy fashion
    pub fn range_update_recursive(
        &mut self,
        current: usize,
        start: usize,
        end: usize,
        mut value: u32,
    ) {
        let (node_start, node_end) = self.ranges[current];
        if node_start >= start && node_end <= end {
            // Total Overlap
            value = self.handle_pending_update(current, value, node_start, node_end);
            self.tree[current] = self.tree[current].min(value);
            self.propagate_lazy_update(current, value, node_start, node_end);
            return;
        } else if end < node_start || node_end < start {
            // No Overlap
            self.handle_pending_update(current, u32::MAX, node_start, node_end);
            return;
        }
        // Partial Overlap Recursion
        let left_child = self.get_left_child(current);
        let right_child = self.get_right_child(current);

        value = self.handle_pending_update(current, value, node_start, node_end);

        let mid = (node_start + node_end) / 2;
        // Recursively update the left and right children
        self.range_update_recursive(left_child, start, mid.min(end), value);
        self.range_update_recursive(right_child, (mid + 1).max(start), end, value);

        self.tree[current] = self.tree[left_child].max(self.tree[right_child]);
    }

    // Range Max Query Function: Lazy Update Implementation
    pub fn range_max_query_lazy(&mut self, start: usize, end: usize) -> u32 {
        self.range_max_query_lazy_recursive(0, start - 1, end - 1)
    }

    // Recursive Max Query Function: Lazy Update Implementation
    pub fn range_max_query_lazy_recursive(
        &mut self,
        current: usize,
        start: usize,
        end: usize,
    ) -> u32 {
        let (node_start, node_end) = self.ranges[current];

        // Handle pending updates on the node
        self.handle_pending_update(current, u32::MAX, node_start, node_end);

        if node_start >= start && node_end <= end {
            // Total Overlap
            return self.tree[current];
        } else if end < node_start || node_end < start {
            // No Overlap
            return 0;
        }
        // Partial Overlap Recursion
        let left_child = self.get_left_child(current);
        let right_child = self.get_right_child(current);
        let mid = (node_start + node_end) / 2;
        let left_result = self.range_max_query_lazy_recursive(left_child, start, mid.min(end));
        let right_result =
            self.range_max_query_lazy_recursive(right_child, (mid + 1).max(start), end);

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

    // Support Function: Propagate lazy updates on child if the node is not a leaf
    fn propagate_lazy_update(
        &mut self,
        current: usize,
        value: u32,
        node_start: usize,
        node_end: usize,
    ) {
        if node_start < node_end {
            let left_child = self.get_left_child(current);
            let right_child = self.get_right_child(current);
            // propagate the minimum between value to propagate and the current lazy value
            // Update lazy value for left child
            self.propagate_one_child(left_child, value);
            self.propagate_one_child(right_child, value);
        }
    }

    // propagate the update on one node
    fn propagate_one_child( &mut self, node: usize, value: u32) {
        self.lazy_updates[node] = match self.lazy_updates[node] {
            Some(left_lazy) => Some(left_lazy.min(value)),
            None => Some(value),
        };
    }

    // Support function to traverse and print the tree
    pub fn print_tree(&self, current: usize) {
        let (node_start, node_end) = self.ranges[current];
        print!(
            "Range: ({},{}): {}, ",
            node_start, node_end, self.tree[current]
        );
        if let Some(update) = self.lazy_updates[current] {
            print!("{} -", update);
        }
        print!("None -");
        if node_start == node_end {
            return;
        }
        self.print_tree(self.get_left_child(current));
        self.print_tree(self.get_right_child(current));
    }
}

// ----------- TEST SECTION ------------

// Test data structure to support test execution
pub struct TestCase {
    data: Vec<u32>,
    queries: Vec<Query>,
    exp_results: Vec<u32>,
}

pub type Query = (usize, usize, Option<u32>);

impl TestCase {
    pub fn new(data: Vec<u32>, queries: Vec<Query>, exp_results: Vec<u32>) -> Self {
        TestCase {
            data,
            queries,
            exp_results,
        }
    }

    pub fn data(&self) -> &Vec<u32> {
        &self.data
    }

    pub fn queries(&self) -> &Vec<Query> {
        &self.queries
    }

    pub fn results(&self) -> &Vec<u32> {
        &self.exp_results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    // ----- test for exercise 1 -----

    // This function load the input array, the queries and the expected output for one file
    // into the data/exercise1 repo
    fn load_test_case_ex1(input_file: File, output_file: File) -> TestCase {
        let input_reader = BufReader::new(input_file);
        let output_reader = BufReader::new(output_file);

        let mut input_lines = input_reader.lines().map(|line| line.unwrap());
        let mut output_lines = output_reader.lines().map(|line| line.unwrap());

        // Extract metadata (array size and number of queries)
        let header = input_lines.next().unwrap();
        let mut header_split = header.split_whitespace();
        let _ = header_split.next().unwrap().parse::<usize>().unwrap();
        let m = header_split.next().unwrap().parse::<usize>().unwrap();

        // Extract initial array data
        let array_data = input_lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|value| value.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut queries = Vec::new();
        let mut results = Vec::new();

        // Process queries
        for _ in 0..m {
            let query_line = input_lines.next().unwrap();
            let mut query_split = query_line.split_whitespace();

            let query_type = query_split.next().unwrap().parse::<usize>().unwrap();
            let start_index = query_split.next().unwrap().parse::<usize>().unwrap();
            let end_index = query_split.next().unwrap().parse::<usize>().unwrap();

            if query_type == 0 {
                // Range update query
                let value = query_split.next().unwrap().parse::<u32>().unwrap();
                queries.push((start_index, end_index, Some(value)));
            } else {
                // Max query
                queries.push((start_index, end_index, None));
                let result = output_lines.next().unwrap().parse::<u32>().unwrap();
                results.push(result);
            }
        }

        TestCase::new(array_data, queries, results)
    }

    // stores data from one input file and check results for that file
    fn execute_test_case1(test_dir: &str, case_index: usize) {
        let input_path = format!("{}/input{}.txt", test_dir, case_index);
        let output_path = format!("{}/output{}.txt", test_dir, case_index);

        let input_file = File::open(input_path).expect("Failed to open input file");
        let output_file = File::open(output_path).expect("Failed to open output file");

        let test_case = load_test_case_ex1(input_file, output_file);

        let arr = test_case.data();
        let queries = test_case.queries();
        let expected_results = test_case.results();

        let mut max_segment_tree = MaxSegmentTree::new(arr);
        let mut actual_results = Vec::new();

        for &(start, end, update_value) in queries {
            if let Some(value) = update_value {
                max_segment_tree.range_update(start, end, value);
            } else {
                let max_value = max_segment_tree.range_max_query_lazy(start, end);
                actual_results.push(max_value);
            }
        }

        assert_eq!(
            actual_results, *expected_results,
            "Test case {} failed: expected {:?}, got {:?}",
            case_index, expected_results, actual_results
        );
    }

    #[test]
    // Run tests on exercise 1
    fn validate_exercise1() {
        let test_path = Path::new("data").join("exercise1");
        let path_as_str = test_path.to_str().expect("Invalid path format");

        for index in 0..=10 {
            execute_test_case1(path_as_str, index);
        }
    }

    // ----- test for exercise 2 -----

    // load intervals, queries and expected results from one of the exercise 2 file
    fn load_test_case_ex2(input_file: File, output_file: File) -> TestCase {
        let input_reader = BufReader::new(input_file);
        let output_reader = BufReader::new(output_file);

        let mut input_lines = input_reader.lines().map(|line| line.unwrap());
        let mut output_lines = output_reader.lines().map(|line| line.unwrap());

        // Extract number of intervals and number of queries
        let header = input_lines.next().unwrap();
        let mut header_split = header.split_whitespace();
        let n = header_split.next().unwrap().parse::<usize>().unwrap();
        let m = header_split.next().unwrap().parse::<usize>().unwrap();

        let mut intervals = Vec::new();

        // extract intervals and push it into intervals array
        for _ in 0..n {
            let interval_line = input_lines.next().unwrap();
            let mut interval = interval_line.split_whitespace();
            let start = interval.next().unwrap().parse::<u32>().unwrap();
            let end = interval.next().unwrap().parse::<u32>().unwrap();
            intervals.push(start);
            intervals.push(end);
        }

        let mut queries = Vec::new();
        let mut results = Vec::new();

        // store queries and results
        for _ in 0..m {
            let query_line = input_lines.next().unwrap();
            let mut query_split = query_line.split_whitespace();

            let start = query_split.next().unwrap().parse::<usize>().unwrap();
            let end = query_split.next().unwrap().parse::<usize>().unwrap();
            let k = query_split.next().unwrap().parse::<u32>().unwrap();

            queries.push((start, end, Some(k)));

            let result = output_lines.next().unwrap().parse::<u32>().unwrap();
            results.push(result);
        }

        TestCase::new(intervals, queries, results)
    }

    // Load data from a file in data/exercise2, execute queries and check correctness
    fn execute_test_case2(test_dir: &str, case_index: usize) {
        let input_path = format!("{}/input{}.txt", test_dir, case_index);
        let output_path = format!("{}/output{}.txt", test_dir, case_index);

        let input_file = File::open(input_path).expect("Failed to open input file");
        let output_file = File::open(output_path).expect("Failed to open output file");

        let test_case = load_test_case_ex2(input_file, output_file);

        let intervals = test_case.data();
        let queries = test_case.queries();
        let expected_results = test_case.results();

        // Create a vector that stores #overlaps between intervals for each position using prefix
        // sum and sweep line strategy
        let n = intervals.len() / 2;
        let mut count: Vec<i32> = vec![0; n + 1];

        for i in 0..intervals.len() {
            let idx = intervals[i] as usize;
            if i % 2 == 0 {
                count[idx] += 1;
            } else {
                count[idx + 1] -= 1;
            }
        }

        // Prefix sums array
        let mut prefix_sum: Vec<i32> = vec![0; n + 1];
        prefix_sum[0] = count[0];
        for i in 1..n + 1 {
            prefix_sum[i] = prefix_sum[i - 1] + count[i];
        }

        // remove last element as unuseful
        prefix_sum.pop();

        // cast prefix sum array to u32
        let prefix_sum_u32: Vec<u32> = prefix_sum.iter().map(|&x| x as u32).collect();

        let max_segment_tree = MaxSegmentTree::new(&prefix_sum_u32);

        let mut actual_results = Vec::new();

        // run tests and store results
        for &(start, end, update_value) in queries {
            if let Some(k) = update_value {
                actual_results.push(max_segment_tree.is_there(start, end, k));
            }
        }

        //check results
        assert_eq!(
            actual_results, *expected_results,
            "Test case {} failed: expected {:?}, got {:?}",
            case_index, expected_results, actual_results
        );
    }

    #[test]

    // Run tests on exercise 2
    fn validate_exercise2() {
        let test_path = Path::new("data").join("exercise2");
        let path_as_str = test_path.to_str().expect("Invalid path format");

        for i in 0..=7 {
            execute_test_case2(path_as_str, i);
        }
    }
}
