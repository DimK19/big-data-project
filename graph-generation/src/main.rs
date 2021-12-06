use rand::distributions::{Bernoulli, Distribution};
use std::fs;

fn main()
{
    let vertices: i8 = 20;
    let p: f64 = 0.2; // not f32, 64 required by Bernoulli, whatever

    // https://docs.rs/rand/0.8.4/rand/distributions/struct.Bernoulli.html
    let distribution = Bernoulli::new(p).unwrap();
    let mut edges = Vec::new();

    for v1 in 1..vertices
    {
        for v2 in 1..vertices
        {
            let decision = distribution.sample(&mut rand::thread_rng());
            if decision
            {
                edges.push((v1, v2));
            }
        }
    }

    let mut all_lines = String::new();
    for e in edges
    {
        let line = format!("{}, {}", e.0, e.1);
        all_lines.push_str(&line);
        all_lines.push('\n');
    }
    fs::write("./test.txt", all_lines).expect("Unable to write file");
}
