use std::fs;

pub type Node = u32;
pub type Edge<N> = (N, N);
pub type Iter = u32;
pub type Diff = isize;

pub fn load_graph(filename: &str, index: usize, peers: usize) -> Vec<Edge<Node>>
{
    use graph_map::GraphMMap;

    let mut result = Vec::new();

    let graph = GraphMMap::new(&filename);
    for &e in graph.edges(0)
    {
        println!("{}", e);
    }

    for vertex in 0..graph.nodes()
    {
        println!("{}", vertex); // only prints up to seven, perhaps binary representation length
        println!("deg({}) = {}", vertex, graph.edges(vertex).len());
        if vertex % peers == index
        {
            for &edge in graph.edges(vertex)
            {
                result.push((vertex as Node, edge as Node));
            }
        }
    }

    result
}

pub fn load_edges(filename: &str) -> Vec<Edge<Node>>
{
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut buf = Vec::new();

    let split = contents.split("\n");
    for i in split
    {
        if(i == "") { continue; }
        let mut parts = i.split_whitespace().map(|s| s.parse::<u32>());
        let node1 = parts.next().unwrap().unwrap();
        let node2 = parts.next().unwrap().unwrap();
        buf.push((node1, node2));
    }

    return buf;
}
