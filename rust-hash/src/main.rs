use std::fs::File;
use std::io::{Read, Write};
use std::io;

fn read_str() -> String {
    let mut str: String = String::new();
    io::stdin().read_line(&mut str).unwrap();
    str.pop();
    str
}

fn main() {
    let f = File::open("input.txt");

    let t = read_str();

    let mut f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file {:?}", error),
    };

    let mut s: String = String::new();

    let s: String = match f.read_to_string(&mut s) {
        Ok(_) => s,
        Err(_) => String::from("-"),
    };

    let ss = s.clone();

    if t == "encode" {
        let ans: String = encode_message(s);

        let mut f = File::create("output.txt").expect("error in File::create");
        f.write_all(ans.as_bytes()).expect("Error in f.write(all");
    }
    
    if t == "decode" {
        let ans: String = decode_message(ss);

        let mut f = File::create("output.txt").expect("error in File::create");
        f.write_all(ans.as_bytes()).expect("Error in f.write(all");
    }

}

fn encode_message(s: String) -> String {
    let mut ans: String = String::from("");

    for c in s.chars() {
        let q = c.clone();
        let qq = q as u8;
        ans.push((qq + 97) as char);
    }

    ans
}

fn decode_message(s: String) -> String {
    let mut ans: String = String::from("");

    for c in s.chars() {
        let q = c.clone();
        let qq = q as u8;
        ans.push((qq - 97) as char);
    }

    ans
}

