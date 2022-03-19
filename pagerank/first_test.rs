use std::collections::VecDeque;
use std::fs;
use std::iter::Iterator;
use simple_pagerank::Pagerank;

// https://crates.io/crates/simple-pagerank
// https://docs.rs/crate/simple-pagerank/latest
// https://docs.rs/simple-pagerank/latest/simple_pagerank/struct.Pagerank.html
// https://masonium.github.io/rustdoc/linxal/
// https://docs.rs/eigenvalues/latest/eigenvalues/

fn main()
{
    let filename = String::from("random_graph.txt");
    let edges = get_edges(filename);

    let mut larry = Pagerank::<u32>::new();
    // think about damping factor

    /*
    for i in 0..10
    {
        println!("count {}: {}", i, edges.iter().filter(|&n| *n == i).count());
    }
    */

    for i in edges
    {
        println!("{} {}", i.0, i.1);
        larry.add_edge(i.0, i.1);
    }

    larry.calculate();
    // print result (always sorted)

    larry.nodes()
        .iter()
    	.map(|(node, score)| println!("page {} with score {}", node, score))
    	.for_each(drop);
}

fn get_edges(filename: String) -> VecDeque<(u32, u32)>
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
