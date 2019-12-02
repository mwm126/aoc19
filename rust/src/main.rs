use std::io::BufRead;

fn main() {
    let input = std::fs::File::open("../input-1.txt").unwrap();
    let masses: Vec<i32> = std::io::BufReader::new(input)
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
    let fuel_total: i32 = masses.clone().into_iter().map(fuel).sum();
    println!("Total fuel requirement: {}", fuel_total);

    let fuel_total2: i32 = masses.into_iter().map(fuel2).sum();
    println!("Total fuel requirement: {}", fuel_total2);
}

fn fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn fuel2(mass: i32) -> i32 {
    let f = fuel(mass);
    if f <= 0 {
        0
    } else {
        f + fuel2(f)
    }
}

#[cfg(test)]
mod tests {
    use super::fuel;
    use super::fuel2;

    #[test]
    fn just_cargo() {
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100_756), 33583);
    }

    #[test]
    fn cargo_plus_fuel() {
        assert_eq!(fuel2(12), 2);
        assert_eq!(fuel2(14), 2);
        assert_eq!(fuel2(1969), 966);
        assert_eq!(fuel2(100_756), 50346);
    }
}
