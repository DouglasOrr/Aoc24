use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

type Direction = u8; // 0 = up, 1 = right, 2 = down, 3 = left

#[derive(PartialEq, Eq, Debug)]
struct Node {
    cost: usize,
    position: usize,
    direction: Direction,
    parent: (usize, Direction),
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}
impl Ord for Node {
    // Note: a node is "less" if its cost is greater
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other.cost.cmp(&self.cost);
    }
}

fn main() {
    let data = fs::read_to_string("inputs/day16/input.txt").unwrap();
    let size = data.split("\n").count();
    let flat_data = data.replace("\n", "");
    let cells = flat_data.as_bytes();
    let start = cells.iter().position(|c| *c == b'S').unwrap();
    let end = cells.iter().position(|c| *c == b'E').unwrap();

    // Part one
    {
        let mut queue: BinaryHeap<Node> = BinaryHeap::new();
        let mut visited: HashSet<(usize, Direction)> = HashSet::new();
        queue.push(Node {
            cost: 0,
            position: start,
            direction: 1,
            parent: (start, 1),
        });
        while !queue.is_empty() {
            let next = queue.pop().unwrap();
            let next_id = (next.position, next.direction);
            if next.position == end {
                println!("part one: {}", next.cost);
                break;
            }
            if visited.contains(&next_id) {
                continue;
            }
            visited.insert(next_id);
            let dp = [-(size as i32), 1, size as i32, -1][next.direction as usize];
            let position_move = (next.position as i32 + dp) as usize;
            if cells[position_move] != b'#' {
                queue.push(Node {
                    cost: next.cost + 1,
                    position: position_move,
                    direction: next.direction,
                    parent: next_id,
                });
            }
            for dd in [3, 1] {
                queue.push(Node {
                    cost: next.cost + 1000,
                    position: next.position,
                    direction: (next.direction + dd) % 4,
                    parent: next_id,
                });
            }
        }
    }

    // Part two
    {
        // This seems like a bit of a muddle.
        // Copy the original shortest-path algorithm, but try to keep track of all best-equal-cost
        // parents for every (position, direction).
        // Also keep searching until we find every (position, direction) corresponding to the end.
        // Then, backtrack through 'parents' to find all the positions that we visit.
        let mut queue: BinaryHeap<Node> = BinaryHeap::new();
        let mut best_costs: HashMap<(usize, Direction), usize> = HashMap::new();
        let mut parents: HashMap<(usize, Direction), HashSet<(usize, Direction)>> = HashMap::new();
        let mut end_cost: Option<usize> = None;
        queue.push(Node {
            cost: 0,
            position: start,
            direction: 1,
            parent: (start, 1),
        });
        while !queue.is_empty() {
            let next = queue.pop().unwrap();
            let next_id = (next.position, next.direction);
            if next.cost > end_cost.unwrap_or(usize::MAX) {
                break;
            }
            if next.position == end {
                end_cost = Some(next.cost);
            }
            let best_cost = best_costs.get(&next_id);
            if best_cost.is_none() || *best_cost.unwrap() == next.cost {
                parents
                    .entry(next_id)
                    .or_insert(HashSet::new())
                    .insert(next.parent);
            }
            if best_cost.is_some() {
                continue;
            }
            best_costs.insert(next_id, next.cost);
            let dp = [-(size as i32), 1, size as i32, -1][next.direction as usize];
            let position_move = (next.position as i32 + dp) as usize;
            if cells[position_move] != b'#' {
                queue.push(Node {
                    cost: next.cost + 1,
                    position: position_move,
                    direction: next.direction,
                    parent: next_id,
                });
            }
            for dd in [3, 1] {
                queue.push(Node {
                    cost: next.cost + 1000,
                    position: next.position,
                    direction: (next.direction + dd) % 4,
                    parent: next_id,
                });
            }
        }
        // Backtrack to find the visited positions
        let mut backqueue: Vec<(usize, Direction)> = Vec::new();
        for direction in 0..4 {
            match parents.get(&(end, direction)) {
                Some(ps) => {
                    for p in ps {
                        backqueue.push(*p);
                    }
                }
                None => {}
            }
        }
        let mut visited: HashSet<(usize, Direction)> = HashSet::new();
        let mut index = 0;
        let empty = HashSet::new();
        while index < backqueue.len() {
            let node = backqueue[index];
            if !visited.contains(&node) {
                visited.insert(node);
                for parent in parents.get(&node).unwrap_or(&empty) {
                    backqueue.push(*parent);
                }
            }
            index += 1;
        }
        let visited_index: HashSet<usize> = visited.iter().map(|(position, _)| *position).collect();
        println!("part two: {}", 1 + visited_index.len()); // +1 for the end node
    }
}
