fn part1(left_array: &Vec<i32>, right_array: &Vec<i32>) {
    // Subtract the arrays and take absolute then add them
    let result: i32 = left_array
        .iter()
        .zip(right_array.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Result Part 1 : {:?}", result);
}

fn part2(left_array: &Vec<i32>, right_array: &Vec<i32>) {
    let mut i = 0;
    let mut j = 0;
    let mut result = 0;
    while i < left_array.len() && j < right_array.len() {
        let left = left_array[i];
        let right = right_array[j];
        if left == right {
            let count1 = left_array[i..].iter().take_while(|&&x| x == left).count();
            let count2 = right_array[j..].iter().take_while(|&&x| x == right).count();
            result += left * count1 as i32 * count2 as i32;
            i += count1;
            j += count2;
        } else if left < right {
            i += 1;
        } else if right < left {
            j += 1;
        }
    }
    println!("Result Part 2 : {:?}", result);
}

fn main() {
    let (mut left_array, mut right_array): (Vec<i32>, Vec<i32>) = include_str!("input.txt")
        .lines()
        .map(|line| {
            // reading each line of the file into a tuple
            let parts: Vec<&str> = line.split_whitespace().collect();
            (
                parts[0].parse::<i32>().unwrap(),
                parts[1].parse::<i32>().unwrap(),
            )
        })
        .unzip(); // separate tuples into two vectors

    // Sort the vectors
    left_array.sort();
    right_array.sort();
    part1(&left_array, &right_array);

    part2(&left_array, &right_array);
}

