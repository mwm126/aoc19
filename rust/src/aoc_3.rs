use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;

pub fn aoc_3() {
    let input = std::fs::File::open("../input-3.txt").unwrap();
    let mut lines = std::io::BufReader::new(input).lines();
    let wire = lines.next().unwrap().unwrap();
    let wire2 = lines.next().unwrap().unwrap();

    println!("min distance = {}", find_distance(&wire, &wire2));
    println!("min time = {}", find_time(&wire, &wire2));
}

fn manhattan_distance(p1: (i32, i32)) -> i32 {
    p1.0.abs() + p1.1.abs()
}

fn parse_move(move_: &str) -> (char, i32) {
    let dir = move_.chars().next().unwrap();
    let dist = &move_[1..].parse::<i32>().unwrap();
    (dir, *dist)
}

fn next_point(position: (i32, i32), move_: char) -> (i32, i32) {
    match move_ {
        'U' => (position.0, position.1 + 1),
        'D' => (position.0, position.1 - 1),
        'R' => (position.0 + 1, position.1),
        'L' => (position.0 - 1, position.1),
        _ => panic!("bad input"),
    }
}

fn points(wire: &str) -> HashSet<(i32, i32)> {
    let mut set = HashSet::new();
    let mut curr_pos = (0, 0);
    for move_ in wire.split(',') {
        let (dir, dist) = parse_move(move_);
        for _ in 0..dist {
            curr_pos = next_point(curr_pos, dir);
            set.insert(curr_pos);
        }
    }
    set
}

fn find_distance(wire: &str, wire2: &str) -> i32 {
    let point_set = points(wire);

    let mut min_dist: Option<i32> = None;
    let mut curr_pos = (0, 0);
    for move_ in wire2.split(',') {
        let (dir, dist) = parse_move(move_);
        for _ in 0..dist {
            curr_pos = next_point(curr_pos, dir);
            if point_set.contains(&curr_pos) {
                let curr_dist = manhattan_distance(curr_pos);
                min_dist = match min_dist {
                    None => Some(curr_dist),
                    Some(dist) => Some(std::cmp::min(dist, curr_dist)),
                };
            }
        }
    }
    min_dist.unwrap()
}

fn point_times(wire: &str) -> HashMap<(i32, i32), i32> {
    let mut map = HashMap::new();
    let mut curr_pos = (0, 0);
    let mut time = 0;
    for move_ in wire.split(',') {
        let (dir, dist) = parse_move(move_);
        for _ in 0..dist {
            time += 1;
            curr_pos = next_point(curr_pos, dir);
            map.insert(curr_pos, time);
        }
    }
    map
}

fn find_time(wire: &str, wire2: &str) -> i32 {
    let point_map = point_times(wire);

    let mut min_time: Option<i32> = None;
    let mut curr_pos = (0, 0);
    let mut curr_time = 0;
    for move_ in wire2.split(',') {
        let (dir, dist) = parse_move(move_);
        for _ in 0..dist {
            curr_time += 1;
            curr_pos = next_point(curr_pos, dir);
            if point_map.contains_key(&curr_pos) {
                let other_time = point_map.get(&curr_pos).unwrap();
                let total_time = other_time + curr_time;
                min_time = match min_time {
                    None => Some(total_time),
                    Some(time) => Some(std::cmp::min(time, total_time)),
                };
            }
        }
    }
    min_time.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_distance((3, 4)), 7);
        assert_eq!(manhattan_distance((-9, 1)), 10);
        assert_eq!(manhattan_distance((2, -4)), 6);
    }

    #[test]
    fn test_nextpoint() {
        assert_eq!(next_point((0, 0), 'R'), (1, 0));
        assert_eq!(next_point((1, 0), 'D'), (1, -1));
    }

    #[test]
    fn test_min_distance() {
        let wire = "R8,U5,L5,D3";
        let wire2 = "U7,R6,D4,L4";
        assert_eq!(find_distance(wire, wire2), 6);

        let wire = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let wire2 = "U62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(find_distance(wire, wire2), 159);

        let wire = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let wire2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(find_distance(wire, wire2), 135);
    }

    #[test]
    fn test_min_time() {
        let wire = "R8,U5,L5,D3";
        let wire2 = "U7,R6,D4,L4";
        assert_eq!(find_time(wire, wire2), 30);

        let wire = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let wire2 = "U62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(find_time(wire, wire2), 610);

        let wire = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let wire2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(find_time(wire, wire2), 410);
    }
}
