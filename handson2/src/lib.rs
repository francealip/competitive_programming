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

        if node_start >= start && node_end <= end {
            // Total Overlap
            // Handle pending updates on the node
            value = self.handle_pending_update(current, value, node_start, node_end);

            self.tree[current] = self.tree[current].min(value);
            self.propagate_lazy_update(current, value, node_start, node_end);
            return self.tree[current];
        } else if end < node_start || node_end < start {
            // No Overlap
            // Handle pending updates on the node
            self.handle_pending_update(current, std::u32::MAX, node_start, node_end);
            return self.tree[current];
        }

        // Partial Overlap Recursion
        // Handle pending updates on the node
        value = self.handle_pending_update(current, value, node_start, node_end); //sbagliato l'invio dei pending update con no overlap

        let mid = (node_start + node_end) / 2;
        let left_result = self.range_update_recursive(left_child, start, mid.min(end), value);
        let right_result =
            self.range_update_recursive(right_child, (mid + 1).max(start), end, value);

        self.tree[current] = left_result.max(right_result);
        self.tree[current]
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
        if node_start < node_end {
            let left_child = self.get_left_child(current);
            let right_child = self.get_right_child(current);
            match (
                self.lazy_updates[left_child],
                self.lazy_updates[right_child],
            ) {
                (Some(left_value), Some(right_value)) => {
                    self.lazy_updates[left_child] = Some(left_value.min(value));
                    self.lazy_updates[right_child] = Some(right_value.min(value));
                }
                (Some(left_value), None) => {
                    self.lazy_updates[left_child] = Some(left_value.min(value));
                    self.lazy_updates[right_child] = Some(value);
                }
                (None, Some(right_value)) => {
                    self.lazy_updates[left_child] = Some(value);
                    self.lazy_updates[right_child] = Some(right_value.min(value));
                }
                (None, None) => {
                    self.lazy_updates[left_child] = Some(value);
                    self.lazy_updates[right_child] = Some(value);
                }
            }
        }
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

        // Handle pending updates on the node
        self.handle_pending_update(current, std::u32::MAX, node_start, node_end);

        if node_start >= start && node_end <= end {
            // Total Overlap
            return self.tree[current];
        } else if end < node_start || node_end < start {
            // No Overlap
            return 0;
        }

        // Partial Overlap Recursion
        let mid = (node_start + node_end) / 2;
        let left_result = self.range_max_query_lazy_recursive(left_child, start, mid.min(end));
        let right_result =
            self.range_max_query_lazy_recursive(right_child, (mid + 1).max(start), end);

        left_result.max(right_result)
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

// TESTING

use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Output;

    pub struct Test {
        data: Vec<u32>,
        queries: Vec<(usize, usize, Option<u32>)>,
        outputs: Vec<u32>,
    }

    impl Test {
        pub fn get_data(&self) -> &Vec<u32> {
            &self.data
        }

        pub fn get_queries(&self) -> &Vec<(usize, usize, Option<u32>)> {
            &self.queries
        }

        pub fn get_outputs(&self) -> &Vec<u32> {
            &self.outputs
        }
    }
    pub fn get_one_file_test(in_file: File, out_file: File) -> Test {
        let mut file_iter_input = BufReader::new(in_file).lines().map(|x| x.unwrap());

        let mut file_iter_output = BufReader::new(out_file).lines().map(|x| x.unwrap());

        // Read the first line for n and m
        let mut line = file_iter_input.next().unwrap();
        let mut iter = line.split_whitespace();
        let (n, m) = (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        );

        // Read the second line for the array
        line = file_iter_input.next().unwrap();
        iter = line.split_whitespace();
        let data = iter
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut queries = Vec::new();
        let mut outputs = Vec::new();

        for _ in 0..m {
            line = file_iter_input.next().unwrap();
            iter = line.split_whitespace();

            // Range Update query
            if iter.next().unwrap().parse::<usize>().unwrap() == 0 {
                let start = iter.next().unwrap().parse::<usize>().unwrap();
                let end = iter.next().unwrap().parse::<usize>().unwrap();
                let value = iter.next().unwrap().parse::<u32>().unwrap();
                queries.push((start, end, Some(value)));
            // Max query
            } else {
                let start = iter.next().unwrap().parse::<usize>().unwrap();
                let end = iter.next().unwrap().parse::<usize>().unwrap();
                queries.push((start, end, None));

                let output = file_iter_output.next().unwrap().parse::<u32>().unwrap();
                outputs.push(output);
            }
        }
        Test {
            data,
            queries,
            outputs,
        }
    }

    //support function that tests one file
    pub fn test_one_file(path: &str, file_number: usize) {
        println!("Reading file: {}/input{}.txt", path, file_number);
        let input_file_path = format!("{}/input{}.txt", path, file_number);
        let output_file_path = format!("{}/output{}.txt", path, file_number);
        let in_file = File::open(input_file_path).unwrap();
        let out_file = File::open(output_file_path).unwrap();

        let test = get_one_file_test(in_file, out_file);
        let arr = test.get_data();
        let queries = test.get_queries();
        let outputs = test.get_outputs();

        let mut segment_tree = SegmentTree::new(arr);
        let mut results: Vec<u32> = Vec::new();

        for query in queries {
            let start = query.0 - 1;
            let end = query.1 - 1;
            if let Some(value) = query.2 {
                segment_tree.range_update(start, end, value);
            } else {
                results.push(segment_tree.range_max_query_lazy(start, end));
            }
        }
        assert_eq!(results, *outputs);
    }

    #[test]
    pub fn test_exercise1() {
        let mut path = Path::new("data").join("exercise1");
        let path_str = path.to_str().unwrap();
        for i in 0..=10 {
            test_one_file(path_str, i);
        }
    }
}
