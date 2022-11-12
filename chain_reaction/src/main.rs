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
                fun_val_copy[merge_candidate] =
                    max(fun_val_copy[merge_candidate], fun_val_copy[curr_id]);
                continue;
            }
        }
    }
    fun_sum
}

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

fn test_runs() {
    let mut fun_values: Vec<i64> = vec![50];
    let mut pointer_values: Vec<usize> = vec![0];

    assert!(execute(fun_values, pointer_values) == 50);
    fun_values = vec![50, 40];
    pointer_values = vec![0, 1];
    assert!(execute(fun_values, pointer_values) == 50);

    fun_values = vec![50, 40];
    pointer_values = vec![0, 0];
    assert!(execute(fun_values, pointer_values) == 90);

    fun_values = vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10];
    pointer_values = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert!(execute(fun_values, pointer_values) == 550);

    fun_values = vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10];
    pointer_values = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert!(execute(fun_values, pointer_values) == 100);

    fun_values = vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10];
    pointer_values = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    assert!(execute(fun_values, pointer_values) == 100);

    fun_values = vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10];
    pointer_values = vec![0, 1, 2, 3, 4, 0, 6, 7, 8, 9];
    assert!(execute(fun_values, pointer_values) == 150);

    fun_values = vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10];
    pointer_values = vec![0, 1, 0, 3, 0, 5, 0, 7, 0, 9];
    assert!(execute(fun_values, pointer_values) == 300);

    let max_val: i64 = 10i64.pow(9);
    fun_values = vec![];
    pointer_values = vec![];
    for _ in 0..1000 {
        fun_values.push(max_val);
        pointer_values.push(0);
    }
    assert!(execute(fun_values, pointer_values) == i128::from(max_val) * 1000);
}

/* Input
3
4
60 20 40 50
0 1 1 2
5
3 2 1 4 5
0 1 1 1 0
8
100 100 100 90 80 100 90 100
0 1 2 1 2 3 1 3
*/
fn main() {
    /* Test data structure */
    test_equality();
    //test_runs();
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
