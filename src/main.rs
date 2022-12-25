#![allow(unused)]
use std::io;
use std::io::Write;
use rand::Rng;

const CIRCLES:[&str;6] = [
    "●", "○", "◌", "◍", "◌", "◍"
];

const COLORS:[&str;15] = [
    /*Reds */
    "[0;31m", "[91m", "[1;31m",//"[2;31m",
    /*Greens */
    "[0;32m", "[92m", "[92m", "[0;32m", "[1;32m", //"[2;32m",
    "[0;32m", "[92m", "[92m", "[0;32m", "[1;32m",
    /*Blues */
    "[0;34m", //"[2;34m", 
    /*Yellow */
    "[93m", //"[2;33m"
];

fn main() {
    println!("Christmas Tree");
    print!("Enter the # of layers of the tree>>>");
    io::stdout().flush();
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    /*convert input: String to input: u16 */
    let input: u16 = input.trim().parse::<u16>().unwrap();


    generate_tree(0, input);
    generate_tree(input/2, input);
    generate_tree(input/2, input);
    generate_pole(input);
}

fn generate_tree(bottom: u16, num: u16) {
    for i in bottom..num {
        for j in 0..num - i{
            print!(" ");
        }
        for k in 0..2*i + 1{
            let mut rng = rand::thread_rng();
            print!("\x1b{}{}\x1b[0m",
                COLORS[rng.gen_range(0..15)],
                CIRCLES[rng.gen_range(0..6)]
            );
        }
        println!("");
    }
    io::stdout().flush();
}

fn generate_pole(length: u16){
    for i in 0..length / 2 {
        for j in 0..length - 2 {
            print!(" ");
            io::stdout().flush();
        }
        println!("\x1b[100m██████\x1b[0m");
    }
}