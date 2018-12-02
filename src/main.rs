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
    let arrays: Vec<_> = lines.into_iter().map(|l| get_counts_arr(l)).collect();
    let a2 = (&arrays).into_iter().filter(|a| a.iter().any(|e| *e == 2)).count();
    let a3 = (&arrays).into_iter().filter(|a| a.iter().any(|e| *e == 3)).count();
    a2*a3
}

fn get_counts_arr(my_str: &str) -> [u32; 26] {
    let mut arr: [u32; 26] = [0; 26];
    for c in my_str.chars() {
        let i = ((c as u32) - 97) as usize; // guaranteed ascii
        arr[i] += 1;
    }
    arr
}