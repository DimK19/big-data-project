use bigd_proj::{Node, Edge};

mod parse;

use timely::dataflow::operators::to_stream::ToStream;
use timely::dataflow::operators::map::Map;
use differential_dataflow::Collection;
use differential_dataflow::collection::AsCollection;
use differential_dataflow::operators::consolidate::Consolidate;
use differential_dataflow:: operators::reduce::Threshold;
//use differential_dataflow:: operators::arrange::arrangement::ArrangeBySelf;
//use differential_dataflow:: operators::arrange::arrangement::ArrangeByKey;
//use differential_dataflow:: operators::join::JoinCore;
//use differential_dataflow:: operators::reduce::Reduce;

fn main() {

		let source = std::env::args().nth(1).expect("Must supply filename");
		let target = String::from("./test_raw");
		
		//mporoume na treksoume to parse eite mesw tou parse opws parakatw h mesw to bigd_proj::parse epeidh exw kanei copy paste ton kwdika sto lib.rs
		parse::parse(&source, &target);
	
	timely::execute_from_args(std::env::args(), |worker| {
		
		let filename = String::from("./test_raw");
		let inspect = std::env::args().nth(2).expect("Must supply inspect").parse::<bool>().expect("Could not parse inspect");
		
		let index = worker.index();
		let peers = worker.peers();
		let timer = worker.timer();
		
		let edges = bigd_proj::load_graph(&filename, index, peers);
		
		
		println!("{:?}\tLoaded {} edges", timer.elapsed(), edges.len());
		
		let mut probe = timely::dataflow::ProbeHandle::new();
		
		worker.dataflow(|scope| {
			
			let timer = timer.clone();
			

			/*if index == 0{
				let count = edges.len();

				for i in 0 .. count {
					println!("{}{}", edges[i].0,edges[i].1);
				}
			}*/
			let edges = 
			edges
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
		
		while worker.step() {}
		
		
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

/*fn triangles_old<G>(edges: &Collection<G, Edge<Node>>) -> Collection<G, (Node, Node, Node)>
where
	G: timely::dataflow::scopes::Scope,
	G::Timestamp: differential_dataflow::lattice::Lattice+std::hash::Hash+Ord
{
	let edges = edges.filter(|&(src,dst)| src < dst);

	let as_self = edges.arrange_by_self();
	let forward = edges.arrange_by_key();
	let reverse = edges.map_in_place(|x| ::std::mem::swap(&mut x.0, &mut x.1))
					   .arrange_by_key();
	
	let counts = edges.map(|(src, _dst)| src)
					  .arrange_by_self();
	
	let cand_count1 = forward.join_core(&counts, |&src, &dst, &()| Some(((src, dst), 1)));
	let cand_count2 = reverse.join_core(&counts, |&src, &dst, &()| Some(((src, dst), 2)));
	
	let winners = cand_count1.concat(&cand_count2)
							 .reduce(|_srcdst, counts, output| {
								 if counts.len() == 2 {
									 let mut min_cnt = isize::max_value();
									 let mut min_idx = usize::max_value();
									 for &(&idx, cnt) in counts.iter() {
										 if min_cnt > cnt {
											 min_idx = idx;
											 min_cnt = cnt;
										 }
									 }
									 output.push((min_idx, 1));
								 }
							 });
	
	let winners1 = winners.flat_map(|((src,dst), index)| if index == 1 { Some((src, dst)) } else { None })
						  .join_core(&forward, |&src, &dst, &ext| Some(((dst, ext), src)))
						  .join_core(&as_self, |&(dst, ext), &src, &()| Some(((dst, ext), src)))
						  .map(|((dst, ext), src)| (src, dst, ext));
	
	let winners2 = winners.flat_map(|((src,dst), index)| if index == 2 { Some((dst, src)) } else { None })
						  .join_core(&forward, |&dst, &src, &ext| Some(((src, ext), dst)))
						  .join_core(&as_self, |&(src, ext), &dst, &()| Some(((src, ext), dst)))
						  .map(|((src, ext), dst)| (src, dst, ext));
	
	winners1.concat(&winners2)
}
*/
