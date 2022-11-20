/// MIT License
///
/// Copyright (c) 2022 David Kudlek
///
/// Permission is hereby granted, free of charge, to any person obtaining a copy
/// of this software and associated documentation files (the "Software"), to deal
/// in the Software without restriction, including without limitation the rights
/// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
/// copies of the Software, and to permit persons to whom the Software is
/// furnished to do so, subject to the following conditions:
///
/// The above copyright notice and this permission notice shall be included in all
/// copies or substantial portions of the Software.
///
/// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
/// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
/// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
/// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
/// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
/// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
/// SOFTWARE.
///
use std::{
    cmp::{max, Ordering},
    io,
};

struct FunElement {
    idx: usize,          // array idx
    next_element: usize, // idx + 1
    fun_val: i64,
}

impl Ord for FunElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.next_element == other.next_element {
            if self.fun_val == other.fun_val {
                return std::cmp::Ordering::Equal;
            } else if self.fun_val < other.fun_val {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        } else if self.next_element < other.next_element {
            return std::cmp::Ordering::Less;
        } else {
            return std::cmp::Ordering::Greater;
        }
    }
}

impl PartialOrd for FunElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FunElement {
    fn eq(&self, other: &Self) -> bool {
        self.next_element == other.next_element && self.fun_val == other.fun_val
    }
}

impl Eq for FunElement {}

fn read() -> String {
    let mut input_str = String::new();
    let stdin = io::stdin();
    stdin
        .read_line(&mut input_str)
        .ok()
        .expect("Failed to read from input!");
    input_str.trim().to_string()
}

fn execute(fun_val: Vec<i64>, next_ptr: Vec<usize>) -> i128 {
    if fun_val.len() != next_ptr.len() {
        panic!("Lists must have the same length!");
    }
    let mut fun_val_copy = fun_val.clone();
    let this_ptr: Vec<usize> = (0usize..fun_val_copy.len()).collect();
    let mut fun_elements: Vec<FunElement> = fun_val_copy
        .iter()
        .zip(next_ptr.iter())
        .zip(this_ptr.iter())
        .map(|((a, b), c)| FunElement {
            idx: *c,
            next_element: *b,
            fun_val: *a,
        })
        .collect();
    fun_elements.sort();
    let mut new_order: Vec<usize> = fun_elements.iter().map(|el| el.idx).collect();
    new_order.reverse();

    let mut fun_sum: i128 = 0;
    for vec_id in 0..new_order.len() {
        let curr_id = new_order[vec_id];
        if vec_id == (new_order.len() - 1) {
            fun_sum += i128::from(fun_val_copy[curr_id]);
            break;
        }
        let next_id = new_order[vec_id + 1];
        if next_ptr[curr_id] == 0 {
            fun_sum += i128::from(fun_val_copy[curr_id]);
            continue;
        } else if next_ptr[curr_id] == next_ptr[next_id] {
            // drop or swap
            if fun_val_copy[curr_id] >= fun_val_copy[next_id] {
                // drop
                fun_sum += i128::from(fun_val_copy[curr_id]);
                continue;
            } else {
                // swap
                fun_sum += i128::from(fun_val_copy[next_id]);
                fun_val_copy[next_id] = fun_val_copy[curr_id];
                continue;
            }
        } else {
            if next_ptr[curr_id] == 0 {
                // drop source
                fun_sum += i128::from(fun_val[curr_id]);
                continue;
            } else {
                // merge
                let merge_candidate: usize = next_ptr[curr_id] - 1;
                #[cfg(trace)]
                println!(
                    "{} {}",
                    fun_val_copy[merge_candidate], fun_val_copy[curr_id]
                );
                fun_val_copy[merge_candidate] =
                    max(fun_val_copy[merge_candidate], fun_val_copy[curr_id]);
                continue;
            }
        }
    }
    fun_sum
}

/// # Chain reaction google coding competition
/// https://codingcompetitions.withgoogle.com/codejam/round/0000000000876ff1/0000000000a45ef7
///
/// Given an element, list, tree or forest. Each element has an fun value and points to another element or into the void (0).
/// The value of one tree is the maximum of all nodes on a single path.
/// If more than one element points to the same node then you have to pick one whose path get executed. The other elements point into the void from there.
/// In other words. You start executing one path from the botton and then you see which path you can still use.
/// The total fun value is the sum of all tree values.
///
/// ```
/// fun_value = 60 20 40 50
/// next_pointer = 0 1 1 2
/// idx = 1 2 3 4
/// ```
///
/// 1 points into the void. 2 and 3 point to 1 and 4 points to 2. There are two ways to execute the tree. Start at 4 or start at 3.
/// If we start at 3 then we execute 3 and 1 => ``max(40,60) = 60``. 2 now points into the void and we execute 4 and 2 => ``max(50,20) = 50``. The sum is 110.
/// If we start at 4 then we execute 4,2 and 1 => ``max(50,20,60) = 60``. It remains 3 with value 40. The sum is 110.
/// Starting at 3 yields more fun in is therby better.
///
///
/// Solution with Total: ``O(N) + O(N log N) + O(N) = O(N log N)``
/// - Create temporary structure: Time is ``O(N)`` and Space is ``O(N)``
/// - Sort with ``next_idx[i-1]`` must be smaller then ``next_idx[1]`` and ``fun_value[i-1]`` must be smaller than ``fun_value[i]``: ``O(N log N)``
/// - Start from the highest index and merge, drop or swap: ``O(N)``
///   - If the current node is the only one that points to the next element then store the ``fun_value = max(fun_value[i], fun_value[next_ptr])``
///   - If more than one node points to the next element then:
///     - If the value is higher than the next value then drop the element and add the fun_value to the sum
///     - If the value is smaller than the next value then swap then drop the next element and copy the current element at it's place
///   - If the node points into the void then drop it and add the fun value to the sum
///
/// # Example input:
/// ```
/// 3
/// 4
/// 60 20 40 50
/// 0 1 1 2
/// 5
/// 3 2 1 4 5
/// 0 1 1 1 0
/// 8
/// 100 100 100 90 80 100 90 100
/// 0 1 2 1 2 3 1 3
/// ```
fn main() {
    /* Read from command line */
    let number_of_testcases = read()
        .parse::<i64>()
        .expect("Number of test cases must be an integer!");
    for idx in 0..number_of_testcases {
        let _ = read()
            .parse::<i64>()
            .expect("Must be a string with integer numbers!");
        let fun_values: Vec<i64> = read()
            .split_whitespace()
            .map(|s| s.parse::<i64>().ok().expect("parse error"))
            .collect();
        let pointer_values: Vec<usize> = read()
            .split_whitespace()
            .map(|s| s.parse::<usize>().ok().expect("parse error"))
            .collect();
        let result = execute(fun_values, pointer_values);
        println!("Case #{}: {}", idx + 1, result);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_equality() {
        let el1 = FunElement {
            idx: 1,
            next_element: 1,
            fun_val: 50,
        };
        let el2 = FunElement {
            idx: 1,
            next_element: 1,
            fun_val: 40,
        };
        let el3 = FunElement {
            idx: 1,
            next_element: 2,
            fun_val: 40,
        };
        let el4 = FunElement {
            idx: 1,
            next_element: 2,
            fun_val: 50,
        };

        assert!(el1 == el1);
        assert!(el2 < el1);
        assert!(el1 < el3);
        assert!(el4 > el2);
        assert!(el1 > el2);
    }

    #[test]
    fn test_single() {
        let fun_values: Vec<i64> = vec![50];
        let pointer_values: Vec<usize> = vec![0];
        assert_eq!(execute(fun_values, pointer_values), 50);
    }

    #[test]
    fn test_row2() {
        let fun_values = vec![50, 40];
        let pointer_values = vec![0, 1];
        assert_eq!(execute(fun_values, pointer_values), 50);
    }

    #[test]
    fn test_parallel2() {
        let fun_values = vec![50, 40];
        let pointer_values = vec![0, 0];
        assert_eq!(execute(fun_values, pointer_values), 90);
    }

    #[test]
    fn test_parallel10() {
        let fun_values = vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10];
        let pointer_values = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(execute(fun_values, pointer_values), 550);
    }

    #[test]
    fn test_row10() {
        let fun_values = vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10];
        let pointer_values = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(execute(fun_values, pointer_values), 100);
    }

    #[test]
    fn test_multi_subtree2() {
        let fun_values = vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10];
        let pointer_values = vec![0, 1, 2, 3, 4, 0, 6, 7, 8, 9];
        assert_eq!(execute(fun_values, pointer_values), 150);
    }

    #[test]
    fn test_multi_subtree5() {
        let fun_values = vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10];
        let pointer_values = vec![0, 1, 0, 3, 0, 5, 0, 7, 0, 9];
        assert_eq!(execute(fun_values, pointer_values), 300);
    }

    #[test]
    fn test_max_range() {
        let max_val: i64 = 10i64.pow(9);
        let mut fun_values = vec![];
        let mut pointer_values = vec![];
        for _ in 0..1000 {
            fun_values.push(max_val);
            pointer_values.push(0);
        }
        assert_eq!(
            execute(fun_values, pointer_values),
            i128::from(max_val) * 1000
        );
    }
}
