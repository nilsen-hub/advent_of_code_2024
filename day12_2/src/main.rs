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
        let mut coords = coords;
        let mut plots: Vec<Plot> = Vec::new();
        let mut checked_coords: HashSet<Coords> = HashSet::new();
        let mut to_visit: VecDeque<Coords> = VecDeque::new();

        // make loop to build an register all connected plots of current plant
        'outer: loop {
            //first build plot struct
            let mut plot = self.get_plot(coords);
            let (x, y) = coords;

            let directions: Vec<Coords> = vec![
                (x, y - 1), // north
                (x, y + 1), // south
                (x + 1, y), // east
                (x - 1, y), // west
            ];

            for (index, direction) in directions.iter().enumerate() {
                if self.field[direction.1][direction.0] == plant {
                    to_visit.push_back(*direction);
                } else {
                    plot.fence_count += 1;
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
    fn get_plot(&self, coords: Coords) -> Plot {
        let (idx, idy) = coords;
        let plot = Plot {
            idx,
            idy,
            fence_count: 0,
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
    coords: HashSet<Coords>,
}
impl Region {
    fn side_counter(&self) -> usize {
        let mut sides: usize = 0;
        let mut fenced_plots: Vec<Plot> = Vec::with_capacity(100);
        let plots = self.plots.clone();
        for plot in plots {
            if plot.fence_count > 0 {
                fenced_plots.push(plot);
            }
        }
        // count north edges
        let mut direction_counter: usize = 0;
        loop {
            sides += self.get_sides(&fenced_plots, direction_counter);
            if direction_counter == 3 {
                break;
            }
            direction_counter += 1;
        }
        //if sides &1 == 1{
        //    sides -= 1;
        //}
        sides
    }
    fn get_sides(&self, plots: &Vec<Plot>, direction: usize) -> usize {
        // directions by index: North, South, East, West
        let plots = plots.clone();
        let mut directed_plots: Vec<Plot> = Vec::with_capacity(50);
        let mut side_counter: usize = 0;

        for plot in plots{
            if plot.fences[direction]{
                directed_plots.push(plot);
            }
        }

        if direction == 0 || direction == 1 {
            // looking for north or south faceing sides
            // sorting by Y axis, gives us the lines
            directed_plots.sort_by_key(|plot| plot.idy);

            for (index, plot) in directed_plots.iter().enumerate(){
                if index == 0{
                    side_counter += 1;
                    continue;
                }
                if plot.idy == directed_plots[index - 1].idy{
                    if plot.idx == directed_plots[index - 1].idx + 1 || plot.idx == directed_plots[index - 1].idx - 1{
                        continue;
                    }
                }
                side_counter += 1;
            }
            
        } else {
            directed_plots.sort_by_key(|plot| plot.idx);
            //if direction == 2{
            //    for plot in &directed_plots{
            //        println!("{:?}", plot);
            //    }
            //}
            for (index, plot) in directed_plots.iter().enumerate(){
                if index == 0{
                    //if direction == 2{
                    //    println!("{:?}", plot);
                    //}
                    side_counter += 1;
                    continue;
                }
                //if plot.idx == directed_plots[index - 1].idx && plot.idy == directed_plots[index - 1].idy + 1{
                //    continue;
                //}
                if plot.idx == directed_plots[index - 1].idx{
                    if plot.idy == directed_plots[index - 1].idy + 1 || plot.idy == directed_plots[index - 1].idy - 1{
                        continue;
                    }
                }
                //if direction == 2{
                //    println!("{:?}", plot);
                //}
                
                side_counter += 1;
            }
        }
        //println!("direction: {direction} sides: {side_counter}");
        side_counter
    }
}
#[derive(Debug, Clone)]
struct Plot {
    idx: usize,
    idy: usize,
    fence_count: usize,
    fences: Vec<bool>, // by index: North, South, East, West
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
        let side_count = region.side_counter();
        println!(
            "Region area: {} | kind: {} | amount of sides {}",
            region.area,
            region.plant,
            side_count,
        );
        acc += side_count * region.area;
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
