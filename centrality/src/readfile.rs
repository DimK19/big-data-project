pub type Vertex = u32;
pub type Edge<V> = (V, V);
//pub type Time = ();
//pub type Iter = u32;
//pub type Diff = isize;

pub fn load_graph(filename: &str) -> Vec<Edge<Vertex>>
{
    use graph_map::GraphMMap;

    let mut result = Vec::new();

    let graph = GraphMMap::new(&filename);
    for &e in graph.edges(0)
    {
        println!("{}", e);
    }
    /*
    for vertex in 0..graph.nodes()
    {
        println!("{}", vertex); // only prints up to seven, perhaps binary representation length
        /*println!("deg({}) = {}", vertex, graph.edges(vertex).len());
        for &edge in graph.edges(vertex)
        {
            result.push((vertex as Vertex, edge as Vertex));
        }*/
    }
    */
    result
}
