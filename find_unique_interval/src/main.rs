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
use find_unique_interval::find_unique_interval::{
    dynamic_search, has_single_interval, naive_search, Interval,
};
use rand::Rng;
use std::time::Duration;
use std::time::Instant;

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

fn execute_test(list: &[Interval]) {
    println!("[RUN    ] Execute test: naive approach");

    let naive_start = Instant::now();
    let naive_result = has_single_interval(list, naive_search);
    let naive_duration = naive_start.elapsed();
    println!(
        "[SUCCESS] Execute test: naive approach with '{}'",
        naive_result.is_some()
    );

    println!("[RUN    ] Execute test: dynamic approach");

    let dynamic_start = Instant::now();
    let dynamic_result = has_single_interval(list, dynamic_search);
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
