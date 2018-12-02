use std::io;
use std::io::Read;

fn main() {
    let mut my_str = String::new();
    io::stdin().read_to_string(&mut my_str)
        .expect("Failed to read from stdin");

    let lines: Vec<_> = my_str.split("\n").filter(|x: &&str| !x.is_empty()).collect();

    println!("{}", checksum(&lines))

}

fn checksum(lines: &Vec<&str>) -> usize {
    // perf: could store get_counts_arr results and reuse
    let c3 = lines.into_iter().filter(|l| is_exactly_thrice(*l)).count();
    let c2 = lines.into_iter().filter(|l| is_exactly_twice(*l)).count();
    c2*c3
}

fn is_exactly_twice(my_str: &str) -> bool {
    let arr = get_counts_arr(my_str);
    arr.iter().any(|e| *e == 2)
}

fn is_exactly_thrice(my_str: &str) -> bool {
    let arr = get_counts_arr(my_str);
    arr.iter().any(|e| *e == 3)
}

fn get_counts_arr(my_str: &str) -> [u32; 26] {
    let mut arr: [u32; 26] = [0; 26];
    for c in my_str.chars() {
        let i = ((c as u32) - 97) as usize; // guaranteed ascii
        arr[i] += 1;
    }
    arr
}