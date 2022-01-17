mod readfile;
//use std::fs::File;

fn main()
{
    // .offsets and .targets for vertices and edges respectively
    readfile::load_graph("C:/Users/KYRIAKOS/Desktop/test");
    /*
    let f = File::open("C:/Users/KYRIAKOS/Desktop/big-data-project/graph-generation/test.txt").ok().expect("error reading file");
    let size = f.metadata().ok().expect("error reading metadata").len() as usize;

    println!("{}", size);
    */
}
