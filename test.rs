//use std::env;
use std::fs;
use std::collections::VecDeque;


fn main(){
    let filename = String::from("random_graph.txt");
    let mut nums = getnodes(filename);   
    println!("{} {}", nums.pop_front().unwrap(), nums.pop_front().unwrap());
    println!("{} {}", nums.pop_front().unwrap(), nums.pop_front().unwrap());

}

fn getnodes(filename: String) -> VecDeque<u32>  {
    // --snip--
    //let mut filename = String::from("random_graph.txt");    
    //println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut buf = VecDeque::new();
    
    let split = contents.split("\n");
    for i in split{
        //println!("{}",i);
        //i.what_is_this();
        let mut parts = i.split_whitespace().map(|s| s.parse::<u32>());
        let node1 = parts.next().unwrap().unwrap();
        let node2 = parts.next().unwrap().unwrap();
        /*let mut split2 = i.split_whitespace();
        let mut node1: i32 = split2.next().map(str::to_string).parse().unwrap();    
        let mut node2: i32 = split2.next().map(str::to_string).parse().unwrap();  */  
        buf.push_back(node1);
        buf.push_back(node2);

    }
    return buf;

}