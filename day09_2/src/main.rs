use std::{collections::VecDeque, fs::read_to_string, time::Instant};

#[derive(Debug, Clone)]
struct HardDrive {
    files: VecDeque<File>,
    gaps: VecDeque<Gap>,
}

impl HardDrive {
    fn fix_start_indices(&mut self) {
        let mut index: usize = 0;
        let bounds = self.files.len();
        loop {
            if index == 0 {
                index += 1;
                continue;
            }
            let prev = self.files[index - 1];
            self.files[index].start_index = prev.start_index + prev.size;
            index += 1;
            if index == bounds {
                break;
            }
        }
    }
    fn compress(&mut self) {
        println!("new compressor!!");
    }
    fn compress_deprec(&mut self) {
        let mut comp_files: VecDeque<File> = VecDeque::with_capacity(5000);
        let mut files = self.files.clone();
        // this makes front into back and VV
        files.make_contiguous().reverse();
        let mut current_file = File::default();
        let mut current_file_idx: usize = 0;
        let mut no_file_found = false;
        // this is where the really clunky gears start turning.
        'outer: loop {
            comp_files.push_back(match files.pop_back() {
                Some(file) => file,
                None => break,
            });
            //println!("{:?}", comp_files);
            let mut current_gap = match self.gaps.pop_front() {
                Some(gap) => gap,
                None => break,
            };
            loop {
                // try to find a file that fits the gap
                if files.len() == 0 {
                    break 'outer;
                }
                for (index, file) in files.make_contiguous().iter().enumerate() {
                    if file.size <= current_gap.size {
                        current_file_idx = index;
                        no_file_found = false;
                        break;
                    }
                    no_file_found = true;
                }
                // handle no file found case
                if no_file_found {
                    let gap_as_file = File {
                        id: 0,
                        size: current_gap.size,
                        start_index: 0,
                    };
                    comp_files.push_back(gap_as_file);
                    break;
                } else {
                    current_file = files.remove(current_file_idx).unwrap();
                }
                // if fits perfectly else a bit small
                if current_file.size == current_gap.size {
                    comp_files.push_back(current_file);
                    break;
                } else {
                    current_gap.size -= current_file.size;
                    comp_files.push_back(current_file);
                }
            }
        }
        self.files = comp_files;
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct File {
    id: usize,
    size: usize,
    start_index: usize,
}
#[derive(Debug, Clone, Copy)]
struct Gap {
    size: usize,
    start_index: usize,
}

impl File {
    fn get_checksum(&self) -> usize {
        let mut acc: usize = 0;
        let size = self.size;
        let mut count = self.start_index;
        while count < self.start_index + size {
            acc += self.id * count;
            count += 1;
        }
        acc
    }
}
fn main() {
    let now = Instant::now();
    let path = "./data/test";
    let full_data = get_list_from_file(path);
    let answer = babbage(full_data);
    println!("The answer is: {}", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(full_data: Vec<String>) -> usize {
    let mut acc = 0;
    let (files, gaps) = get_files_and_gaps(full_data);
    let mut hdd = HardDrive { files, gaps };
    hdd.compress();
    hdd.fix_start_indices();

    for file in hdd.files {
        println!("{:?}", file);
        acc += file.get_checksum();
    }

    acc
}

fn get_files_and_gaps(full_data: Vec<String>) -> (VecDeque<File>, VecDeque<Gap>) {
    let string: Vec<char> = full_data[0].chars().collect();
    let mut files: VecDeque<File> = VecDeque::with_capacity(5000);
    let mut gaps: VecDeque<Gap> = VecDeque::with_capacity(5000);
    let bounds = string.len();
    // to deal with zero-case
    files.push_back(build_file(0, string[0]));
    let mut count = 1;
    loop {
        // kinda ugly, but should work
        let mut gap = build_gap(string[count]);
        gap.size = files[files.len() - 1].start_index + files[files.len() - 1].size
        gaps.push_back(gap);
        count += 1;
        if count == bounds {
            break;
        }
        files.push_back(build_file(count / 2, string[count]));
        count += 1;
        if count == bounds {
            break;
        }
    }

    (files, gaps)
}
fn build_file(id: usize, size: char) -> File {
    let size: usize = size.to_digit(10).unwrap() as usize;
    File {
        id,
        size,
        start_index: 0,
    }
}
fn build_gap(size: char) -> Gap {
    let size: usize = size.to_digit(10).unwrap() as usize;
    Gap {
        size,
        start_index: 0,
    }
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
