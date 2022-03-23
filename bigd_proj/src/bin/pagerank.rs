extern crate timely;
extern crate graph_map;
extern crate differential_dataflow;

use timely::order::Product;
use timely::dataflow::{*, operators::Filter};

use differential_dataflow::operators::{*, iterate::Variable};
//use differential_dataflow::input::InputSession;
use differential_dataflow::AsCollection;

use crate::differential_dataflow::input::Input;


fn main() {

    // snag a filename to use for the input graph.
    //let filename = std::env::args().nth(1).unwrap();
    let filename = String::from("random_graph.txt");
    let (nums,_nodes) = bigd_proj::getnodes(filename);
    let iterations: u32 = std::env::args().nth(1).unwrap().parse().unwrap();
    let inspect = std::env::args().nth(2) == Some("inspect".to_string());

    timely::execute_from_args(std::env::args().skip(2), move |worker| {

        let peers = worker.peers();
        let index = worker.index();
		
		let (mut input, probe) = 
        worker.dataflow::<u32,_,_>(|scope| {

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
            }).filter(move |_| inspect)
                .consolidate()
                .inspect(|x| println!("test{:?}", x))
                .probe();


            /*let probe = edges.filter(move |_| inspect).inspect(|x| println!("test{:?}", x))
            .probe();*/
            /*let probe = 
			pagerank2(iterations, &edges)
                .filter(move |_| inspect)
                .consolidate()
                .inspect(|x| println!("test{:?}", x))
				.probe();
                //.probe_with(&mut probe);*/
			
			(input, probe)
        });

        let len: usize = nums.len()/2; // o arithmos twn zeugariwn
        for i in 0 .. (len / peers) + if index < (len % peers) { 1 } else { 0 } {
            input.update((*nums.get(2*index + 2*i*peers).unwrap(), *nums.get(2*index + 2*i*peers + 1).unwrap()),1);
        }

        let timer = ::std::time::Instant::now();
        input.advance_to(1);
        input.flush();
        worker.step_while(|| probe.less_than(input.time()));


        if index == 0 {
            println!("round 0 finished after {:?} (loading)", timer.elapsed());
        }
        let batch = 1;
        if batch > 0 {
            // Just have worker zero drive input production.
            if index == 0 {

                let mut next = batch;
                let no_rounds: u32 = 4;
                for round in 1..no_rounds+2{

                    input.advance_to(round);

                    if round != no_rounds+1 {
                        if round == 1{
                            input.update((2,1), 1);
                        }
                        if round == 2{
                            input.update((3,1), 1);
                        }
                    }
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
            }
        }

    }).unwrap();
}

// Returns a weighted collection in which the weight of each node is proportional
// to its PageRank in the input graph `edges`.
/*fn pagerank2<G>(iters: Iter, edges: &Collection<G, Edge, Diff>) -> Collection<G, Node, Diff>
where
    G: Scope,
    G::Timestamp: Lattice,
{
    // initialize many surfers at each node.
    let nodes =
    edges.flat_map(|(x,y)| Some(x).into_iter().chain(Some(y)))
         .distinct();

    // snag out-degrees for each node.
    let degrs = edges.map(|(src,_dst)| src)
                     .count();

    edges.scope().iterative::<Iter,_,_>(|inner| {

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

        if iters > 0 {
            pushed =
            pushed
             .inner
             .filter(move |(_d,t,_r)| t.inner < iters)
             .as_collection();
        }

        // Bind the recursive variable, return its limit.
        ranks.set(&pushed);
        pushed.leave()
    })
}*/