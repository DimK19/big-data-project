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
