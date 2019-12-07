use std::collections::HashMap;
use std::io::BufRead;

pub fn aoc_6() {
    let input = std::fs::File::open("../input-6.txt").unwrap();
    let lines: Vec<String> = std::io::BufReader::new(input)
        .lines()
        .map(|s| s.unwrap())
        .collect();
    let links = parse_input(lines);
    let dag = make_dag(links);

    let answer = total_orb_count(&dag);
    println!("Total number of direct and indirect orbits: {}", answer);
    let answer = you_to_san(&dag);
    println!("Orbital transfer from YOU to SAN: {}", answer);
}

fn parse_input(lines: Vec<String>) -> Vec<Vec<String>> {
    lines
        .iter()
        .map(|line| line.split(')').take(2).map(|s| s.to_owned()).collect())
        .collect()
}

fn make_dag(links: Vec<Vec<String>>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for link in links {
        map.insert(link[1].clone(), link[0].clone());
    }
    map
}

fn route_to_com(dag: &HashMap<String, String>, orb: &str) -> Vec<String> {
    let mut route: Vec<String> = vec![];
    let mut current = orb.to_owned();
    while current != "COM" {
        current = dag[&current].clone();
        route.push(current.clone());
    }
    route
}

fn count_orbs(dag: &HashMap<String, String>, orb: &str) -> usize {
    let route = route_to_com(dag, orb);
    route.len()
}

fn total_orb_count(dag: &HashMap<String, String>) -> usize {
    dag.iter().map(|orb| count_orbs(&dag, orb.0)).sum()
}

fn you_to_san(dag: &HashMap<String, String>) -> usize {
    let mut you_route = route_to_com(dag, "YOU");
    let mut san_route = route_to_com(dag, "SAN");
    you_route.reverse();
    san_route.reverse();
    let mut ii = 0;
    while you_route[ii] == san_route[ii] {
        ii += 1;
    }
    (you_route.len() - ii) + (san_route.len() - ii)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbit_checksum() {
        let input: Vec<String> = vec![
            "COM)B".to_owned(),
            "B)C".to_owned(),
            "C)D".to_owned(),
            "D)E".to_owned(),
            "E)F".to_owned(),
            "B)G".to_owned(),
            "G)H".to_owned(),
            "D)I".to_owned(),
            "E)J".to_owned(),
            "J)K".to_owned(),
            "K)L".to_owned(),
        ];
        let links = parse_input(input);
        let dag = make_dag(links);
        assert_eq!(count_orbs(&dag, "D"), 3);
        assert_eq!(count_orbs(&dag, "L"), 7);
        assert_eq!(count_orbs(&dag, "COM"), 0);
        assert_eq!(total_orb_count(&dag), 42);
    }

    #[test]
    fn test_santa_map() {
        let input: Vec<String> = vec![
            "COM)B".to_owned(),
            "B)C".to_owned(),
            "C)D".to_owned(),
            "D)E".to_owned(),
            "E)F".to_owned(),
            "B)G".to_owned(),
            "G)H".to_owned(),
            "D)I".to_owned(),
            "E)J".to_owned(),
            "J)K".to_owned(),
            "K)L".to_owned(),
            "K)YOU".to_owned(),
            "I)SAN".to_owned(),
        ];
        let links = parse_input(input);
        let dag = make_dag(links);
        assert_eq!(you_to_san(&dag), 4);
    }
}
