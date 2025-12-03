use std::fs;

fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Fatal error: {}", e);
            String::new()
        }
    }
}

fn find_n_max_number(line: &str, n: usize) -> i64 {
    let digits: Vec<char> = line.chars().collect();

    if digits.len() < n || n == 0 {
        return 0;
    }

    let mut result = Vec::new();
    let mut pos = 0;

    while result.len() < n {
        let need = n - result.len();
        let can_skip = digits.len() - pos - need;
        let search_end = pos + can_skip;

        let mut best_idx = pos;
        for j in pos..=search_end {
            if digits[j] > digits[best_idx] {
                best_idx = j;
            }
        }

        result.push(digits[best_idx]);
        pos = best_idx + 1;
    }

    result
        .iter()
        .collect::<String>()
        .parse::<i64>()
        .unwrap_or(0)
}

fn first_half(data: String) -> i64 {
    let mut bat_sum: i64 = 0;
    for line in data.lines() {
        bat_sum += find_n_max_number(line.trim(), 2);
    }
    bat_sum
}

fn second_half(data: String) -> i64 {
    let mut bat_sum: i64 = 0;
    for line in data.lines() {
        bat_sum += find_n_max_number(line.trim(), 12);
    }
    bat_sum
}

pub fn run() {
    let test = String::from(
        "987654321111111
        811111111111119
        234234234234278
        818181911112111",
    );

    let total_joltage = first_half(test.clone());
    println!("Part 1 (test): {}", total_joltage);
    assert!(total_joltage == 357);

    let data = read_file("2025/day3/day3.txt");
    let total_joltage = first_half(data.clone());
    println!("2025(Day3): First Half (total_joltage): {}", total_joltage);

    let total_joltage_p2 = second_half(test.clone());
    println!("Part 2 (test): {}", total_joltage_p2);
    assert!(total_joltage_p2 == 3121910778619);

    let total_joltage_p2 = second_half(data.clone());
    println!(
        "2025(Day3): Second Half (total_joltage): {}",
        total_joltage_p2
    );
}
