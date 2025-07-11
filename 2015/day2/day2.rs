use std::{cmp, fs};

fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Fatal error: {}", e);
            String::new()
        }
    }
}

fn first_half(data: String) -> i32 {
    let mut total_paper = 0;

    for line in data.lines() {
        let dims: Vec<_> = line.split('x').map(|c| c.parse::<i32>().unwrap()).collect();

        if dims.len() == 3 {
            let (l, w, h) = (dims[0], dims[1], dims[2]);
            let side1 = l * w;
            let side2 = w * h;
            let side3 = h * l;
            let min_side = cmp::min(side1, cmp::min(side2, side3));

            total_paper += 2 * (side1 + side2 + side3) + min_side;
        } else {
            eprintln!("Skipping invalid line: {}", line);
        }
    }

    total_paper
}

fn second_half(data: String) -> i32 {
    let mut total_ribbon = 0;

    for line in data.lines() {
        let dims: Vec<_> = line.split('x').map(|c| c.parse::<i32>().unwrap()).collect();

        if dims.len() == 3 {
            let (l, w, h) = (dims[0], dims[1], dims[2]);
            let min_perimeter = cmp::min(l + w, cmp::min(w + h, l + h));
            let vol = l * w * h;

            total_ribbon += 2 * min_perimeter + vol;
        } else {
            eprintln!("Skipping invalid line: {}", line);
        }
    }

    total_ribbon
}

pub fn run() {
    let data = read_file("2015/day2/day2.txt");
    let total_paper = first_half(data.clone());
    let total_ribbon = second_half(data.clone());
    println!("First Half (total_paper): {}", total_paper);
    println!("Second Half (total_ribbon): {}", total_ribbon);
}
