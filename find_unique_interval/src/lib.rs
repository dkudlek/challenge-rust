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

pub mod find_unique_interval {

    use serde::Deserialize;
    use serde::Serialize;

    /// An Interval structure containing the lower and upper bound
    #[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Deserialize, Serialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    pub struct Interval {
        low: i32,
        high: i32,
    }

    impl Interval {
        pub fn new(low: i32, high: i32) -> Interval {
            Interval { low, high }
        }

        pub fn copy(&self) -> Interval {
            Interval {
                low: self.low,
                high: self.high,
            }
        }

        pub fn overlaps(lhs: &Interval, rhs: &Interval) -> bool {
            if rhs.low <= lhs.high && rhs.high >= lhs.low {
                return true;
            }
            false
        }
    }
    pub fn has_single_interval<F>(list: &[Interval], f: F) -> Option<Interval>
    where
        F: Fn(&[Interval]) -> Option<Interval>,
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
    pub fn naive_search(list: &[Interval]) -> Option<Interval> {
        for (idx, itr) in list.iter().enumerate() {
            let mut has_overlap = false;
            for (idx2, itr2) in list.iter().enumerate() {
                if idx == idx2 {
                    continue;
                } else if Interval::overlaps(itr, itr2) {
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
    pub fn dynamic_search(list: &[Interval]) -> Option<Interval> {
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use crate::find_unique_interval::*;

    struct IntervalTest {
        name: String,
        first_interval: Interval,
        second_interval: Interval,
        result: bool,
    }

    #[test]
    fn run_sanity_check() {
        let tests = vec![
            IntervalTest {
                name: "0".to_string(),
                first_interval: Interval::new(0, 4),
                second_interval: Interval::new(3, 5),
                result: true,
            },
            IntervalTest {
                name: "1".to_string(),
                first_interval: Interval::new(3, 5),
                second_interval: Interval::new(4, 5),
                result: true,
            },
            IntervalTest {
                name: "2".to_string(),
                first_interval: Interval::new(0, 4),
                second_interval: Interval::new(0, 4),
                result: true,
            },
            IntervalTest {
                name: "3".to_string(),
                first_interval: Interval::new(0, 4),
                second_interval: Interval::new(0, 4),
                result: true,
            },
            IntervalTest {
                name: "4".to_string(),
                first_interval: Interval::new(0, 4),
                second_interval: Interval::new(6, 7),
                result: false,
            },
            IntervalTest {
                name: "5".to_string(),
                first_interval: Interval::new(3, 5),
                second_interval: Interval::new(6, 7),
                result: false,
            },
            IntervalTest {
                name: "6".to_string(),
                first_interval: Interval::new(4, 5),
                second_interval: Interval::new(6, 7),
                result: false,
            },
        ];
        for next in tests {
            assert_eq!(
                next.result,
                Interval::overlaps(&next.first_interval, &next.second_interval),
                "Testing: {}",
                next.name
            );
        }
    }

    struct AlgoTest {
        name: String,
        vector: Vec<Interval>,
        result: Option<Interval>,
    }

    #[test]
    fn run_small_examples() {
        let tests = vec![
            AlgoTest {
                name: "empty".to_string(),
                vector: Vec::<Interval>::new(),
                result: None,
            },
            AlgoTest {
                name: "one".to_string(),
                vector: vec![Interval::new(0, 3)],
                result: Some(Interval::new(0, 3)),
            },
            AlgoTest {
                name: "unmatched first".to_string(),
                vector: vec![
                    Interval::new(0, 3),
                    Interval::new(4, 6),
                    Interval::new(5, 7),
                    Interval::new(7, 10),
                ],
                result: Some(Interval::new(0, 3)),
            },
            AlgoTest {
                name: "unmatched last".to_string(),
                vector: vec![
                    Interval::new(4, 6),
                    Interval::new(5, 7),
                    Interval::new(7, 10),
                    Interval::new(25, 50),
                ],
                result: Some(Interval::new(25, 50)),
            },
            AlgoTest {
                name: "unmatched middle".to_string(),
                vector: vec![
                    Interval::new(3, 5),
                    Interval::new(4, 6),
                    Interval::new(7, 9),
                    Interval::new(10, 30),
                    Interval::new(10, 20),
                ],
                result: Some(Interval::new(7, 9)),
            },
            AlgoTest {
                name: "matched".to_string(),
                vector: vec![
                    Interval::new(1, 3),
                    Interval::new(2, 4),
                    Interval::new(3, 5),
                    Interval::new(4, 6),
                ],
                result: None,
            },
        ];

        for case in &tests {
            assert_eq!(
                case.result,
                has_single_interval(&case.vector, naive_search),
                "Test {}",
                case.name
            );
        }
        for case in &tests {
            assert_eq!(
                case.result,
                has_single_interval(&case.vector, dynamic_search),
                "Test {}",
                case.name
            );
        }
    }
}
