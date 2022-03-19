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

fn main() {

    let mut args = std::env::args();
    args.next();

    let nodes: u32 = args.next().unwrap().parse().unwrap();
    //let edges: usize = args.next().unwrap().parse().unwrap();
    
    //let nodes: u32 = 10000;
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

            /*let probe = triangles(&edges)
				.filter(move |_| inspect)
				.map(|_| ())
				.inspect(move |x| println!("{:?}\tTriangles {:?}", timer.elapsed(), x.2))
				//.probe_with(&mut probe)
                .probe();*/
				

            let out_degr_distr_in =
            edges
                .map(|(src, _dst)| src);
                //.count_total();
                //.map(|(_src, cnt)| cnt as usize);
                //.count_total();

            let out_degr_distr_out =
            edges
                .map(|(_src, dst)| dst);
                //.count_total();
                //.map(|(_src, cnt)| cnt as usize);
                //.count_total();

            let out_degr_distr = out_degr_distr_in.concat(&out_degr_distr_out)
                                .count_total()
                                .map(|(_src, cnt)| cnt as usize)
                                .count_total();

            // show us something about the collection, notice when done.
            let probe =
            out_degr_distr
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
                for round in 1..12{

                    input.advance_to(round);

                    // twra fainetai na einai komple
                    if round != 11 {
                        //input.update((1,4), 1);
                        //input.update((1,4), -1);
                        input.update((rng1.gen_range(0, nodes), rng1.gen_range(0, nodes)), 1);
                        input.update((rng2.gen_range(0, nodes), rng2.gen_range(0, nodes)), -1);

                    }
                        //println!("test");

                    //input.update((rng1.gen_range(0, nodes), rng1.gen_range(0, nodes)), 1);
                    // ta tupwnei kai ta ektelei ston epomeno guro
                    //opote an valw to round na phgainei gia mia timh parapanw
                    // tha tupwsei opws ta thelw
                    // kai apla sto telos tha uparxoun kai merika extra pou tha
                    // eiani oi allages tou teleutaiou round
                    // opote mporw aploa na valw sto teleutaio round na mhn uparxoun allages
                    // if round == next{
                    if round > next {
                        let timer = ::std::time::Instant::now();
                        input.flush();
                        while probe.less_than(input.time()) {
                            worker.step();
                        }
                        println!("round {} finished after {:?}", next, timer.elapsed());
                        next += batch;
                    }
                }
            //println!("round final finished after {:?}", timer.elapsed());

            }
        }
    }).unwrap();
}