use std::collections::HashMap;

fn main() {
    let int_vec = vec![7, 3, 10, 27, 14, 18, 3, 31, 9, 12];

    let (mean, median, mode) = calc_stats(&mut int_vec.clone());
    println!("Mean: {}\nMedian: {}\nMode: {}", mean, median, mode);
}

fn calc_stats(int_vec: &mut Vec<i32>) -> (f64, f64, i32) {
    int_vec.sort();

    let mut mean = 0;
    let median;

    let mut map: HashMap<i32, i32> = HashMap::new();

    let length = int_vec.len();

    if length % 2 == 0 {
        median = (int_vec[length / 2 - 1] + int_vec[length / 2]) as f64 / 2.0;
    }
    else {
        median = int_vec[length / 2] as f64;
    }

    for int in int_vec {
        let entry = map.entry(*int).or_insert(0);
        *entry += 1;
        mean += *int;
    }

    let mean = (mean as f64) / (length as f64);

    let mut frequency = 0;
    let mut mode = 0;

    for (key, value) in map.into_iter() {
        if value > frequency {
            frequency = value;
            mode = key;
        }
    }

    return (mean, median, mode);
}
