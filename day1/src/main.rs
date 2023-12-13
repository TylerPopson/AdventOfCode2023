use std::fs;
fn main() {

    let input = fs::read_to_string("input.txt").unwrap();
    let mut sum:u32=0;

    for line in input.lines(){ 
        let replaced = line.replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "eight9eight");

        let parsed: Vec<&str> = replaced.matches(char::is_numeric).collect();
        let numbers: Vec<u32> = parsed.iter().map(|&s| s.parse().unwrap()).collect();
        let lin_num: u32 = (10*numbers[0])+numbers[numbers.len()-1];
        sum+=lin_num;

    }
    
    println!("{}", sum);
}