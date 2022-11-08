use std::{
    cmp::{max, Ordering},
    io,
};

struct FunElement {
    idx: usize,          // array idx
    next_element: usize, // idx + 1
    fun_val: i32,
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
    println!("{}", input_str);
    input_str
}

fn execute(fun_val: Vec<i32>, next_ptr: Vec<usize>) -> i32 {
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

    let mut fun_sum = 0;
    for vec_id in 0..new_order.len() {
        let curr_id = new_order[vec_id];
        if vec_id == (new_order.len() - 1) {
            fun_sum += fun_val_copy[curr_id];
            break;
        }
        let next_id = new_order[vec_id + 1];
        if next_ptr[curr_id] == next_ptr[next_id] {
            // drop or swap
            if fun_val_copy[curr_id] >= fun_val_copy[next_id] {
                // drop
                fun_sum += fun_val_copy[curr_id];
                continue;
            } else {
                // swap
                fun_sum += fun_val_copy[next_id];
                new_order[next_id] = new_order[curr_id];
                continue;
            }
        } else {
            if next_ptr[curr_id] == 0 {
                // drop source
                fun_sum += fun_val[curr_id];
                continue;
            } else {
                // merge
                let merge_candidate: usize = next_ptr[curr_id] - 1;
                fun_val_copy[merge_candidate] =
                    max(fun_val_copy[merge_candidate], fun_val[curr_id]);
                continue;
            }
        }
    }
    fun_sum
}

fn main() {
    /*
    let number_of_testcases = read()
        .parse::<i32>()
        .expect("Number of test cases must be an integer!");

    for _ in 0..number_of_testcases {
        let _ = read()
            .parse::<i32>()
            .expect("Must be a string with integer numbers!");
        let fun_values: Vec<i32> = read()
            .split_whitespace()
            .map(|s| s.parse::<i32>().ok().expect("parse error"))
            .collect();
        let _ = read()
            .parse::<i32>()
            .expect("Must be a string with integer numbers!");
        let pointer_values: Vec<i32> = read()
            .split_whitespace()
            .map(|s| s.parse::<i32>().ok().expect("parse error"))
            .collect();
        let mut funElements: Vec<FunElement> = fun_values
            .iter()
            .zip(pointer_values.iter())
            .map(|(a, b)| FunElement {
                next_element: *a,
                fun_val: *b,
            })
            .collect();
        funElements.sort();
        for el in funElements {
            println!("{} {}", el.next_element, el.fun_val);
        }
    }
    */
    let result_one = execute(vec![60, 20, 40, 50], vec![0, 1, 1, 2]);
    assert!(result_one == 110);
    println!("Fun sum is {}", result_one);
    let result_two = execute(vec![3, 2, 1, 4, 5], vec![0, 1, 1, 1, 0]);
    println!("Fun sum is {}", result_two);
    assert!(result_two == 14);
    let result_three = execute(
        vec![100, 100, 100, 90, 80, 100, 90, 100],
        vec![0, 1, 2, 1, 2, 3, 1, 3],
    );
    println!("Fun sum is {}", result_three);
    assert!(result_three == 490);
}
