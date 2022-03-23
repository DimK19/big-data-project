use std::collections::VecDeque;
use std::fs;
use std::process;

// tou dhmhtrh
pub type Node = u32;
pub type Edge<N> = (N, N);
pub type Iter = u32;
pub type Diff = isize;
// tou dhmhtrh

pub fn getnodes(filename: String) -> (VecDeque<u32>, u32)  {
    // --snip--
    //let mut filename = String::from("random_graph.txt");
    //println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut buf = VecDeque::new();

    let mut max: u32 = 0;

    let split = contents.split("\n");
    for i in split{
        // println!("{}",i);
        // i.what_is_this();
        // kalwntas mia sunarthsh pou den uparxei vlepei apo to error to type tou antikeimenou gia to opoio thn kaleses
        let parts = i.split_whitespace().map(|s| s.parse::<u32>());
        let mut count: u32 = 0;
        for p in parts{
            if count < 2{
                let val = p.unwrap();
                if val > max{
                    max = val;
                }
                buf.push_back(val)
            }
            count += 1;
        }
        if count > 2 || count == 1{
            println!("There was probably an error with the formatting of the edges file. Please check!");
            process::exit(1);
        }
        /*
        // auto htan to palio part alla eskage an uparxe newline sto telos tou arxeiou
        // twra den skaei alla kai pali prepei na ginetai manual elegxos oti den uparxei grammh me mia mono timh
        let node1 = parts.next().unwrap().unwrap();
        let node2 = parts.next().unwrap().unwrap();
        let mut split2 = i.split_whitespace();
        let mut node1: i32 = split2.next().map(str::to_string).parse().unwrap();
        let mut node2: i32 = split2.next().map(str::to_string).parse().unwrap();
        buf.push_back(node1);
        buf.push_back(node2);
        */
    }
    //println!("{}",max+1);
    return (buf,max);
}

pub fn getchanges(filename: String) -> (VecDeque<(u32,u32,i32)>, VecDeque<u32>, u32)  {

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut buf = VecDeque::new();
    let mut sizes = VecDeque::new();


    let split = contents.split("\n");
    
    let mut cur_size: u32 = 0;
    let mut temp_size: u32 = 0;
    let mut rounds:u32 = 0;

    for i in split{

        let parts = i.split_whitespace().map(|s| s.parse::<i32>());

        let mut count: u32 = 0;
        let mut temp = VecDeque::new();

        for p in parts{
            if count < 3{
                let val = p.unwrap();
                temp.push_back(val);
            }
            count += 1;
        }
        if count == 3 {
            if temp_size+1 <= cur_size{
                if *temp.get(0).unwrap() < 0 || *temp.get(1).unwrap() < 0{
                    println!("There was probably an error with the formatting of the edges file. Please check!");
                    process::exit(1);
                }
                if *temp.get(2).unwrap() != -1 && *temp.get(2).unwrap() != 1{
                    println!("There was probably an error with the formatting of the edges file. Please check!");
                    process::exit(1);
                }
                buf.push_back((*temp.get(0).unwrap() as u32,*temp.get(1).unwrap() as u32,*temp.get(2).unwrap()));
                temp_size += 1;
            } 
            else{
                println!("There was probably an error with the formatting of the edges file. Please check!");
                process::exit(1);
            }
        }
        else if count == 1 {
            if temp_size == cur_size{
                if temp_size != 0{
                    sizes.push_back(cur_size);
                    rounds += 1;
                }
                let tt = *temp.get(0).unwrap();
                //tt.fun();
                cur_size = tt as u32;
                temp_size = 0;
            }
            else{
                println!("There was probably an error with the formatting of the edges file. Please check!");
                process::exit(1);
            }
        }
        else if count != 0 {
            println!("There was probably an error with the formatting of the edges file. Please check!");
            process::exit(1);
        }
    }
    if temp_size == cur_size && cur_size != 0 {
        sizes.push_back(cur_size);
        rounds += 1;
    }
    else if cur_size != 0{
        println!("There was probably an error with the formatting of the edges file. Please check!");
        process::exit(1);
    }
    return (buf,sizes,rounds);
}

pub fn get_edges(filename: String) -> VecDeque<(u32, u32)>
{
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut buf = VecDeque::new();

    let split = contents.split("\n");
    for i in split
    {
        let mut parts = i.split_whitespace().map(|s| s.parse::<u32>());
        let node1 = parts.next().unwrap().unwrap();
        let node2 = parts.next().unwrap().unwrap();
        buf.push_back((node1, node2));
    }
    
    return buf;
}

pub fn load_edges(filename: &str) -> Vec<Edge<Node>>
{
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut buf = Vec::new();

    let split = contents.split("\n");
    for i in split
    {
        if i == "" { continue; }
        let mut parts = i.split_whitespace().map(|s| s.parse::<u32>());
        let node1 = parts.next().unwrap().unwrap();
        let node2 = parts.next().unwrap().unwrap();
        buf.push((node1, node2));
    }

    return buf;
}
