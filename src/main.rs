use std::fs;

fn main() {
    println!("Hello, world!");
    let result = day_1_2("inputs/input_1.txt");
    println!("The answer is: {result}");
}


fn day_1_2(file_path : &str) -> i32 {
    println!("Find all the occurrences of number in the left list and multiply by the number of times it appears in the right list.");

    let mut list1 = vec![];
    let mut list2 = vec![];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
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

fn day_1_1(file_path : &str) -> i32 {
    println!("Sort 2 lists and then compare the values at each index and add the difference to the count.");

    let mut list1 = vec![];
    let mut list2 = vec![];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
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
