use clap::Parser;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use std::time::Instant;

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Interval {
    low: i32,
    high: i32,
}

pub enum Mode {
    Naive,              // O(N *N)
    DynamicProgramming, // O(N log(N) + N)
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
        } else if lhs.low <= rhs.high && lhs.high >= rhs.low {
            return true;
        }
        false
    }

    fn has_single_interval(list: &Vec<Interval>, mode: &Mode) -> (bool, Interval) {
        if list.is_empty() {
            return (false, Interval::new(0, 0));
        }
        match mode {
            Mode::Naive => Interval::naive_search(&list),
            Mode::DynamicProgramming => Interval::dynamic_search(list),
        }
    }

    /**
     * Naive approach with: O(N * N)
     *
     * Compare each intervale with all other intervals.
     * Early exit when we find one interval that doesn't overlap with an other interval from the
     * list.
     */
    fn naive_search(list: &Vec<Interval>) -> (bool, Interval) {
        for (idx, itr) in list.iter().enumerate() {
            let mut has_overlap = false;
            for (idx2, itr2) in list.iter().enumerate() {
                if idx == idx2 {
                    continue;
                } else if Interval::overlaps(&itr, &itr2) {
                    has_overlap = true
                }
            }
            if !has_overlap {
                return (true, itr.copy());
            }
        }
        (false, Interval::new(0, 0))
    }

    /**
     * Dynamic approach with: O(N log N) + O(N)
     * (1) Sort the array: O(N log N)
     * (2) Touch each element and compare to memorized interval: O(N)
     *
     * Memoization technique: We use one interval to memorize all the intervals we've seen. When a
     * interval overlaps with it then we grow this interval. This means for each element, we only
     * need to compare against this interval. If it doesn't overlap then we create a new interval.
     * If this is the last element or the next element does not overlap then we found an interval
     * that doesn't overlap with any other interval.
     * Early exit when we find one interval that doesn't overlap with an other interval from the
     * list.
     */
    fn dynamic_search(list: &Vec<Interval>) -> (bool, Interval) {
        /* Sort list (and copy) */
        let mut sorted_list = list.to_vec();
        sorted_list.sort();

        /* Initialize other helper variables */
        let mut span = Interval::new(0, 0);
        let mut found = false;
        let idx_max = sorted_list.len() - 1;
        for (idx, itr) in sorted_list.into_iter().enumerate() {
            /* Update buffer and skip first check */
            if idx == 0 {
                span = itr;
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
                    /* First is single */
                    return (true, span.copy());
                } else if idx == idx_max {
                    /* Last is single */
                    return (true, itr.copy());
                } else if found {
                    /*
                     * Middle is single
                     *
                     * The last interval did't overlap with the temporary interval and the current
                     * interval also doesn't overlap with the last one. The last one does not have
                     * an overlap with any other interval.
                     * */
                    return (true, span.copy());
                }
                span.low = itr.low;
                span.high = itr.high;
                found = true;
            }
        }
        (false, Interval::new(0, 0))
    }
}

fn run_sanity_check() {
    /* Sanity check  */
    println!("[RUN    ] Sanity check");
    let interval_a = Interval::new(0, 4);
    let interval_b = Interval::new(3, 5);
    let interval_c = Interval::new(4, 5);
    let interval_d = Interval::new(6, 7);

    /* Test helper functions  */
    println!("[RUN    ] Test helper functions");
    assert!(Interval::overlaps(&interval_a, &interval_b));
    assert!(Interval::overlaps(&interval_b, &interval_c));
    assert!(Interval::overlaps(&interval_a, &interval_a));

    assert!(!Interval::overlaps(&interval_a, &interval_d));
    assert!(!Interval::overlaps(&interval_b, &interval_d));
    assert!(!Interval::overlaps(&interval_c, &interval_d));
    println!("[SUCCESS] Test helper functions");
}

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
    let mut interval;

    /* Naive approach */
    (result, interval) = Interval::has_single_interval(&unmatched_first, &Mode::Naive);
    assert!(result && interval == Interval::new(0, 3));
    (result, interval) = Interval::has_single_interval(&unmatched_last, &Mode::Naive);
    assert!(result && interval == Interval::new(25, 50));
    (result, interval) = Interval::has_single_interval(&unmatched_middle, &Mode::Naive);
    assert!(result && interval == Interval::new(7, 9));
    (result, _) = Interval::has_single_interval(&matched, &Mode::Naive);
    assert!(!result);
    println!("[SUCCESS] Sanity check: naive approach");

    /* dynamic approach */
    (result, interval) = Interval::has_single_interval(&unmatched_first, &Mode::DynamicProgramming);
    assert!(result && interval == Interval::new(0, 3));
    (result, interval) = Interval::has_single_interval(&unmatched_last, &Mode::DynamicProgramming);
    assert!(result && interval == Interval::new(25, 50));
    (result, interval) =
        Interval::has_single_interval(&unmatched_middle, &Mode::DynamicProgramming);
    assert!(result && interval == Interval::new(7, 9));
    (result, _) = Interval::has_single_interval(&matched, &Mode::Naive);
    assert!(!result);
    println!("[SUCCESS] Sanity check: dynamic approach");
}

/*
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
*/

fn read_from_disk(file: String) -> Vec<Interval> {
    let mut vector = vec![];
    let mut rdr = match csv::Reader::from_path(file) {
        Ok(file_rdr) => file_rdr,
        Err(e) => panic!("Could not open file! {}", e),
    };
    for result in rdr.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        match result {
            Ok(record) => vector.push(record),
            Err(_) => continue,
        };
    }
    vector
}

fn execute_test(list: &Vec<Interval>) {
    println!("[RUN    ] Execute test: naive approach");
    let naive_result;
    let naive_start = Instant::now();
    (naive_result, _) = Interval::has_single_interval(&list, &Mode::Naive);
    let naive_duration = naive_start.elapsed();
    println!(
        "[SUCCESS] Execute test: naive approach with '{}'",
        naive_result
    );

    println!("[RUN    ] Execute test: dynamic approach");
    let dynamic_result;
    let dynamic_start = Instant::now();
    (dynamic_result, _) = Interval::has_single_interval(&list, &Mode::DynamicProgramming);
    let dynamic_duration = dynamic_start.elapsed();
    println!(
        "[SUCCESS] Execute test: dynamic approach with '{}'",
        dynamic_result
    );

    assert!(naive_result == dynamic_result);

    println!(
        "[EVAL   ] Naive Approach took {} us",
        naive_duration.as_micros()
    );
    println!(
        "[EVAL   ] Dynamic Approach took {} us",
        dynamic_duration.as_micros()
    );
}

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[arg(long, default_value = "")]
    file_with_overlap: String,
    /// The path to the file to read
    #[arg(long, default_value = "")]
    file_without_overlap: String,
    #[arg(long, default_value = "0")]
    number_of_rand_runs: i32,
}

fn main() {
    let args = Cli::parse();

    run_sanity_check();
    run_small_examples();

    /* Test big dataset with overlap  */
    if !args.file_with_overlap.is_empty() {
        println!("[RUN    ] Test with overlap");
        let vec = read_from_disk(args.file_with_overlap);
        execute_test(&vec);
        println!("[SUCCESS] Test with overlap");
    } else {
        println!("[Skipped] Test with overlap");
    }

    /* Test big dataset without overlap  */
    if !args.file_without_overlap.is_empty() {
        println!("[RUN    ] Test with overlap");
        let vec = read_from_disk(args.file_without_overlap);
        execute_test(&vec);
        println!("[SUCCESS] Test with overlap");
    } else {
        println!("[Skipped] Test without overlap");
    }

    /* Random Test Suite */
    println!("[RUN    ] Execute random test");
    let max_size = 2_i32.pow(22);
    for _ in 0..args.number_of_rand_runs {
        let mut rng = rand::thread_rng();
        let mut rand_vec: Vec<Interval> = Vec::new();
        for _i in 0..100_000 {
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
