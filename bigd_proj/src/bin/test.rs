extern crate rand;
extern crate timely;
extern crate differential_dataflow;


fn main() {

    let mut args = std::env::args();
    args.next();

    //let nodes: u32 = args.next().unwrap().parse().unwrap();
    //let edges: usize = args.next().unwrap().parse().unwrap();
    
    //let nodes: u32 = 5;
    //let edges: usize = 50;
    let _batch: u32 = args.next().unwrap().parse().unwrap();
    let _inspect: bool = args.next().unwrap() == "inspect";
    
    let filename = String::from("edges_to_change.txt");
    //let nums = bigd_proj::getnodes(filename).0;
    let (changes,sizes,rounds) = bigd_proj::getchanges(filename);
    println!("{}\n",rounds);
    for i in sizes{
        println!("{}",i);
    }
    println!("");
    for i in changes{
        println!("{:?}",i);
    }
    
    
}