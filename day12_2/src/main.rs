use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
    time::Instant,
};
#[derive(Debug, Clone)]
struct Farm {
    regions: Vec<Region>,
    field: field,
    mapped: HashSet<coords>,
}

impl Farm {
    fn walker(&mut self) {
        for (idy, line) in self.field.iter().enumerate() {
            for (idx, plot) in line.iter().enumerate() {
                if *plot == '+' {
                    continue;
                }
                let coords: coords = (idx, idy);
                if self.mapped.contains(&coords) {
                    continue;
                }
                let region = self.map_region(coords);
                self.regions.push(region.clone());
                self.mapped.extend(region.coords);
            }
        }
    }
    fn map_region(&self, coords: coords) -> Region {
        // set up region content variables
        let (x, y) = coords;
        let plant: char = self.field[y][x];
        let mut coords = coords;
        let mut plots: Vec<Plot> = Vec::new();
        let mut checked_coords: HashSet<coords> = HashSet::new();
        let mut to_visit: VecDeque<coords> = VecDeque::new();

        // make loop to build an register all connected plots of current plant
        'outer: loop {
            //first build plot struct
            let mut plot = self.get_plot(coords);
            let (x, y) = coords;

            let directions: Vec<coords> = vec![
                (x - 1, y), // north
                (x + 1, y), // south
                (x, y + 1), // east
                (x, y - 1), // west
            ];

            for (index, direction) in directions.iter().enumerate() {
                if self.field[direction.1][direction.0] == plant {
                    to_visit.push_back(*direction);
                } else {
                    match index {
                        0 => plot.fences[index] = true,
                        1 => plot.fences[index] = true,
                        2 => plot.fences[index] = true,
                        3 => plot.fences[index] = true,
                        _ => panic!("Something in map_region really messed up"),
                    }
                }
            }
            checked_coords.insert(coords);
            plots.push(plot);
            if to_visit.len() == 0 {
                break;
            }
            loop {
                coords = to_visit.pop_front().unwrap();
                if !checked_coords.contains(&coords) {
                    break;
                }
                if to_visit.len() == 0 {
                    break 'outer;
                }
            }
        }

        let area = &plots.len();
        let region = Region {
            plant,
            plots,
            area: *area,
            coords: checked_coords,
        };
        region
    }
    fn get_plot(&self, coords: coords) -> Plot {
        let (idx, idy) = coords;
        let plot = Plot {
            idx,
            idy,
            fences: vec![false; 4],
        };
        plot
    }
}
#[derive(Debug, Clone)]
struct Region {
    plant: char,
    plots: Vec<Plot>,
    area: usize,
    coords: HashSet<coords>,
}
impl region {
    fn side_counter(&self) -> usize {
        let mut sides: usize = 0;
        sides
    }
}
#[derive(Debug, Clone)]
struct Plot {
    idx: usize,
    idy: usize,
    fences: Vec<bool>, // by index: North, South, East, West
}

type field = Vec<Vec<char>>;
type coords = (usize, usize);

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
    acc
}
fn get_farm(field: field) -> Farm {
    let farm = Farm {
        regions: Vec::new(),
        field,
        mapped: HashSet::new(),
    };
    farm
}

fn parse(data: Vec<String>) -> field {
    // padding the field with + to avoid edges
    let padding: Vec<char> = vec!['+'; data[0].len() + 2];
    let mut output: field = field::with_capacity(142);
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
