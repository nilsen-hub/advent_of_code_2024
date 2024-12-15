use std::{fs::read_to_string, time::Instant};

fn main() {
    let now = Instant::now();
    let path = "./data/data";
    let full_data = get_list_from_file(path);
    let answer = babbage(full_data);
    println!("The answer is: {}", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(full_data: Vec<String>) -> usize{
    let mut acc: usize = 0;
    let data: Vec<Vec<char>> = parse(full_data);
    for stone in data{
        let mut stone: Vec<Vec<char>> = vec![stone];
        let mut meta_stones: Vec<Vec<Vec<char>>> = Vec::new();
        let mut stones: Vec<Vec<char>> = Vec::new();
        let mut first = blink_machine(stone);
        'outer: loop{
            let mut count = 1000;
            while count >= 0 {
                
            }
        }
    }
    acc
}
fn blink_machine(data: Vec<Vec<char>>) -> Vec<Vec<char>>{
    let mut data = data;
    let mut limit: usize = 25;
    while limit > 0 {
        limit -= 1;
        let mut temp_vec: Vec<Vec<char>> = Vec::new();
        for mut stone in data{
            if stone.len() == 1 && stone[0] == '0'{
                stone[0] = '1';
                temp_vec.push(stone);
                continue;
            } else if stone.len() % 2 != 0{
                let mut as_string:String = stone.into_iter().collect();
                let mut as_num: usize = as_string.parse().unwrap();
                as_num *= 2024;
                as_string = as_num.to_string();
                stone = as_string.chars().collect();
                temp_vec.push(stone);
                continue;
            } else {
                let mut second:Vec<char> = stone.drain((stone.len()/2)..).collect();
                let mut zero_vec: Vec<usize> = Vec::new();
                for (index, el) in second.iter().enumerate(){
                    if *el == '0'{
                        zero_vec.push(index);
                    }else{
                        break;
                    }
                }
                if zero_vec.len() == second.len(){
                    second = vec!['0'];
                } else {
                    for _el in zero_vec{
                        second.remove(0);
                    }
                }

                let stones: Vec<Vec<char>> = vec![stone, second];
                for stone in stones{
                    temp_vec.push(stone);
                }
            } 

        }
        data = temp_vec;
    }
    data
}
//fn blink_machine_deprec(blink_limit: usize, stone: Vec<char>, acc: usize) -> usize{
//    let mut acc = acc;
//    if blink_limit == 0{
//        acc += 1;
//        return acc
//    } else {
//        let blink_limit = blink_limit - 1;
//        let mut stone = stone;
//        
//        if stone.len() == 1 && stone[0] == '0'{
//            println!(" 00 {:?} should trigger here - 00", stone);
//            stone[0] = '1';
//            acc += blink_machine(blink_limit, stone, acc);
//            return acc
//        } else if stone.len() % 2 != 0{
//            println!(" !%2 {:?} should trigger here - !%2", stone);
//            let mut as_string:String = stone.into_iter().collect();
//            let mut as_num: usize = as_string.parse().unwrap();
//            as_num *= 2024;
//            as_string = as_num.to_string();
//            stone = as_string.chars().collect();
//            acc +=  blink_machine(blink_limit, stone, acc);
//            return acc
//        } else {
//            println!(" %2 should trigger here: {:?} %2", stone);
//            let mut second:Vec<char> = stone.drain((stone.len()/2)..).collect();
//            let mut zero_vec: Vec<usize> = Vec::new();
//            for (index, el) in second.iter().enumerate(){
//                if *el == '0'{
//                    zero_vec.push(index);
//                }else{
//                    break;
//                }
//            }
//            for _el in zero_vec{
//                second.remove(0);
//            }
//            let stones: Vec<Vec<char>> = vec![stone, second];
//
//            for stone in stones{
//                acc += blink_machine(blink_limit, stone, acc);
//                return acc
//            }
//        }  
//    }
//    println!(" its here");
//    acc
//}
fn parse(full_data: Vec<String>) -> Vec<Vec<char>>{
    let mut output: Vec<Vec<char>> = Vec::new();
    let split:Vec<&str> = full_data[0].split_whitespace().collect();
    for el in split{
        let chars: Vec<char> = el.chars().collect();
        output.push(chars); 
    }

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
