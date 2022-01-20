use bigd_proj::{Node, Edge};


use timely::dataflow::operators::to_stream::ToStream;
use timely::dataflow::operators::map::Map;
use differential_dataflow::Collection;
use differential_dataflow::collection::AsCollection;
use differential_dataflow::operators::consolidate::Consolidate;
use differential_dataflow:: operators::reduce::Threshold;
use differential_dataflow::input::InputSession;
use timely::dataflow::{InputHandle, ProbeHandle};
use timely::dataflow::operators::{Inspect, Probe};
use std::io::{stdin,stdout,Write};


//use differential_dataflow:: operators::arrange::arrangement::ArrangeBySelf;
//use differential_dataflow:: operators::arrange::arrangement::ArrangeByKey;
//use differential_dataflow:: operators::join::JoinCore;
//use differential_dataflow:: operators::reduce::Reduce;

fn main() {

		let source = std::env::args().nth(1).expect("Must supply filename");
		let target = String::from("./test_raw");
		
		//mporoume na treksoume to parse eite mesw tou parse opws parakatw h mesw to bigd_proj::parse epeidh exw kanei copy paste ton kwdika sto lib.rs
		bigd_proj::parse(&source, &target);
	
	timely::execute_from_args(std::env::args(), |worker| {
		
		let filename = String::from("./test_raw");
		let inspect = std::env::args().nth(2).expect("Must supply inspect").parse::<bool>().expect("Could not parse inspect");
		
		let index = worker.index();
		let peers = worker.peers();
		let timer = worker.timer();
		
		//let edges = bigd_proj::load_graph(&filename, index, peers);
		
		
		//println!("{:?}\tLoaded {} edges", timer.elapsed(), edges.len());
		
        let mut input = InputHandle::new();
        let mut probe = ProbeHandle::new();


		worker.dataflow(|scope| {
			
			let timer = timer.clone();
            //let (input, stream) = scope.new_input();
			

			/*if index == 0{
				let count = edges.len();

				for i in 0 .. count {
					println!("{}{}", edges[i].0,edges[i].1);
				}
			}*/
            //input.to_stream(scope)
            //.inspect(|x| println!("seen: {:?}", x))
            //.probe_with(&mut probe);

            println!("we are here");
            //let mut input = input.to_stream(scope);

            //edges.inspect(move |x| println!("{:?}", x));

            //println!("we are there");

			let edges = 
			input
				.to_stream(scope)
				.map(|edge| (edge, 0, 1))
				.as_collection()
				.map(|(src,dst)| if src < dst { (src,dst) } else { (src,dst) })
				.distinct()
				;

			/*use timely::dataflow::operators::capture::Extract;
			for i in 0 .. 20 {
				println!("{:?}", scope.extract());
			}*/
			

			triangles(&edges)
				.filter(move |_| inspect)
				.map(|_| ())
				.consolidate()
				.inspect(move |x| println!("{:?}\tTriangles {:?}", timer.elapsed(), x.2))
				.probe_with(&mut probe)
				;
		});

        let edges = bigd_proj::load_graph(&filename, index, peers);
		
        input.advance_to(0);
        for i in 0 .. edges.len() {
            //println!("{:?}",edges[i]);
            input.send((edges[i].0,edges[i].1));
            while probe.less_than(input.time()){
                worker.step();
            }
        }

        println!("***************************************");
        let mut s=String::new();

        stdin().read_line(&mut s).expect("Did not enter a correct string");

        input.advance_to(1);
        input.send((2,7));
        worker.step();
		
		
		println!("{:?}\tComputation stable", timer.elapsed());
	
	}).expect("Timely computation failed to start");

}

fn triangles<G>(edges: &Collection<G, Edge<Node>>) -> Collection<G, (Node, Node, Node)>
where
	G: timely::dataflow::scopes::Scope,
	G::Timestamp: differential_dataflow::lattice::Lattice+std::hash::Hash+Ord
{
	use differential_dataflow::operators::join::Join;

	//gia tupwma
	/*edges
		.inspect(move |x| println!("{:?}\t{:?}\t{:?}", x.0, x.1, x.2));*/

	edges
		.join(&edges)
		.map(|(a,(b,c))| (a,b,c))
		.filter(|(_a,b,c)| b < c)
		.map(|(a,b,c)| ((b,c),a))
		.semijoin(&edges)
		.map(|((b,c),a)|(a,b,c))

}


