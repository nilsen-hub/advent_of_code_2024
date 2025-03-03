use std::{collections::HashMap, fs::read_to_string, time::Instant};

fn main() {
    let path = "./data/data";
    let full_data = get_list_from_file(path);
    let now = Instant::now();
    let answer = babbage(full_data);
    println!("The answer is: {}", answer);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn babbage(full_data: Vec<String>) -> usize {
    let mut acc: usize = 0;
    let (page_map, orders) = parser(full_data);
    for order in orders {
        if !valid_order(&page_map, &order) {
            let mut new_order = order.clone();
            loop {
                new_order = make_valid(&page_map, &new_order);
                if valid_order(&page_map, &new_order) {
                    acc += new_order[order.len() / 2];
                    break;
                }
            }
        }
    }
    acc
}
fn make_valid(page_map: &HashMap<usize, Vec<usize>>, order: &Vec<usize>) -> Vec<usize> {
    let mut order = order.clone();
    let bound: usize = order.len();
    let mut order_counter: usize = 0;
    loop {
        if order_counter == bound {
            return order;
        }
        let page = order[order_counter];
        let map = match page_map.get(&page) {
            Some(out) => out,
            None => {
                order_counter += 1;
                continue;
            }
        };
        let mut count = order_counter + 1;
        while count < bound {
            if map.contains(&order[count]) {
                let swap = order[order_counter];
                order[order_counter] = order[count];
                order[count] = swap;
                return order;
            }
            count += 1;
        }
        order_counter += 1;
    }
}
fn valid_order(page_map: &HashMap<usize, Vec<usize>>, order: &Vec<usize>) -> bool {
    for (index, page) in order.iter().enumerate() {
        let map = match page_map.get(page) {
            Some(out) => out,
            None => continue,
        };
        let mut count = index;
        while count < order.len() {
            if map.contains(&order[count]) {
                return false;
            }
            count += 1;
        }
    }
    true
}
fn parser(full_data: Vec<String>) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut page_map: HashMap<usize, Vec<usize>> = HashMap::with_capacity(2048);
    let mut update_orders: Vec<Vec<usize>> = Vec::with_capacity(256);
    for line in full_data {
        if line.contains("|") {
            let page: Vec<&str> = line.split('|').collect();
            let left: usize = page[0].parse().unwrap();
            let right: usize = page[1].parse().unwrap();
            let mut records: Vec<usize> = match page_map.get(&right) {
                Some(rec) => rec.to_vec(),
                None => Vec::new(),
            };
            records.push(left);
            page_map.insert(right, records);
            continue;
        }
        if line.contains(',') {
            let mut orders: Vec<usize> = Vec::with_capacity(16);
            let orders_string: Vec<&str> = line.split(',').collect();
            for el in orders_string {
                orders.push(el.parse().unwrap());
            }
            update_orders.push(orders);
        }
    }
    (page_map, update_orders)
}
fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
