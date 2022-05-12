extern crate timely;
extern crate graph_map;
extern crate differential_dataflow;

use timely::order::Product;
use timely::dataflow::{*, operators::Filter};
use differential_dataflow::operators::{*, iterate::Variable};
use differential_dataflow::AsCollection;
use differential_dataflow::input::Input;

fn main() {

    let mut args = std::env::args();
    args.next();

    let batch: u32 = args.next().unwrap().parse().unwrap();
    let inspect: bool = args.next().unwrap() == "inspect";
    let iterations: u32 = std::env::args().nth(1).unwrap().parse().unwrap();

    let filename = String::from("random_graph.txt");
    let (nums,_nodes) = bigd_proj::getnodes(filename);

   timely::execute_from_args(std::env::args().skip(3), move |worker| {

        let index = worker.index();
        let peers = worker.peers();

        // create a degree counting differential dataflow
        let (mut input, probe) = worker.dataflow::<u32,_,_>(|scope| {

            // create edge input, count a few ways.
			let (input, edges) = scope.new_collection::<_,isize>();

            let nodes =
            edges.flat_map(|(x,y)| Some(x).into_iter().chain(Some(y)))
                 .distinct();
        
            // snag out-degrees for each node.
            let degrs = edges.map(|(src,_dst)| src)
                             .count();
        
            let probe = edges.scope().iterative::<u32,_,_>(|inner| {
        
                // Bring various collections into the scope.
                let edges = edges.enter(inner);
                let nodes = nodes.enter(inner);
                let degrs = degrs.enter(inner);
        
                // Initial and reset numbers of surfers at each node.
                let inits = nodes.explode(|node| Some((node, 6_000_000)));
                let reset = nodes.explode(|node| Some((node, 1_000_000)));
        
                // Define a recursive variable to track surfers.
                // We start from `inits` and cycle only `iters`.
                let ranks = Variable::new_from(inits, Product::new(Default::default(), 1));
        
                // Match each surfer with the degree, scale numbers down.
                let to_push =
                degrs.semijoin(&ranks)
                     .threshold(|(_node, degr), rank| (5 * rank) / (6 * degr))
                     .map(|(node, _degr)| node);
        
                // Propagate surfers along links, blend in reset surfers.
                let mut pushed =
                edges.semijoin(&to_push)
                     .map(|(_node, dest)| dest)
                     .concat(&reset)
                     .consolidate();
        
                if iterations > 0 {
                    pushed =
                    pushed
                     .inner
                     .filter(move |(_d,t,_r)| t.inner < iterations)
                     .as_collection();
                }
        
                // Bind the recursive variable, return its limit.
                ranks.set(&pushed);
                pushed.leave()
            })//.map(|_| 0) 
              // me auto to map emfanizei to athroisma olwn twn pageranks
              // an einai megalos o graphos den xwraei na tupwnei gia ton kathena ksexwrista)
              // an theloume gia kathena ksexwrista apla svhnoume to map
              .filter(move |_| inspect)
              .consolidate()
              .inspect(|x| println!("{:?}", x))
              .probe();
			
			(input, probe)
        });

        // o arithmos twn zeugariwn
        let len: usize = nums.len()/2;

        // Load up graph data. Round-robin among workers.
        for i in 0 .. (len / peers) + if index < (len % peers) { 1 } else { 0 } {
            input.update((*nums.get(2*index + 2*i*peers).unwrap(), *nums.get(2*index + 2*i*peers + 1).unwrap()),1);
        }

        let filename = String::from("edges_to_change.txt");
        let (mut changes, mut no_changes, no_rounds) = bigd_proj::getchanges(filename);
        input.advance_to(1);
        let timer = ::std::time::Instant::now();
        input.flush();
        worker.step_while(|| probe.less_than(input.time()));

        if index == 0 {
            println!("round 0 finished after {:?} (loading)", timer.elapsed());
        }

        if batch > 0 {

            // Just have worker zero drive input production.
            if index == 0 {

                let mut next = batch;

                for round in 1..no_rounds+2{

                    input.advance_to(round);

                    // mallon kalutera o upologismos prwta kai h eisagwgh meta
                    if round > next && round != 1{
                        let timer = ::std::time::Instant::now();
                        input.flush();
                        while probe.less_than(input.time()) {
                            worker.step();
                        }
                        println!("round {} finished after {:?}", next, timer.elapsed());
                        next += batch;
                    }

                    // prosoxh prwta fortwnei tou epomenou kai meta kanei tous upologismous tou prohgoumenou
                    if round != no_rounds+1{
                        let changes_temp = no_changes.pop_front().unwrap();
                        for _i in 0..changes_temp {
                            let helper = changes.pop_front().unwrap();
                            input.update((helper.0,helper.1),helper.2 as isize);
                        }
                    }           
                }
            }
        }
    }).unwrap();
}