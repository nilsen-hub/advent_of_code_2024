use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    time::Instant,
};
#[derive(Clone, Debug)]
struct Field {
    field: Vec<Vec<char>>,
    coords: HashMap<char, Vec<Coord>>,
}
#[derive(Clone, Debug)]
struct Coord {
    x: i32,
    y: i32,
}
fn main() {
    let now = Instant::now();
    let path = "./data/data";
    let full_data = get_list_from_file(path);
    let answer = babbage(full_data);
    println!("The answer is: {}", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(full_data: Vec<String>) -> usize {
    let mut acc: HashSet<(i32, i32)> = HashSet::new();
    let field = parse_field(full_data);

    let x_bounds: std::ops::Range<i32> = 0..(field.field[0].len()) as i32;
    let y_bounds: std::ops::Range<i32> = 0..(field.field.len()) as i32;
    //println!("Xbounds: {:?} Ybounds: {:?}", x_bounds, y_bounds);
    for (_key, coords) in field.coords {
        let bounds = coords.len();
        for (index, coord) in coords.iter().enumerate() {
            let mut count = 0;
            loop {
                if count == bounds {
                    break;
                }
                if count == index {
                    count += 1;
                    continue;
                }
                let x_offset = coord.x - coords[count].x;
                let y_offset = coord.y - coords[count].y;
                let antinode_x = coord.x + x_offset;
                let antinode_y = coord.y + y_offset;

                if x_bounds.contains(&(&antinode_x)) && y_bounds.contains(&(&antinode_y)) {
                    acc.insert((antinode_x, antinode_y));
                }

                count += 1;
            }
        }
    }
    acc.len()
}
fn parse_field(full_data: Vec<String>) -> Field {
    let mut field: Vec<Vec<char>> = Vec::with_capacity(100);
    let mut coords: HashMap<char, Vec<Coord>> = HashMap::new();
    for line in full_data {
        let line_c: Vec<char> = line.chars().collect();
        field.push(line_c);
    }
    for (idy, line) in field.iter().enumerate() {
        for (idx, c) in line.iter().enumerate() {
            if c != &'.' {
                let coord = Coord {
                    x: idx as i32,
                    y: idy as i32,
                };
                if !coords.contains_key(c) {
                    let vector: Vec<Coord> = vec![coord];
                    coords.insert(*c, vector);
                } else {
                    let mut vector: Vec<Coord> = coords.get(c).unwrap().to_vec();
                    vector.push(coord);
                    coords.insert(*c, vector);
                }
            }
        }
    }

    let output = Field { field, coords };
    return output;
}
fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_example() {}
}
