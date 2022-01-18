use std::env;
use std::fs;

fn main() {
    // Statements here are executed when the compiled binary is called

    /*let mut name = String::from("anton");
    name = String::from("bill");
    name.push('\u{0000}');
    println!("my name is: {}", name.len());
    let expensive_closure = |num| {
        println!("calculating slowly...");
        num
    };
    println!("{}", expensive_closure(5));
    */
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    //let mut myvec: Vec<Vec<u8>>  = Vec::new(); 
    let mut nodes = 0;
    let mut v: Vec<Vec<u32>> = Vec::new();
    //let filename = "random_graph.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let split = contents.lines();
    for lines in split {
        let mut line = String::from(lines);
        line.pop();
        for numbers in line.split(" "){
            if numbers.parse::<i32>().unwrap() >= nodes{
                nodes = numbers.parse::<i32>().unwrap();
            }
        }
    }
    for i in 0..nodes{
        //arxikopoish vector me mhdenika vectors
    }
    //let filename = "random_graph.txt";
    /*let mut nodes = 0;
    
    println!("nodes: {}", nodes);
    const SIZE = nodes;
    let mut array: [Vec<u8>; SIZE] = Default::default();
*/
    //println!("With text:\n{}", contents);
}
