use std::{collections::VecDeque, fs::read_to_string, ops::Index, time::Instant, usize};

#[derive(Debug, Clone)]
struct HardDrive {
    files: VecDeque<File>,
    gaps: VecDeque<Gap>,
}

impl HardDrive {
    fn compress(&mut self) {
        let mut compressed_files: VecDeque<File> = VecDeque::with_capacity(5000);
        let mut files = self.files.clone();
        let mut gaps = self.gaps.clone();
        files.make_contiguous().reverse();
        'outer: for mut file in files{
            let mut gap_index: usize = usize::MAX;
            for (index, gap) in gaps.make_contiguous().iter().enumerate(){
                if file.start_index <= gap.start_index{
                    compressed_files.push_front(file);
                    continue 'outer;
                }
                if file.size <= gap.size{
                    gap_index = index;
                    break;
                }
            }
            if file.size == gaps[gap_index].size{
                file.start_index = gaps[gap_index].start_index;
                compressed_files.push_front(file);
                gaps.remove(gap_index);
            } else {
                file.start_index = gaps[gap_index].start_index;
                gaps[gap_index].start_index += file.size;
                gaps[gap_index].size -= file.size;
                compressed_files.push_front(file);
            }
        }
        self.files = compressed_files;
    }
}

#[derive(Debug, Clone, Copy)]
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
    let path = "./data/data";
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

    for file in hdd.files {
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
        gap.start_index = files.back().unwrap().start_index + files.back().unwrap().size;
        gaps.push_back(gap);
        count += 1;
        if count == bounds {
            break;
        }
        let mut file = build_file(count / 2, string[count]);
        file.start_index = gaps.back().unwrap().start_index + gaps.back().unwrap().size;
        files.push_back(file);
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
