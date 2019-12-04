pub fn aoc_4() {
    let start = 240_298;
    let end = 784_956;
    println!("number of passwords = {}", count_pwds(start, end));
    println!("number of passwords = {}", count_pwds2(start, end));
}

fn count_pwds(start: u32, end: u32) -> u32 {
    let mut count = 0;
    for pwd in start..end {
        let pwd_s = format!("{}", pwd);

        if is_valid(&pwd_s) {
            count += 1;
        }
    }
    count
}

fn count_pwds2(start: u32, end: u32) -> u32 {
    let mut count = 0;
    for pwd in start..end {
        let pwd_s = format!("{}", pwd);

        if is_valid2(&pwd_s) {
            count += 1;
        }
    }
    count
}

fn is_valid(pwd_s: &str) -> bool {
    has_double(pwd_s) && not_decreasing(pwd_s)
}

fn is_valid2(pwd_s: &str) -> bool {
    has_double_without_triple(pwd_s) && not_decreasing(pwd_s)
}

fn has_double(pwd: &str) -> bool {
    let pwd: Vec<u32> = pwd.chars().map(|c| c.to_digit(10).unwrap()).collect();
    for i in 1..pwd.len() {
        if pwd[i] == pwd[i - 1] {
            return true;
        }
    }
    false
}

fn has_double_without_triple(pwd: &str) -> bool {
    let pwd: Vec<u32> = pwd.chars().map(|c| c.to_digit(10).unwrap()).collect();
    for i in 1..pwd.len() {
        if pwd[i] == pwd[i - 1]
            && ((i == 1 || pwd[i] != pwd[i - 2]) && (i == pwd.len() - 1 || pwd[i] != pwd[i + 1]))
        {
            return true;
        }
    }
    false
}

fn not_decreasing(pwd: &str) -> bool {
    let pwd: Vec<u32> = pwd.chars().map(|c| c.to_digit(10).unwrap()).collect();
    for i in 1..pwd.len() {
        if pwd[i] < pwd[i - 1] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_double() {
        assert!(has_double("111111"));
        assert!(has_double("224373"));
        assert!(!has_double("015203"));
        assert!(has_double("015233"));
    }

    #[test]
    fn test_decreating() {
        assert!(not_decreasing("111111"));
        assert!(not_decreasing("224579"));
        assert!(!not_decreasing("015203"));
        assert!(!not_decreasing("015233"));
    }

    #[test]
    fn test_valid() {
        assert!(is_valid("111111"));
        assert!(!is_valid("223450"));
        assert!(!is_valid("123789"));
    }

    #[test]
    fn test_has_double_without_triple() {
        assert!(!has_double_without_triple("111111"));
        assert!(has_double_without_triple("224373"));
        assert!(!has_double_without_triple("015203"));
        assert!(!has_double_without_triple("015333"));
        assert!(!has_double_without_triple("012223"));
    }
}
