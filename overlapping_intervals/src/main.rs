

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone)]
pub(crate) struct Interval{
    low: i64, 
    high: i64
}

pub enum Mode{
    Naive, 
    BinSearch,
    DynamicProgramming
}

impl Interval{
    pub fn new(low: i64, high: i64) -> Interval{
        Interval{
            low:low, 
            high:high
        }
    }

    fn copy(&self) -> Interval{
        Interval { low: self.low, high: self.high }
    }


    fn overlaps(lhs: &Interval, rhs: &Interval) -> bool{
        if rhs.low <= lhs.high && rhs.high >= lhs.low{
            return true
        }else if  lhs.low <= rhs.high && lhs.high >= rhs.low{
            return true
        }
        false
    }

    fn has_single_interval(list: &Vec<Interval>, mode: &Mode) -> (bool, Interval){
        match mode{
            Mode::Naive => Interval::naive_search(&list),
            Mode::BinSearch => (false, Interval::new(0, 0)),
            Mode::DynamicProgramming => Interval::dynamic_search(list),
        }
    }

    fn naive_search(list: &Vec<Interval>) -> (bool, Interval){
        for (idx, itr) in list.iter().enumerate(){
            let mut has_overlap = false;
            for (idx2, itr2) in list.iter().enumerate(){
                if idx == idx2{
                    continue;
                }else if Interval::overlaps(&itr, &itr2) {
                    has_overlap = true
                }
            }
            if !has_overlap{
                return (true, itr.copy())
            }
        }
        (false, Interval::new(0, 0))
    }

    fn dynamic_search(list: &Vec<Interval>) -> (bool, Interval){
        let mut span = match list.first(){
            Some(val) => val.copy(), 
            None => return (false, Interval::new(0, 0))
        };
        let mut sorted_list = list.to_vec();
        sorted_list.sort();
        let mut found = false;
        let idx_max = sorted_list.len() - 1;
        for (idx, itr) in sorted_list.into_iter().enumerate(){
            let has_overlap = Interval::overlaps(&span, &itr);
            if has_overlap{
                span.high = itr.high;
                found = false;
            }else{
                if idx == 1{
                    /* First is single */
                    return (true, span.copy());
                }else if idx == idx_max{
                    /* Last is single */
                    return (true, itr.copy())
                }else if found {
                    /* Middle is single */
                    return (true, span.copy())
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
    println!("Hello, world!");
    let interval_a = Interval::new(0, 4);
    let interval_b = Interval::new(3, 5);
    let interval_c = Interval::new(4, 5);
    let interval_d = Interval::new(6, 7);

    /* Test helper functions  */
    assert!(Interval::overlaps(&interval_a, &interval_b));
    assert!(Interval::overlaps(&interval_b, &interval_c));
    assert!(Interval::overlaps(&interval_a, &interval_a));

    assert!(!Interval::overlaps(&interval_a, &interval_d));
    assert!(!Interval::overlaps(&interval_b, &interval_d));
    assert!(!Interval::overlaps(&interval_c, &interval_d));
    println!("Test: overlaps success");

    /* Sanity check  */
    println!("sanity check");
    let unmatched_first = vec![Interval::new(0,3), Interval::new(4,6), Interval::new(5,7), Interval::new(7,10)];
    let unmatched_last = vec![Interval::new(4,6), Interval::new(5,7), Interval::new(7,10), Interval::new(25,50)];
    let unmatched_middle = vec![Interval::new(3,5), Interval::new(4,6), Interval::new(7,9), Interval::new(10,30), Interval::new(10,20)];
    let matched = vec![Interval::new(1,3), Interval::new(2,4), Interval::new(3,5), Interval::new(4,6)];
    let mut result;
    let mut interval;

    /* Naive approach */
    (result, interval) = Interval::has_single_interval(&unmatched_first, &Mode::Naive);
    assert!(result && interval == Interval::new(0,3));
    (result, interval) = Interval::has_single_interval(&unmatched_last, &Mode::Naive);
    assert!(result && interval == Interval::new(25,50));
    (result, interval) = Interval::has_single_interval(&unmatched_middle, &Mode::Naive);
    assert!(result && interval == Interval::new(7,9));
    (result, _) = Interval::has_single_interval(&matched, &Mode::Naive);
    assert!(!result);

    /* dynamic approach */
    (result, interval) = Interval::has_single_interval(&unmatched_first, &Mode::DynamicProgramming);
    assert!(result && interval == Interval::new(0,3));
    (result, interval) = Interval::has_single_interval(&unmatched_last, &Mode::DynamicProgramming);
    assert!(result && interval == Interval::new(25,50));
    (result, interval) = Interval::has_single_interval(&unmatched_middle, &Mode::DynamicProgramming);
    assert!(result && interval == Interval::new(7,9));
    (result, _) = Interval::has_single_interval(&matched, &Mode::Naive);
    assert!(!result);
}
