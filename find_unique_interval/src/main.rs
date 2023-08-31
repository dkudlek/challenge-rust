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
use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;
use std::time::Instant;

/// An Interval structure containing the lower and upper boound
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Interval {
    low: i32,
    high: i32,
}

impl Interval {
    pub fn new(low: i32, high: i32) -> Interval {
        Interval {
            low: low,
            high: high,
        }
    }

    fn copy(&self) -> Interval {
        Interval {
            low: self.low,
            high: self.high,
        }
    }

    fn overlaps(lhs: &Interval, rhs: &Interval) -> bool {
        if rhs.low <= lhs.high && rhs.high >= lhs.low {
            return true;
        }
        false
    }

    fn has_single_interval<F>(list: &Vec<Interval>, f: F) -> Option<Interval>
    where
        F: Fn(&Vec<Interval>) -> Option<Interval>,
    {
        if list.is_empty() {
            return None;
        } else if list.len() == 1 {
            return Some(list[0].copy());
        }
        f(list)
    }

    /// Naive approach with: O(N * N)
    ///
    /// Compare each intervale with all other intervals.
    /// Early exit when we find one interval that doesn't overlap with an other interval from the
    /// list.
    fn naive_search(list: &Vec<Interval>) -> Option<Interval> {
        for (idx, itr) in list.iter().enumerate() {
            let mut has_overlap = false;
            for (idx2, itr2) in list.iter().enumerate() {
                if idx == idx2 {
                    continue;
                } else if Interval::overlaps(&itr, &itr2) {
                    has_overlap = true;
                    break; // Early exit because we found an overlapping interval
                }
            }
            if !has_overlap {
                return Some(itr.copy());
            }
        }
        None
    }

    /// Dynamic approach with: O(N*log(N)) + O(N) ~ O(N*log(N))
    /// (1) Sort the array: O(N log N)
    /// (2) Touch each element and compare to a memorized interval: O(N)
    ///
    /// Memoization technique: We use one interval to memorize all the intervals we've seen. When a
    /// interval overlaps with it then we grow this interval. This means for each element, we only
    /// need to compare against this interval. If it doesn't overlap then we create a new interval.
    /// If this is the last element or the next element does not overlap then we found an interval
    /// that doesn't overlap with any other interval.
    /// Early exit when we find one interval that doesn't overlap with an other interval from the
    /// list.
    fn dynamic_search(list: &Vec<Interval>) -> Option<Interval> {
        // Sort list (and copy)
        let mut sorted_list = list.to_vec();
        sorted_list.sort();

        // Initialize other helper variables
        let mut span = Interval::new(0, 0);
        let mut found = false;
        let idx_max = sorted_list.len() - 1;
        for (idx, itr) in sorted_list.into_iter().enumerate() {
            // Update buffer and skip first check
            if idx == 0 {
                span = itr;
                found = true;
                continue;
            }
            let has_overlap = Interval::overlaps(&span, &itr);
            if has_overlap {
                if itr.high > span.high {
                    span.high = itr.high;
                }
                found = false;
            } else {
                if idx == 1 {
                    // First is single
                    return Some(span.copy());
                } else if idx == idx_max {
                    // Last is single
                    return Some(itr.copy());
                } else if found {
                    // Middle is single
                    //
                    // The last interval did't overlap with the temporary interval and the current
                    // interval also doesn't overlap with the last one. The last one does not have
                    // an overlap with any other interval.
                    return Some(span.copy());
                }
                span.low = itr.low;
                span.high = itr.high;
                found = true;
            }
        }
        None
    }
}

#[allow(dead_code)]
fn write_to_disk(list: &Vec<Interval>) {
    let mut wtr = match csv::Writer::from_path("no_overlap.csv") {
        Ok(file_wrt) => file_wrt,
        Err(e) => panic!("Could not open file! {}", e),
    };
    for itr in list {
        let _ = wtr.serialize(itr);
    }
    let _ = wtr.flush();
}

fn read_from_disk(file: String) -> Vec<Interval> {
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

fn execute_random_test(n: i32) {
    // Random Test Suite
    println!("[#######]");
    println!("[RUN    ] Execute random test");
    let max_size = 2_i32.pow(20);
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let mut rand_vec: Vec<Interval> = Vec::new();
        for _i in 0..1_000_000 {
            let mut one: i32 = rng.gen_range(0..i32::MAX);
            let mut two: i32 = rng.gen_range(0..i32::MAX);
            if one > two {
                let delta = (one - two).abs() - max_size;
                if delta > 0 {
                    one = two + max_size;
                }
                rand_vec.push(Interval::new(two, one));
            } else {
                let delta = (one - two).abs() - max_size;
                if delta > 0 {
                    two = one + max_size;
                }
                rand_vec.push(Interval::new(one, two));
            }
        }
        execute_test(&rand_vec);
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

fn execute_test(list: &Vec<Interval>) {
    println!("[RUN    ] Execute test: naive approach");
    let naive_result;
    let naive_start = Instant::now();
    naive_result = Interval::has_single_interval(&list, Interval::naive_search);
    let naive_duration = naive_start.elapsed();
    println!(
        "[SUCCESS] Execute test: naive approach with '{}'",
        naive_result.is_some()
    );

    println!("[RUN    ] Execute test: dynamic approach");
    let dynamic_result;
    let dynamic_start = Instant::now();
    dynamic_result = Interval::has_single_interval(&list, Interval::dynamic_search);
    let dynamic_duration = dynamic_start.elapsed();
    println!(
        "[SUCCESS] Execute test: dynamic approach with '{}'",
        dynamic_result.is_some()
    );

    assert!(naive_result == dynamic_result);

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

#[derive(Parser)]
struct Cli {
    // The pattern to look for
    #[arg(long, default_value = "")]
    file_with_overlap: String,
    // The path to the file to read
    #[arg(long, default_value = "")]
    file_without_overlap: String,
    #[arg(long, default_value = "0")]
    number_of_rand_runs: i32,
}

/// # Find unique interval in list of intervals
///
/// Given a list of intervals:
/// We want to know if there's one interval which doesn't overlap with another interval
///
/// An interval overlaps if end of one and start of the other are the equal (closed interval, including start and end value)
/// e.g.
/// - ``[0, 3]`` and ``[1, 2]`` overlap
/// - ``[0, 3]`` and ``[3, 5]`` overlap
/// - ``[0, 3]`` and ``[4, 6]`` don't overlap
///
/// # Solutions :
/// 1. Naive Solution: ``O(N * N)``
/// 2. Dynamic solution: ``O(N * log(N) + N) ~ O(N*log(N))``
///
/// # Deliberations:
/// - tuple compare compares value by value:
///   - ``(1, 2) < (2, 4)``, because ``1 < 2``
///   - ``(1, 2) < (1, 3)``, because ``1 == 1 and 2 < 3``
///   - ``(1, 2) > (0, 1)``, because ``1 > 0``
fn main() {
    let args = Cli::parse();

    // Test big dataset with overlap
    if !args.file_with_overlap.is_empty() {
        println!("[#######]");
        println!("[RUN    ] Test with overlap");
        let vec = read_from_disk(args.file_with_overlap);
        execute_test(&vec);
        println!("[SUCCESS] Test with overlap");
    } else {
        println!("[#######]");
        println!("[Skipped] Test with overlap");
    }

    // Test big dataset without overlap
    if !args.file_without_overlap.is_empty() {
        println!("[#######]");
        println!("[RUN    ] Test without overlap");
        let vec = read_from_disk(args.file_without_overlap);
        execute_test(&vec);
        println!("[SUCCESS] Test without overlap");
    } else {
        println!("[#######]");
        println!("[Skipped] Test without overlap");
    }
    // Execute N tests with random data
    execute_random_test(args.number_of_rand_runs);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn run_sanity_check() {
        // Sanity check
        println!("[RUN    ] Sanity check");
        let interval_a = Interval::new(0, 4);
        let interval_b = Interval::new(3, 5);
        let interval_c = Interval::new(4, 5);
        let interval_d = Interval::new(6, 7);

        // Test helper functions
        println!("[RUN    ] Test helper functions");
        assert!(Interval::overlaps(&interval_a, &interval_b));
        assert!(Interval::overlaps(&interval_b, &interval_c));
        assert!(Interval::overlaps(&interval_a, &interval_a));

        assert!(!Interval::overlaps(&interval_a, &interval_d));
        assert!(!Interval::overlaps(&interval_b, &interval_d));
        assert!(!Interval::overlaps(&interval_c, &interval_d));
        println!("[SUCCESS] Test helper functions");
    }

    #[test]
    fn run_small_examples() {
        let unmatched_first = vec![
            Interval::new(0, 3),
            Interval::new(4, 6),
            Interval::new(5, 7),
            Interval::new(7, 10),
        ];
        let unmatched_last = vec![
            Interval::new(4, 6),
            Interval::new(5, 7),
            Interval::new(7, 10),
            Interval::new(25, 50),
        ];
        let unmatched_middle = vec![
            Interval::new(3, 5),
            Interval::new(4, 6),
            Interval::new(7, 9),
            Interval::new(10, 30),
            Interval::new(10, 20),
        ];
        let matched = vec![
            Interval::new(1, 3),
            Interval::new(2, 4),
            Interval::new(3, 5),
            Interval::new(4, 6),
        ];
        let mut result;

        // Naive approach
        println!("[RUN    ] Sanity check: naive approach");
        result = Interval::has_single_interval(&unmatched_first, Interval::naive_search);
        assert!(result.is_some() && result.unwrap() == Interval::new(0, 3));
        result = Interval::has_single_interval(&unmatched_last, Interval::naive_search);
        assert!(result.is_some() && result.unwrap() == Interval::new(25, 50));
        result = Interval::has_single_interval(&unmatched_middle, Interval::naive_search);
        assert!(result.is_some() && result.unwrap() == Interval::new(7, 9));
        result = Interval::has_single_interval(&matched, Interval::naive_search);
        assert!(result.is_none());
        println!("[SUCCESS] Sanity check: naive approach");

        // dynamic approach
        println!("[RUN    ] Sanity check: dynamic approach");
        result = Interval::has_single_interval(&unmatched_first, Interval::dynamic_search);
        assert!(result.is_some() && result.unwrap() == Interval::new(0, 3));
        result = Interval::has_single_interval(&unmatched_last, Interval::dynamic_search);
        assert!(result.is_some() && result.unwrap() == Interval::new(25, 50));
        result = Interval::has_single_interval(&unmatched_middle, Interval::dynamic_search);
        assert!(result.is_some() && result.unwrap() == Interval::new(7, 9));
        result = Interval::has_single_interval(&matched, Interval::naive_search);
        assert!(result.is_none());
        println!("[SUCCESS] Sanity check: dynamic approach");
    }
}
