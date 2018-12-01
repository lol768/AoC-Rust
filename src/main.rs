use std::io;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut my_str = String::new();
    io::stdin().read_to_string(&mut my_str)
        .expect("Failed to read from stdin");

    let lines: Vec<_> = my_str.split("\n").filter(|x: &&str| !x.is_empty())
        .map(|x: &str| x.parse::<i32>().unwrap()).collect();


    let mut int_vec: HashSet<i32> = HashSet::new();
    int_vec.insert(0);
    let mut last_total: i32 = 0;

    for i in 0..1000 {
        println!("{}", i);
        check(&lines, &mut int_vec, &mut last_total);
    }
}

fn check(lines: &Vec<i32>, int_vec: &mut HashSet<i32>, last_total: &mut i32) -> () {
    for line in lines {
        let total_now = *last_total + line;
        if int_vec.contains(&total_now) {
            println!("Got seen-before-total! {}", &total_now);
            std::process::exit(0);
        }

        *last_total = total_now;
        int_vec.insert(total_now);
    }
}
