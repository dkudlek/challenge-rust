/**
 * MIT License
 *
 * Copyright (c) 2022 David Kudlek
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */
/*
 * Given a list of building hights.
 * Select the building hight that minimizes the number of removed levels.
 * For each building smaller than the selected building, we remove all levels.
 * For each building higher than the selected building, we remove all levels above the selected building height.
 *
 * Input: [1, 2, 3, 4, 5]
 *          _
 *        _|x|                        S - Selected building height
 *      _|x|x|            _ _ _       x - Demolish levels
 *    _|o|o|o|     ->    |o|o|o|      o - keep levels
 *  _|x|o|o|o|           |o|o|o|
 * |x|x|o|o|o|        _ _|o|o|o|
 *  0 1 2 3 4
 *      S
 *
 * Return minimal number of demolished levels
 *
 * Trival:
 * - For each building height, test result: O(N * (N-1))
 *
 * Better solution:
 * - Sort the building heights: O(N log N)
 * - Memoization: Integrate building height over the list: O(N)
 * - For each possible building height, we can check the number of levels by taking the
 *     - left value: number of levels up to the selected building which we need to remove
 *     - last value: Total number of levels of all builds where substract the left value and the height of the selected buildings times the number of building after the selected building
 * - Walk over all levels and pick the best: O(N)
 *
 * Total: O(N log N) + O(N) + O(N) = O(N log N)
 *
 * Notes:
 * - input: [1, 2, 3, 4, 5]
 * - integrated: [1, 3, 6, 10, 15]
 * - Select idx: 2, height: 3
 * - val[idx-1] = 3 Levels need to be removed
 * - val[end] = 15 - val[idx] - (len(integrated) - 1 - idx ) * input[idx]
 *            = 15 - 6 - (5 - 1 - 2) * 3 = 9 - (2 * 3) = 3
 *
 */
use rand::Rng;
use std::time::Duration;
use std::time::Instant;

pub enum Mode {
    Naive,              // O(N *N)
    DynamicProgramming, // O(N log(N) + N)
}

/**
 * Naive approach with: O(N * N)
 *
 * Compare each intervale with all other intervals.
 * Early exit when we find one interval that doesn't overlap with an other interval from the
 * list.
 */
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

/**
 * Dynamic approach with: O(N*log(N)) + O(N) + O(N) ~ O(N*log(N))
 * (1) Sort: O(N*log(N))
 * (2) Integrate: O(N)
 * (3) Find min: O(N)
 *
 */
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

fn run(list_of_building_heights: &Vec<i32>, mode: &Mode) -> i64 {
    if list_of_building_heights.is_empty() || has_negative(list_of_building_heights) {
        return -1;
    } else if list_of_building_heights.len() == 1 {
        return 0;
    }
    match mode {
        Mode::Naive => naive_search(&list_of_building_heights),
        Mode::DynamicProgramming => dynamic_search(list_of_building_heights),
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
    println!(
        "[SUCCESS] Execute test: naive approach with '{}'",
        naive_result
    );

    println!("[RUN    ] Execute test: dynamic approach");
    let dynamic_result;
    let dynamic_start = Instant::now();
    dynamic_result = run(&list, &Mode::DynamicProgramming);
    let dynamic_duration = dynamic_start.elapsed();
    println!(
        "[SUCCESS] Execute test: dynamic approach with '{}'",
        dynamic_result
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

fn execute_random_test(n: i32) {
    /* Random Test Suite */
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

fn sanity_check() {
    println!("[RUN    ] Sanity check");
    /*
                _
              _|x|
            _|x|x|
          _|o|o|o|
        _|x|o|o|o|
       |x|x|o|o|o|
        0 1 2 3 4
            S
    */
    let mut building_heights = vec![1, 2, 3, 4, 5];
    let mut min_levels = 6;
    assert!(min_levels == run(&building_heights, &Mode::Naive));
    assert!(min_levels == run(&building_heights, &Mode::DynamicProgramming));
    /*
                _
               |x|
               |x|
               |x|
              _|x|
             |o|o|
             |o|o|
             |o|o|
             |o|o|
        _ _ _|o|o|
       |x|x|x|o|o|
        0 1 2 3 4
              S
    */
    println!("[SUCCESS] Sanity check: Default test");
    building_heights = vec![1, 1, 1, 6, 10];
    min_levels = 7;
    assert!(min_levels == run(&building_heights, &Mode::Naive));
    assert!(min_levels == run(&building_heights, &Mode::DynamicProgramming));
    /*
        _ _ _ _ _
       |o|o|o|o|o|
       |o|o|o|o|o|
       |o|o|o|o|o|
       |o|o|o|o|o|
       |o|o|o|o|o|
       |o|o|o|o|o|
       |o|o|o|o|o|
       |o|o|o|o|o|
       |o|o|o|o|o|
       |o|o|o|o|o|
        0 1 2 3 4
        S
    */
    println!("[SUCCESS] Sanity check: non linear building heights successfull");
    building_heights = vec![10, 10, 10, 10, 10];
    min_levels = 0;
    assert!(min_levels == run(&building_heights, &Mode::Naive));
    assert!(min_levels == run(&building_heights, &Mode::DynamicProgramming));
    /*
               _
              |o|
              |o|
              |o|
              |o|
              |o|
              |o|
              |o|
              |o|
              |o|
       _ _ _ _|o|
       0 1 2 3 4
               S
    */
    println!("[SUCCESS] Sanity check: equal building heights successfull (pick first)");

    building_heights = vec![0, 0, 0, 0, 10];
    min_levels = 0;
    assert!(min_levels == run(&building_heights, &Mode::Naive));
    assert!(min_levels == run(&building_heights, &Mode::DynamicProgramming));
    println!("[SUCCESS] Sanity check: pick last");

    // Negative numbers
    building_heights = vec![1, -1];
    assert!(run(&building_heights, &Mode::Naive) == -1);
    assert!(run(&building_heights, &Mode::DynamicProgramming) == -1);
    println!("[SUCCESS] Sanity check: Negative levels");

    // Empty list
    building_heights = vec![];
    assert!(run(&building_heights, &Mode::Naive) == -1);
    assert!(run(&building_heights, &Mode::DynamicProgramming) == -1);
    println!("[SUCCESS] Sanity check: Empty list");

    // Single Element
    building_heights = vec![1];
    assert!(run(&building_heights, &Mode::Naive) == 0);
    assert!(run(&building_heights, &Mode::DynamicProgramming) == 0);
    println!("[SUCCESS] Sanity check: Single Element");
}

fn main() {
    sanity_check();
    execute_random_test(3);
}
