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
use clap::Parser;
use rand::Rng;
use std::error::Error;
use std::time::Duration;
use std::time::Instant;

pub enum Mode {
    Naive,              // O(N *N)
    DynamicProgramming, // O(N log(N) + N)
}

use std::fmt;

#[derive(Debug)]
struct GenericError {
    details: String,
}

impl GenericError {
    fn new(msg: &str) -> GenericError {
        GenericError {
            details: msg.to_string(),
        }
    }
}

impl Error for GenericError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

/// Naive approach with: ``O(N * N)``
///
/// Compare each intervale with all other intervals.
/// Early exit when we find one interval that doesn't overlap with an other interval from the
/// list.
fn naive_search(list_of_building_heights: &Vec<i32>) -> i64 {
    let mut min_demolished_levels: i64 = -1;
    for selected_height in list_of_building_heights {
        let mut demolished_levels: i64 = 0;
        for height in list_of_building_heights {
            if height >= selected_height {
                demolished_levels = demolished_levels
                    .checked_add(i64::from(*height) - i64::from(*selected_height))
                    .unwrap();
            } else {
                demolished_levels = demolished_levels.checked_add(i64::from(*height)).unwrap();
            }
        }
        if min_demolished_levels == -1 || demolished_levels < min_demolished_levels {
            min_demolished_levels = demolished_levels
        }
    }
    return min_demolished_levels;
}

/// Dynamic approach with: ``O(N*log(N)) + O(N) + O(N) ~ O(N*log(N))``
/// 1. Sort: ``O(N*log(N))``
/// 2. Integrate: ``O(N)``
/// 3. Find min: ``O(N)``
///
fn dynamic_search(list_of_building_heights: &Vec<i32>) -> i64 {
    // Sort
    let mut local_building_heights = list_of_building_heights.to_vec();
    local_building_heights.sort();
    // Memoization
    let mut integrate_building_heights = vec![];
    let mut accu: i64 = 0;
    for height in &local_building_heights {
        accu = accu + i64::from(*height);
        integrate_building_heights.push(accu);
    }
    let mut min_demolished_levels = -1;
    for (idx, height) in local_building_heights.into_iter().enumerate() {
        let mut left_levels = 0;
        if idx > 0 {
            left_levels = integrate_building_heights[idx - 1]
        }
        let last_idx = integrate_building_heights.len() - 1;
        let right_levels = integrate_building_heights[last_idx]
            - integrate_building_heights[idx]
            - ((last_idx - idx) as i64 * i64::from(height));
        let demolished_levels = left_levels + right_levels;
        if min_demolished_levels == -1 || demolished_levels < min_demolished_levels {
            min_demolished_levels = demolished_levels
        }
    }
    return min_demolished_levels;
}

fn has_negative(list_of_building_heights: &Vec<i32>) -> bool {
    for val in list_of_building_heights {
        if *val < 0 {
            return true;
        }
    }
    return false;
}

fn run(list_of_building_heights: &Vec<i32>, mode: &Mode) -> Result<i64, GenericError> {
    if list_of_building_heights.is_empty() {
        return Err(GenericError::new("vector empty!"));
    }
    if has_negative(list_of_building_heights) {
        return Err(GenericError::new("Interval has bunker!"));
    }
    if list_of_building_heights.len() == 1 {
        return Ok(0);
    }
    match mode {
        Mode::Naive => Ok(naive_search(&list_of_building_heights)),
        Mode::DynamicProgramming => Ok(dynamic_search(list_of_building_heights)),
    }
}

fn to_time(duration: Duration) -> String {
    let hours = ((duration.as_secs() as f32) / 60.0) % 60.0;
    let minutes = (duration.as_secs() as f32) / 60.0;
    let micros = duration.as_micros() % 1_000_000;
    format!(
        "{:02.0}:{:02.0}:{:02.0}.{:06}",
        hours,
        minutes,
        duration.as_secs(),
        micros
    )
}

fn execute_test(list: &Vec<i32>) {
    println!("[RUN    ] Execute test: naive approach");
    let naive_result;
    let naive_start = Instant::now();
    naive_result = run(&list, &Mode::Naive);
    let naive_duration = naive_start.elapsed();
    let naive_val = naive_result.unwrap();
    println!(
        "[SUCCESS] Execute test: naive approach with '{}'",
        naive_val
    );

    println!("[RUN    ] Execute test: dynamic approach");
    let dynamic_result;
    let dynamic_start = Instant::now();
    dynamic_result = run(&list, &Mode::DynamicProgramming);
    let dynamic_duration = dynamic_start.elapsed();
    let dynamic_val = dynamic_result.unwrap();
    println!(
        "[SUCCESS] Execute test: dynamic approach with '{}'",
        dynamic_val
    );

    assert!(naive_val == dynamic_val);

    println!(
        "[EVAL   ] Naive Approach took   {} || {:12}us",
        to_time(naive_duration),
        naive_duration.as_micros()
    );
    println!(
        "[EVAL   ] Dynamic Approach took {} || {:12}us",
        to_time(dynamic_duration),
        dynamic_duration.as_micros()
    );
}

fn execute_random_test(n: i32) {
    // Random Test Suite
    println!("[#######]");
    println!("[RUN    ] Execute random test");
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let mut rand_vec: Vec<i32> = Vec::new();
        for _i in 0..10_000 {
            rand_vec.push(rng.gen_range(0..i32::MAX));
        }
        execute_test(&rand_vec);
    }
}

#[allow(dead_code)]
fn write_to_disk(list: &Vec<i32>) {
    let mut wtr = match csv::Writer::from_path("sample.csv") {
        Ok(file_wrt) => file_wrt,
        Err(e) => panic!("Could not open file! {}", e),
    };
    for itr in list {
        let _ = wtr.serialize(itr);
    }
    let _ = wtr.flush();
}

fn read_from_disk(file: String) -> Vec<i32> {
    let mut vector = vec![];
    let mut rdr = match csv::Reader::from_path(file) {
        Ok(file_rdr) => file_rdr,
        Err(e) => panic!("Could not open file! {}", e),
    };
    for result in rdr.deserialize() {
        match result {
            Ok(record) => vector.push(record),
            Err(_) => continue,
        };
    }
    vector
}

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[arg(short, long, default_value = "resources/level_buildings.csv")]
    file: String,
    /// Number of random rounds
    #[arg(short, long, default_value = "0")]
    number_of_rand_runs: i32,
}

/// # Level Buildings (Given a list of building hights.)
/// Select the building hight that minimizes the number of removed levels.
/// For each building smaller than the selected building, we remove all levels.
/// For each building higher than the selected building, we remove all levels above the selected building height.
///
/// ```
/// Input: [1, 2, 3, 4, 5]
///          _
///        _|x|                        S - Selected building height
///      _|x|x|            _ _ _       x - Demolish levels
///    _|o|o|o|     ->    |o|o|o|      o - keep levels
///  _|x|o|o|o|           |o|o|o|
/// |x|x|o|o|o|        _ _|o|o|o|
///  0 1 2 3 4
///      S
/// ```
/// Return minimal number of demolished levels
///
/// ## Trival:
/// - For each building height, test result: ``O(N * (N-1))``
///
/// ## Better solution:
/// Complexity: ``O(N log N) + O(N) + O(N) = O(N log N)``
/// - Sort the building heights: ``O(N log N)``
/// - Memoization: Integrate building height over the list: ``O(N)``
/// - For each possible building height, we can check the number of levels by taking the
///     - left value: number of levels up to the selected building which we need to remove
///     - last value: Total number of levels of all builds where substract the left value and the height of the selected buildings times the number of building after the selected building
/// - Walk over all levels and pick the best: ``O(N)``
///
///
/// # Notes:
/// - input: ``[1, 2, 3, 4, 5]``
/// - integrated: ``[1, 3, 6, 10, 15]``
/// - Select idx: ``2``, height: ``3``
/// - ``val[idx-1] = 3`` Levels need to be removed
/// - ``val[end] = 15 - val[idx] - (len(integrated) - 1 - idx ) * input[idx]
///            = 15 - 6 - (5 - 1 - 2) * 3 ``=`` 9 - (2 * 3) = 3``
fn main() {
    let args = Cli::parse();

    // Test big dataset with overlap
    if !args.file.is_empty() {
        println!("[#######]");
        println!("[RUN    ] Test with sample file");
        let vec = read_from_disk(args.file);
        execute_test(&vec);
        println!("[SUCCESS] Test with sample file");
    } else {
        println!("[#######]");
        println!("[Skipped] Test with sample file");
    }

    execute_random_test(args.number_of_rand_runs);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    /// ```
    ///            _
    ///          _|x|
    ///        _|x|x|
    ///      _|o|o|o|
    ///    _|x|o|o|o|
    ///   |x|x|o|o|o|
    ///    0 1 2 3 4
    ///        S
    /// ```
    #[test]
    fn test_linear() {
        let building_heights = vec![1, 2, 3, 4, 5];
        let min_levels = 6;
        assert!(min_levels == run(&building_heights, &Mode::Naive).unwrap());
        assert!(min_levels == run(&building_heights, &Mode::DynamicProgramming).unwrap());
    }

    /// ```
    ///            _
    ///           |x|
    ///           |x|
    ///           |x|
    ///          _|x|
    ///         |o|o|
    ///         |o|o|
    ///         |o|o|
    ///         |o|o|
    ///    _ _ _|o|o|
    ///   |x|x|x|o|o|
    ///    0 1 2 3 4
    ///          S
    /// ```
    #[test]
    fn test_big_diff() {
        let building_heights = vec![1, 1, 1, 6, 10];
        let min_levels = 7;
        assert!(min_levels == run(&building_heights, &Mode::Naive).unwrap());
        assert!(min_levels == run(&building_heights, &Mode::DynamicProgramming).unwrap());
    }

    /// ```
    ///    _ _ _ _ _
    ///   |o|o|o|o|o|
    ///   |o|o|o|o|o|
    ///   |o|o|o|o|o|
    ///   |o|o|o|o|o|
    ///   |o|o|o|o|o|
    ///   |o|o|o|o|o|
    ///   |o|o|o|o|o|
    ///   |o|o|o|o|o|
    ///   |o|o|o|o|o|
    ///   |o|o|o|o|o|
    ///    0 1 2 3 4
    ///    S
    /// ```
    #[test]
    fn test_equal_height() {
        let building_heights = vec![10, 10, 10, 10, 10];
        let min_levels = 0;
        assert!(min_levels == run(&building_heights, &Mode::Naive).unwrap());
        assert!(min_levels == run(&building_heights, &Mode::DynamicProgramming).unwrap());
    }

    /// ```
    ///           _
    ///          |o|
    ///          |o|
    ///          |o|
    ///          |o|
    ///          |o|
    ///          |o|
    ///          |o|
    ///          |o|
    ///          |o|
    ///   _ _ _ _|o|
    ///   0 1 2 3 4
    ///           S
    /// ```
    #[test]
    fn test_single_peak() {
        let building_heights = vec![0, 0, 0, 0, 10];
        let min_levels = 0;
        assert!(min_levels == run(&building_heights, &Mode::Naive).unwrap());
        assert!(min_levels == run(&building_heights, &Mode::DynamicProgramming).unwrap());
    }

    #[test]
    fn test_negative() {
        // Negative numbers
        let building_heights = vec![1, -1];
        assert!(run(&building_heights, &Mode::Naive).is_err());
        assert!(run(&building_heights, &Mode::DynamicProgramming).is_err());
    }

    #[test]
    fn test_empty() {
        // Empty list
        let building_heights = vec![];
        assert!(run(&building_heights, &Mode::Naive).is_err());
        assert!(run(&building_heights, &Mode::DynamicProgramming).is_err());
    }

    #[test]
    fn test_single_element() {
        // Single Element
        let building_heights = vec![1];
        assert!(run(&building_heights, &Mode::Naive).unwrap() == 0);
        assert!(run(&building_heights, &Mode::DynamicProgramming).unwrap() == 0);
    }
}
