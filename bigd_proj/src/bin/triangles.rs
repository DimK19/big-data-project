extern crate rand;
extern crate timely;
extern crate differential_dataflow;

use rand::{Rng, SeedableRng, StdRng};

// use bigd_proj::{Node, Edge};

// use timely::dataflow::operators::to_stream::ToStream;
// use timely::dataflow::operators::map::Map;
// use differential_dataflow::Collection;
// use differential_dataflow::collection::AsCollection;
// use differential_dataflow::operators::consolidate::Consolidate;
// use differential_dataflow:: operators::reduce::Threshold;
// use differential_dataflow::operators::Count;

use differential_dataflow::input::Input;
use differential_dataflow::operators::count::CountTotal;
use crate::differential_dataflow::operators::Join;

fn main() {

    let mut args = std::env::args();
    args.next();

    let nodes: u32 = args.next().unwrap().parse().unwrap();
    //let edges: usize = args.next().unwrap().parse().unwrap();
    
    //let nodes: u32 = 5;
    //let edges: usize = 50;
    let batch: u32 = args.next().unwrap().parse().unwrap();
    let inspect: bool = args.next().unwrap() == "inspect";
    
    let filename = String::from("random_graph.txt");
    let nums = bigd_proj::getnodes(filename);
    //nums.pop_front().unwrap().helppp();

    //println!("{} {}", nums.pop_front().unwrap(), nums.pop_front().unwrap());
    //println!("{} {}", nums.pop_front().unwrap(), nums.pop_front().unwrap());
    // define a new computational scope, in which to run BFS
    timely::execute_from_args(std::env::args().skip(3), move |worker| {

        // to metaferw pio kata gia na metraw mono to xrono twn upologismwn
        // let timer = ::std::time::Instant::now();
        
        let index = worker.index();
        let peers = worker.peers();

        // create a degree counting differential dataflow
        let (mut input, probe) = worker.dataflow::<u32,_,_>(|scope| {

            // create edge input, count a few ways.
            let (input, edges) = scope.new_collection::<_,isize>();
			      
            let triangles = 
            edges
                .join(&edges)
                .map(|(a,(b,c))| (a,b,c))
                .filter(|(_a,b,c)| b < c)
                .map(|(a,b,c)| ((b,c),a))
                .semijoin(&edges)
                .map(|((b,c),a)|(a,b,c))
                .map(|(_a,_b,_c)| 1)
                .count_total()
                .map(|(_src, cnt)| cnt as usize)
                .count_total();

            // show us something about the collection, notice when done.
            let probe =
            triangles
                .map(|(src, _cnt)| src)
                .filter(move |_| inspect)
                .inspect(|x| println!("observed: {:?}", x))
                .probe();

            (input, probe)
        });

        let seed: &[_] = &[1, 2, 3, index];
        let mut rng1: StdRng = SeedableRng::from_seed(seed);    // rng for edge additions
        let mut rng2: StdRng = SeedableRng::from_seed(seed);    // rng for edge deletions

        //println!("i am worker {}", index);
        let len: usize = nums.len()/2; // o arithmos twn zeugariwn
        // Load up graph data. Round-robin among workers.
        for i in 0 .. (len / peers) + if index < (len % peers) { 1 } else { 0 } {
            input.update((*nums.get(2*index + 2*i*peers).unwrap(), *nums.get(2*index + 2*i*peers + 1).unwrap()),1);
            //input.update((rng1.gen_range(0, nodes), rng1.gen_range(0, nodes)), 1)
            //println!("{} {} {}", *nums.get(2*index + 2*i*peers).unwrap(), *nums.get(2*index + 2*i*peers+1).unwrap(), index);
        }

        /*let len: usize = nums.len()/2;
        if index == 0{
            for i in 0 .. len{
                input.update((*nums.get(i*2).unwrap(), *nums.get(i*2+1).unwrap()),1);
                //println!("{} {}", *nums.get(i*2).unwrap(),*nums.get(i*2+1).unwrap())
            }
        }*/

        let timer = ::std::time::Instant::now();
        input.advance_to(1);
        input.flush();
        worker.step_while(|| probe.less_than(input.time()));

        if index == 0 {
            println!("round 0 finished after {:?} (loading)", timer.elapsed());
        }

        if batch > 0 {
            // Just have worker zero drive input production.
            if index == 0 {

                let mut next = batch;
                for round in 1..17{

                    input.advance_to(round);

                    if round != 16 {
                        //input.update((2,4), 1);
                        //input.update((1,2), -1);
                        input.update((rng1.gen_range(0, nodes), rng1.gen_range(0, nodes)), 1);
                        input.update((rng2.gen_range(0, nodes), rng2.gen_range(0, nodes)), -1);
                    }


                    //input.update((1,2), 1);
                    //input.update((2,4), 1);

                    //let node1 = rng1.gen_range(0, nodes);
                    //let node2 = rng1.gen_range(0, nodes);
                    //println!("{} {}", node1, node2);
                    //input.update((node1,node2), 1);

                    //input.update((rng2.gen_range(0, nodes), rng2.gen_range(0, nodes)), -1);
                    //input.update((rng1.gen_range(0, nodes), rng1.gen_range(0, nodes)), 1);
                    if round > next {
                    //if round == next {
                        let timer = ::std::time::Instant::now();
                        input.flush();
                        while probe.less_than(input.time()) {
                            worker.step();
                        }
                        println!("round {} finished after {:?}", next, timer.elapsed());
                        next += batch;
                    }
                }
            }
        }
    }).unwrap();
}