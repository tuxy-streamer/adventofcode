use std::{self, fs};

fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Fatal error: {}", e);
            String::new()
        }
    }
}

fn parse_coords(coord: &str) -> (usize, usize) {
    let (x, y) = coord.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn first_half(data: String) -> i32 {
    let mut light_grid: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];

    for line in data.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let (command, start, end) = if words[0] == "turn" {
            (words[1], words[2], words[4])
        } else if words[0] == "toggle" {
            ("toggle", words[1], words[3])
        } else {
            continue;
        };
        let (row_start, col_start) = parse_coords(start);
        let (row_end, col_end) = parse_coords(end);
        for row in row_start..=row_end {
            for col in col_start..=col_end {
                match command {
                    "on" => light_grid[row][col] = true,
                    "off" => light_grid[row][col] = false,
                    "toggle" => light_grid[row][col] = !light_grid[row][col],
                    _ => {}
                }
            }
        }
    }

    light_grid.iter().flatten().filter(|&&state| state).count() as i32
}

fn second_half(data: String) -> i32 {
    let mut light_grid: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];

    for line in data.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let (command, start, end) = if words[0] == "turn" {
            (words[1], words[2], words[4])
        } else if words[0] == "toggle" {
            ("toggle", words[1], words[3])
        } else {
            continue;
        };
        let (row_start, col_start) = parse_coords(start);
        let (row_end, col_end) = parse_coords(end);
        for row in row_start..=row_end {
            for col in col_start..=col_end {
                match command {
                    "on" => light_grid[row][col] += 1,
                    "off" => light_grid[row][col] = (light_grid[row][col] - 1).max(0),
                    "toggle" => light_grid[row][col] += 2,
                    _ => {}
                }
            }
        }
    }

    light_grid.iter().flatten().sum()
}
pub fn run() {
    let data = read_file("2015/day6/day6.txt");
    let no_of_on_lights: i32 = first_half(data.clone());
    let total_brightness: i32 = second_half(data.clone());
    println!(
        "2015(Day6): First Half (no_of_on_lights): {}",
        no_of_on_lights
    );
    println!(
        "2015(Day6): Second Half (total_brightness): {}",
        total_brightness
    );
}
