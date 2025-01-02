use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
    time::Instant,
};
#[derive(Debug, Clone)]
struct Farm {
    regions: Vec<Region>,
    field: Field,
    mapped: HashSet<Coords>,
}

impl Farm {
    fn walker(&mut self) {
        for (idy, line) in self.field.iter().enumerate() {
            for (idx, plot) in line.iter().enumerate() {
                if *plot == '+' {
                    continue;
                }
                let coords: Coords = (idx, idy);
                if self.mapped.contains(&coords) {
                    continue;
                }
                let region = self.map_region(coords);
                self.regions.push(region.clone());
                self.mapped.extend(region.coords);
            }
        }
    }
    fn map_region(&self, coords: Coords) -> Region {
        // set up region content variables
        let (x, y) = coords;
        let plant: char = self.field[y][x];
        let mut perimeter: usize = 0;
        let mut coords = coords;
        let mut checked_coords: HashSet<Coords> = HashSet::new();
        let mut to_visit: VecDeque<Coords> = VecDeque::new();

        'outer: loop {
            //first build plot struct
            let (x, y) = coords;

            let directions: Vec<Coords> = vec![
                (x - 1, y), // north
                (x + 1, y), // south
                (x, y + 1), // east
                (x, y - 1), // west
            ];

            for direction in directions {
                if self.field[direction.1][direction.0] == plant {
                    to_visit.push_back(direction);
                } else {
                    perimeter += 1;
                }
            }

            checked_coords.insert(coords);

            loop {
                if to_visit.len() == 0 {
                    break 'outer;
                }
                coords = to_visit.pop_front().unwrap();
                if !checked_coords.contains(&coords) {
                    break;
                }
            }
        }

        let region = Region {
            area: checked_coords.len(),
            perimeter,
            coords: checked_coords,
        };

        region
    }
}
#[derive(Debug, Clone)]
struct Region {
    area: usize,
    perimeter: usize,
    coords: HashSet<Coords>,
}
type Field = Vec<Vec<char>>;
type Coords = (usize, usize);

fn main() {
    let now = Instant::now();
    let path = "./data/data";
    let full_data = get_list_from_file(path);
    let answer = babbage(full_data);
    println!("The answer is: {}", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(full_data: Vec<String>) -> usize {
    let mut acc = 0;
    let field = parse(full_data);
    let mut farm = get_farm(field);
    farm.walker();
    for region in farm.regions {
        acc += region.area * region.perimeter;
    }
    acc
}
fn get_farm(field: Field) -> Farm {
    let farm = Farm {
        regions: Vec::new(),
        field,
        mapped: HashSet::new(),
    };
    farm
}
fn parse(data: Vec<String>) -> Field {
    // padding the field with + to avoid edges
    let padding: Vec<char> = vec!['+'; data[0].len() + 2];
    let mut output: Field = Field::with_capacity(142);
    output.push(padding.clone());
    for line in data {
        let output_line = format!("{}{}{}", '+', line, '+');
        output.push(output_line.chars().collect());
    }
    output.push(padding);
    output
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
