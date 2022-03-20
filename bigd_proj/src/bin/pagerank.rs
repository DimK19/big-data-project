extern crate rand;
extern crate timely;
extern crate differential_dataflow;

use bigd_proj::lib;

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

    //let nodes: u32 = args.next().unwrap().parse().unwrap();
    //let edges: usize = args.next().unwrap().parse().unwrap();

    //let nodes: u32 = 10000;
    //let edges: usize = 50;
    let batch: u32 = args.next().unwrap().parse().unwrap();
    let inspect: bool = args.next().unwrap() == "inspect";

    let filename = String::from("random_graph.txt");
    //let nums = bigd_proj::getnodes(filename);
    //let (nums, nodes) = bigd_proj::getnodes(filename);
    let larry = lib::get_edges(filename);
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
                let sergey =
                larry
                    .to_stream(scope)
                    .map(|edge| (edge, 0, 1))
                    .as_collection()
                    .map(|(src, dst)| if src < dst { (src, dst) } else { (dst, src) })
                    .distinct();

                pagerank(20, &sergey)
                    .filter(move |_| inspect)
                    .map(|_| ())
                    .consolidate()
                    .inspect(move |x| println!("{:?} idk {:?}", timer.elapsed(), x))
                    .probe_with(&mut probe);
                /*
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
*/
            // show us something about the collection, notice when done.
/*
            let probe =
            out_degr_distr
                .filter(move |_| inspect)
                .inspect(|x| println!("observed: {:?}", x))
                .probe();

            (input, probe)*/
        }).expect("Timely computation failed");

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
                let no_rounds: u32 = 4;

                for round in 1..no_rounds+2{

                    input.advance_to(round);

                    // twra fainetai na einai komple
                    if round != no_rounds+1 {
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


// Returns a weighted collection, in which the weight of each vertex is proportional
// to its pg value in the input graph "edges".
fn pagerank<G>(iters: u32, edges: &Collection<G, (u31, u32), isize>) -> Collection<G, u32, isize>
where
    G: timely::dataflow::scopes::Scope,
    G::Timestamp: differential_dataflow::lattice::Lattice,
{
    // initialize many surfers at each node
    let nodes = edges.flat_map(|(x, y)| Some(x).into_iter().chain(Some(y))).distinct();

    // snag-out degrees at each nodes
    let degrs = edges.map(|(src, _dst)| src).count();

    edges.scope().iterative::<u32, _, _>(|inner| {
        // Bring various collections into the scope
        let edges = edges.enter(inner);
        let nodes = nodes.enter(inner);
        let degrs = degrs.enter(inner);

        // Initial and reset numbers of surfers at each node
        let inits = nodes.explode(|node| Some((node, 6_000_000)));
        let reset = nodes.explode(|node| Some((node, 1_000_000)));

        // Define a recursive variable to track surfers
        // Starting from "inits" and cycling only "iters"
        let ranks = Variable::new_from(inits, Product::new(Default::default(), 1));

        // Match each surfer with the degree, scale numbers down
        let to_push =
        degrs.semijoin(&ranks)
             .threshold(|(_node, degr), rank| (5 * rank) / (6 * degr))
             .map(|(node, _degr)| node);

        // Propagate surfers along links, blend in reset surfers
        let mut pushed =
        edges.semijoin(&to_push)
             .map(|(_node, dest)| dest)
             .concat(&reset)
             .consolidate();

        if iters > 0
        {
            pushed =
            pushed
             .inner
             .filter(move |(_d, t, _r)| t.inner < iters)
             .as_collection();
        }

        // Bind the recursive variable, return its limit
        ranks.set(&pushed);
        pushed.leave()
    })
}
