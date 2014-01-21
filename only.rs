use std::io::buffered::{BufferedReader};
use std::io::{stdin};
use std::os;

fn main() {
    if os::args().len() > 1 {
        select(os::args()[1]);
    } else {
        cat();
    }
}

fn cat() {
    let mut stdin = BufferedReader::new(stdin());
    for line in stdin.lines() {
        print(line);
    }
}

fn select(identifier: &str) {
    let mut count = 0;
    let mut inside = false;
    let mut stdin = BufferedReader::new(stdin());
    for line in stdin.lines() {
        if line.contains(~"[[#"+identifier+"]]") {
            inside = !inside;
            if inside {
                println!("--------------------- {} {}", identifier, count);
                count += 1;
            }
        } else if inside {
            print(line);
        }
    }
}
