use rand::Rng;
use std::time::Instant;

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone)]
pub(crate) struct Interval {
    low: i64,
    high: i64,
}

pub enum Mode {
    Naive,
    BinSearch,
    DynamicProgramming,
}

impl Interval {
    pub fn new(low: i64, high: i64) -> Interval {
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
        match mode {
            Mode::Naive => Interval::naive_search(&list),
            Mode::BinSearch => Interval::binary_search(list),
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

    fn binsearch_match(interval: Interval, list: &Vec<Interval>) -> bool {
        let mut low = 0;
        let mut high = list.len() - 1;
        while low <= high {
            let mid = (low + high) / 2;
            let midval = list[mid].copy();
            if low == high {
                break;
            }
            if midval < interval {
                if interval != midval && Interval::overlaps(&interval, &midval) {
                    return true;
                }
                low = mid + 1
            } else if midval >= interval {
                if interval != midval && Interval::overlaps(&interval, &midval) {
                    /*
                    Skip the interval that we currently look at
                    If it's equal we can look into the right branch
                    */
                    return true;
                }
                high = mid;
            } else {
                break;
            }
        }
        return false;
    }

    fn binary_search(list: &Vec<Interval>) -> (bool, Interval) {
        let mut sorted_list = list.to_vec();
        sorted_list.sort();

        for itr in sorted_list.clone().into_iter() {
            /*
             * Binary Search for each interval in our list
             */
            let has_overlap = Interval::binsearch_match(itr.copy(), &sorted_list);
            if !has_overlap {
                return (true, itr.copy());
            }
        }
        (false, Interval::new(0, 0))
    }

    /**
     * Dynamic approach with: O(N log N) + O(N)
     * (1) Sort the array: N log N
     * (2) Touch each element and compare to memorized interval
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
        let mut span = match list.first() {
            Some(val) => val.copy(),
            None => return (false, Interval::new(0, 0)),
        };
        let mut sorted_list = list.to_vec();
        sorted_list.sort();
        let mut found = false;
        let idx_max = sorted_list.len() - 1;
        for (idx, itr) in sorted_list.into_iter().enumerate() {
            let has_overlap = Interval::overlaps(&span, &itr);
            if has_overlap {
                span.high = itr.high;
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

/// .
fn main() {
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

    /* Sanity check  */
    println!("[RUN    ] Sanity check");
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

    /* Binary Search approach */
    (result, interval) = Interval::has_single_interval(&unmatched_first, &Mode::BinSearch);
    assert!(result && interval == Interval::new(0, 3));
    (result, interval) = Interval::has_single_interval(&unmatched_last, &Mode::BinSearch);
    assert!(result && interval == Interval::new(25, 50));
    (result, interval) = Interval::has_single_interval(&unmatched_middle, &Mode::BinSearch);
    assert!(result && interval == Interval::new(7, 9));
    (result, _) = Interval::has_single_interval(&matched, &Mode::Naive);
    assert!(!result);
    println!("[SUCCESS] Sanity check: binary search approach");

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

    println!("[RUN    ] Execute random test");
    let mut rng = rand::thread_rng();
    let mut rand_vec: Vec<Interval> = Vec::new();
    for _i in 0..1_000_000 {
        let one: i64 = rng.gen_range(0..i64::MAX);
        let two: i64 = rng.gen_range(0..i64::MAX);
        if one > two {
            rand_vec.push(Interval::new(two, one));
        } else {
            rand_vec.push(Interval::new(one, two));
        }

        println!("[SUCCESS] Execute random test: Test data generated");
    }

    println!("[RUN    ] Execute random test: naive approach");
    let naive_result;
    let naive_start = Instant::now();
    (naive_result, _) = Interval::has_single_interval(&rand_vec, &Mode::Naive);
    let naive_duration = naive_start.elapsed();
    println!(
        "[SUCCESS] Execute random test: naive approach with '{}'",
        naive_result
    );

    println!("[RUN    ] Execute random test: binarySearch approach");
    let bin_result;
    let bin_start = Instant::now();
    (bin_result, _) = Interval::has_single_interval(&rand_vec, &Mode::BinSearch);
    let bin_duration = bin_start.elapsed();
    println!(
        "[SUCCESS] Execute random test: binarySearch approach with '{}'",
        bin_result
    );
    assert!(naive_result == bin_result);

    println!("[RUN    ] Execute random test: dynamic approach");
    let dynamic_result;
    let dynamic_start = Instant::now();
    (dynamic_result, _) = Interval::has_single_interval(&rand_vec, &Mode::DynamicProgramming);
    let dynamic_duration = dynamic_start.elapsed();
    println!(
        "[SUCCESS] Execute random test: dynamic approach with '{}'",
        dynamic_result
    );
    assert!(bin_result == dynamic_result);

    println!(
        "[EVAL   ] Naive Approach took {} s",
        naive_duration.as_secs()
    );
    println!(
        "[EVAL   ] binarySearch Approach took {} s",
        bin_duration.as_secs()
    );
    println!(
        "[EVAL   ] Dynamic Approach took {} s",
        dynamic_duration.as_secs()
    );

    assert!(naive_result == dynamic_result);
}
