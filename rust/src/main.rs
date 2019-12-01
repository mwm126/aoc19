use std::io::BufRead;

fn main() {
    let input = std::fs::File::open("input-1.txt").unwrap();
    let fuel_total: i32 = std::io::BufReader::new(input)
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .map(fuel)
        .sum();
    println!("Total fuel requirement: {}", fuel_total);
}

fn fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

#[cfg(test)]
mod tests {
    use super::fuel;

    #[test]
    fn it_works() {
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100_756), 33583);
    }
}
