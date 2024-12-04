use std::fs;
use regex::Regex;

fn main() {
    println!("Hello, world!");
    let result = day_3_2("inputs/input_3.txt");
    println!("The answer is: {result}");
}

fn day_3_2(file_path: &str) -> i32 {
    println!("Now there are dos and don'ts");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    recursion_string_fun(&contents, 0)
}

fn recursion_string_fun(s: &str, mut sum: i32) -> i32 {
    let re = Regex::new(r"don\'t\(\)").unwrap();
    match re.find(s) {
        Some(value) => {
            let (left, right) = s.split_at(value.end());
            sum += get_sum_of_muls(left);
            let re2 = Regex::new(r"do\(\)").unwrap();
            match re2.find(right) {
                Some(value2) => {
                    let (_, right2) = right.split_at(value2.start());
                    return recursion_string_fun(right2, sum);
                }
                None => return sum,
            }
        }
        None => sum + get_sum_of_muls(s),
    }
}

fn day_3_1(file_path: &str) -> i32 {
    println!("Now there are dos and don'ts");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    get_sum_of_muls(&contents)
}

fn get_sum_of_muls(contents: &str) -> i32 {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mut sum = 0;
    for mul in re.find_iter(&contents) {
        sum += get_mul_value(mul.as_str());
    }
    sum
}

fn get_mul_value(mul: &str) -> i32 {
    let re = Regex::new(r"\d{1,3}").unwrap();
    let num = re.find_iter(mul).map(|m| m.as_str().parse::<i32>().unwrap());
    num.product::<i32>()
}

// NOT WORKING
fn day_2_2(file_path: &str) -> i32 {
    println!("One number can be removed to make safe.");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut count = 0;
    for line in contents.lines() {
        let words: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let result = check_words(&words);
        let mut line_safe = false;
        match result {
            Ok(()) => {
                line_safe = true;
            }
            Err(index) => {
                {
                    let mut copy = words.clone();
                    copy.remove(0);
                    let result = check_words(&copy);
                    match result {
                        Ok(()) => {
                            println!("Here {:?}", copy);
                            line_safe = true;
                        }
                        Err(_) => {}
                    }
                }
                if !line_safe {
                    let mut copy = words.clone();
                    copy.remove(index);
                    let result = check_words(&copy);
                    println!("there {:?}", copy);
                    match result {
                        Ok(()) => {
                            println!("Here {:?}", copy);
                            line_safe = true;
                        }
                        Err(_) => {}
                    }
                }
            }
        }
        if line_safe {
            println!("Line is safe: {:?}", words);
            count += 1;
        } else {
            println!("Line is not safe: {:?}", words);
        }
    }
    return count;
}

fn check_words(words: &Vec<i32>) -> Result<(), usize> {
    let mut gradual = true;
    let mut non_zero = true;
    let mut increasing = false;
    let mut decreasing = false;

    for i in 1..words.len() {
        let a = words[i - 1];
        let b = words[i];

        if (a - b) > 3 || (a - b) < -3 {
            gradual = false;
        }
        if a - b == 0 {
            non_zero = false;
        }
        if a < b {
            increasing = true;
        }
        if a > b {
            decreasing = true;
        }
        if !(gradual && non_zero && ((increasing && !decreasing) || (!increasing && decreasing))) {
            return Err(i);
        }
    }
    return Ok(());
}

fn day_2_1(file_path: &str) -> i32 {
    println!("Find gradually (<3), non-zero (>0) increasing or decreasing rows of numbers");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut count = 0;

    for line in contents.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();

        let mut gradual = true;
        let mut non_zero = true;
        let mut increasing = false;
        let mut decreasing = false;

        for i in 1..words.len() {
            let a = words[i - 1].parse::<i32>().unwrap();
            let b = words[i].parse::<i32>().unwrap();

            if (a - b) > 3 || (a - b) < -3 {
                gradual = false;
            }
            if a - b == 0 {
                non_zero = false;
            }
            if a < b {
                increasing = true;
            }
            if a > b {
                decreasing = true;
            }
        }
        if gradual && non_zero && ((increasing && !decreasing) || (!increasing && decreasing)) {
            count += 1;
        }
    }
    return count;
}

fn day_1_2(file_path: &str) -> i32 {
    println!("Find all the occurrences of number in the left list and multiply by the number of times it appears in the right list.");

    let mut list1 = vec![];
    let mut list2 = vec![];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    for line in contents.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        list1.push(words[0].parse::<i32>().unwrap());
        list2.push(words[1].parse::<i32>().unwrap());
    }

    let mut count = 0;
    for i in 0..list1.len() {
        let mut tally = 0;
        for j in 0..list2.len() {
            if list1[i] == list2[j] {
                tally += 1;
            }
        }
        count += tally * list1[i];
    }
    return count;
}

fn day_1_1(file_path: &str) -> i32 {
    println!("Sort 2 lists and then compare the values at each index and add the difference to the count.");

    let mut list1 = vec![];
    let mut list2 = vec![];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    for line in contents.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        list1.push(words[0].parse::<i32>().unwrap());
        list2.push(words[1].parse::<i32>().unwrap());
    }
    list1.sort();
    list2.sort();

    let mut count = 0;

    for i in 0..list1.len() {
        if list1[i] > list2[i] {
            count += list1[i] - list2[i];
        } else {
            count += list2[i] - list1[i];
        }
    }
    return count;
}
