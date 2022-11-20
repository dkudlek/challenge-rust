use std::{cmp::max, collections::HashSet};

///
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
///

///
/// Room in a dungeon
///
/// idx: i32 => Id of the current room
///
/// passages: i32 => The number of passages that can be seen when visiting the room
///
struct Room {
    idx: usize, // idx > 0
    passages: usize,
}

impl Room {
    /// Creates a new room
    pub fn new(idx: usize, passages: usize) -> Room {
        Room {
            idx: idx,
            passages: passages,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Action {
    Teleport,
    Move,
}

fn get_lower_bound(n: usize) -> usize {
    // guard
    if n < 2 {
        panic!(
            "Number of nodes is below minimum! Range must be 2..10e5, buf value is {}!",
            n
        );
    }
    let value = n / 2;
    let remainder = n % 2;
    if remainder > 0 {
        return value + 1;
    }
    value
}

fn get_upper_bound(n: usize) -> usize {
    // guard
    if n < 2 {
        panic!(
            "Number of nodes is below minimum! Range must be 2..10e5, buf value is {}!",
            n
        );
    }
    let mut buf = 1; // for n = 2
    for i in 3..=n {
        buf = buf + i - 1;
    }
    buf
}

struct Explorer {
    number_of_rooms: usize,
    lower_bound: usize,
    upper_bound: usize,
    visited_rooms: HashSet<usize>,
    last_action: Action,
}

impl Explorer {
    fn new(number_of_rooms: usize) -> Explorer {
        Explorer {
            number_of_rooms: number_of_rooms,
            lower_bound: 0,
            upper_bound: get_upper_bound(number_of_rooms),
            visited_rooms: HashSet::new(),
            last_action: Action::Teleport,
        }
    }

    fn get_estimate(&self) -> usize {
        max(self.lower_bound, self.upper_bound)
    }

    fn observe(&mut self, room: &Room) -> (Action, Option<usize>) {
        let already_visited = self.visited_rooms.contains(&room.idx);
        if already_visited {
            // Already seen this room
            self.upper_bound -= 1; // One unique Path found
            #[cfg(trace)]
            println!(
                "[Visited ] lhs: {} rhs: {}",
                self.lower_bound, self.upper_bound
            );
            self.last_action = Action::Teleport;
            for i in 1..=self.number_of_rooms {
                if !self.visited_rooms.contains(&i) {
                    return (self.last_action, Some(i));
                }
            }
        } else if self.last_action == Action::Move {
            // Never seen this room and moved
            self.visited_rooms.insert(room.idx);
            self.upper_bound -= self.number_of_rooms - 1 - room.passages; // Reduce potential passages
            self.lower_bound += room.passages - 1; // Don't double count. Remove the passage that we just moved from
            #[cfg(trace)]
            println!(
                "[Move    ] lhs: {} rhs: {}",
                self.lower_bound, self.upper_bound
            );
            self.last_action = Action::Teleport;
            for i in 1..=self.number_of_rooms {
                if !self.visited_rooms.contains(&i) {
                    return (self.last_action, Some(i));
                }
            }
        } else {
            // Never seen this room and teleported
            self.visited_rooms.insert(room.idx);
            self.upper_bound -= self.number_of_rooms - 1 - room.passages; // Reduce potential passages
            self.lower_bound += room.passages;
            #[cfg(trace)]
            println!(
                "[Teleport] lhs: {} rhs: {}",
                self.lower_bound, self.upper_bound
            );
        }
        self.last_action = Action::Move;
        return (self.last_action, None);
    }
}

/// # Explore dungeon (Twisty Little Passages)
/// https://codingcompetitions.withgoogle.com/codejam/round/0000000000876ff1/0000000000a45fc0
///
/// We need to explore a dungeon. The map is hidden, but we get the unqiue ID and the
/// number of passages from this room. Each room is connect with at least one passage to
/// another room. The number of exploration actions is limited.
/// We can only use three actions:
/// - teleport to room X
/// - move which picks one passage by uniform distribution
/// - finish which also returns the estimated number of passages
///
/// The goal is to estimate the total number of unqiue passages in the dungeon and we must
/// get within: (2/3) * P <= E <= (4/3) * P where P is the actual number and E is the
/// estimate.
///
/// At the start, we get the number of rooms, number of actions and we'll get the first observation.
/// In total we get ``(number_of_actions + 1)`` observation to explore the dungeon.
///
/// Range of test cases (T): 1..100
/// Range of nodes (N): 2 .. 10e5
/// Range of actions (K): 1 .. 8000
///
/// # Deliberations:
/// The minimum number of unique passages is: ``E_min(N) = ceil(N/2.0)``
/// - E_min(2) = 1 // two nodes can only connect via one unique passage => (o-o)
/// - E_min(3) = ceil(3/2) = ceil(1.5) = 2 // Each node must be connect with at least on unique passage => (o-o-o)
/// - E_min(4) = ceil(4/2) = 2 // Two nodes always connect, but they don't interconnect (E_min(2) + E_min(2) ) => (o-o) (o-o)
/// - E_min(5) = ceil(5/2) = 3 // E_min(2) + E_min(3) => (o-o, o-o-o)
///
/// The maximum number of unique passages is: ``E_max(N) = E_max(N-1) + (N-1) with E_max(2) = E_min(2) = 1, N >= 2``
/// - Each additional node creates N-1 new connects (one to each exisiting node and none to itself)
/// - E_max(2) = 1 => (o-o)
/// - E_max(3) = E_max(2) + (3 - 1) = 1 + 2 = 3
/// - E_max(4) = E_max(3) + (4 - 1) = 3 + 3 = 6
/// - E_max(5) = E_max(4) + (5 - 1) = 6 + 4 = 10
///
/// We'll need to store at least the IDs of the rooms that we've already visited.
fn main() {
    println!("Hello, world!");
    // N - number of rooms
    //
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use rand::Rng;

    struct Dungeon {
        number_of_actions: usize,
        total_unique_passages: usize,
        rooms: Vec<Vec<usize>>,
    }

    impl Dungeon {
        /// idx range 1..=N
        fn get_room(&self, idx: usize) -> Room {
            Room {
                idx: idx,
                passages: self.rooms[idx - 1].len(),
            }
        }

        // selects a random passage and returns the room idx
        fn random_move(&self, idx: usize) -> usize {
            let current_idx = idx - 1;
            let mut rng = rand::thread_rng();
            let passage_idx = rng.gen_range(0..self.rooms[current_idx].len());
            self.rooms[current_idx][passage_idx]
        }
    }

    #[test]
    fn test_lower_bound() {
        assert_eq!(get_lower_bound(2), 1);
        assert_eq!(get_lower_bound(3), 2);
        assert_eq!(get_lower_bound(4), 2);
        assert_eq!(get_lower_bound(5), 3);
    }

    #[test]
    fn test_upper_bound() {
        assert_eq!(get_upper_bound(2), 1);
        assert_eq!(get_upper_bound(3), 3);
        assert_eq!(get_upper_bound(4), 6);
        assert_eq!(get_upper_bound(5), 10);
    }

    fn run_dungeon(dungeon: &Dungeon) {
        let mut rng = rand::thread_rng();
        let mut room_idx = rng.gen_range(1..=dungeon.rooms.len());
        let mut explorer = Explorer::new(dungeon.rooms.len());
        for _ in 0..=dungeon.number_of_actions {
            let (action, option) = explorer.observe(&dungeon.get_room(room_idx));
            match action {
                Action::Teleport => {
                    room_idx = option.unwrap();
                }
                Action::Move => {
                    room_idx = dungeon.random_move(room_idx);
                }
            }
        }
        let result = explorer.get_estimate();
        let lower_bound = 20 / 3 * dungeon.total_unique_passages / 10;
        let upper_bound = 40 / 3 * dungeon.total_unique_passages / 10;
        assert!(
            lower_bound <= result,
            "lhs:{} > result: {}",
            lower_bound,
            result
        );
        assert!(
            result <= upper_bound,
            "result: {} > rhs: {}",
            result,
            upper_bound
        );
    }

    /// Test ``(o-o)``
    #[test]
    fn test_smallest_example() {
        let test = vec![vec![2], vec![1]];
        let dungeon = Dungeon {
            number_of_actions: 1,
            total_unique_passages: 1,
            rooms: test,
        };
        run_dungeon(&dungeon);
    }

    /// Test ``(o-o-o)``
    #[test]
    fn test_n3_min() {
        let test = vec![vec![2], vec![1, 3], vec![2]];
        let dungeon = Dungeon {
            number_of_actions: 2,
            total_unique_passages: 2,
            rooms: test,
        };
        run_dungeon(&dungeon);
    }

    /// Test ``(o-o-o)``
    #[test]
    fn test_n3_max() {
        let test = vec![vec![2, 3], vec![1, 3], vec![1, 2]];
        let dungeon = Dungeon {
            number_of_actions: 2,
            total_unique_passages: 3,
            rooms: test,
        };
        run_dungeon(&dungeon);
    }

    /// 1 connects to 2, 3 and 5
    /// 2 connects to 1 and 3
    /// 3 connects to 1 and 2
    /// 4 connects to 5
    /// 5 connects to 1 and 4
    #[test]
    fn test_n5_max() {
        let test = vec![vec![2, 3, 5], vec![1, 3], vec![1, 2], vec![5], vec![1, 4]];
        let dungeon = Dungeon {
            number_of_actions: 3,
            total_unique_passages: 5,
            rooms: test,
        };
        run_dungeon(&dungeon);
    }
    /// 1 connects to 2, 3 and 5
    /// 2 connects to 1
    /// 3 connects to 1
    /// 4 connects to 5
    /// 5 connects to 1 and 4
    #[test]
    fn test_n5() {
        let test = vec![vec![2, 3, 5], vec![1], vec![1], vec![5], vec![1, 4]];
        let dungeon = Dungeon {
            number_of_actions: 3,
            total_unique_passages: 4,
            rooms: test,
        };
        run_dungeon(&dungeon);
    }
}
