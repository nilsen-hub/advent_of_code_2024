use std::{collections::HashSet, fs::read_to_string, time::Instant};

type Field = Vec<Vec<char>>;

#[derive(Debug, Clone)]
enum Direction{
    North,
    South,
    East,
    West,
}
#[derive(Debug, Clone, Copy,)]
struct Coords{
    a: isize, 
    b: isize,
}

impl std::ops::Add<(isize, isize)> for Coords {
  type Output = Coords;

  fn add(self, rhs: (isize, isize)) -> Self::Output {
    Self::Output {
        a: self.a + rhs.0,
        b: self.b + rhs.1,
    }    
  }
}
impl std::ops::Add<Coords> for Coords {
    type Output = Coords;
  
    fn add(self, rhs: Coords) -> Self::Output {
      Self::Output {
          a: self.a + rhs.a,
          b: self.b + rhs.b,
      }    
    }
  }
impl Coords{
    const NORTH: Coords = Coords{
        a:0,
        b:-1,
    };

    const WEST: (isize, isize) = (-1, 0);
    // fn add(&self, other: Coords) -> Coords{
    //     return Coords{
    //         a: self.a + other.a, 
    //         b: self.b + other.b,
    //     }
    // }
    fn from(t0:isize, t1:isize) -> Coords{
        return Coords{
            a: t0,
            b: t1,
        };
    }
}
#[derive(Debug, Clone)]
struct InputData {
    input: String,
}
impl InputData {
    fn parse(&self) -> Solver {
        let lines = self.input.lines();
        let mut field: Field = Vec::with_capacity(150);
        for line in lines{
            field.push(line.chars().collect());
        }
        for line in &field{
            for c in line{
                print!("{c}");
            }
            println!("");
        }
        println!("");
        let mut end = Coords::from(0,0);
        
        for (idx, c) in field[1].iter().enumerate(){
            if *c == 'S'{
                end = Coords::from(idx, 1);
            }
        }
        let start = Coords::from(end.b, end.a);
        return Solver{
            maze: Maze{
                field,
                start,
                end
            },
            rudolph: Rudolph {
                position: start,
                direction: Direction::East,
                seen_positions: HashSet::with_capacity(500),
                points: 0,
            },
        };
    }
}
#[derive(Debug, Clone)]
struct Maze{
    field: Field,
    start: Coords,
    end: Coords,
}
#[derive(Debug, Clone)]
struct Rudolph{
    position: Coords,
    seen_positions: HashSet<Coords>,
    direction: Direction,
    points: usize,
}
#[derive(Debug, Clone)]
struct Solver{
    maze: Maze,
    rudolph: Rudolph,
}
impl Solver{
    fn solve(&mut self,) -> usize {
        return self.rudolph.points;
    }
    fn rudolph_turn(&mut self){

    }
    fn rudolph_move_to_next_junction(&mut self){

    }
    fn find_next_junction(&self, mut pos:Coords) -> Option<Coords>{
        
        match self.rudolph.direction{
            North=>,
            South=>,
            East=>,
            West=>,
        }
    }
}
fn main() {
    let path = "./data/test_1";
    let input = InputData {
        input: match read_to_string(path) {
            Ok(file) => file,
            Err(_) => panic!("File should be here"),
        },
    };
    babbage(input);
}
fn babbage(input: InputData){
    let now = Instant::now();
    let mut solver = input.parse();
    println!("Add tuple experiment: {:?}", tup_add);
    println!("The answer is: {}", solver.solve());
    println!("babbage runtime: {}", now.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_examples() {
    }
}
