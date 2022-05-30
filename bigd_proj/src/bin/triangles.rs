extern crate rand;
extern crate timely;
extern crate differential_dataflow;

use differential_dataflow::input::Input;
use differential_dataflow::operators::count::CountTotal;
use crate::differential_dataflow::operators::Join;

// gia na treksoun prepei na uparxoun ta arxeia random_graph.txt kai edges_to_change.txt anagkastika

fn main() {

    let mut args = std::env::args();
    args.next();

    let batch: u32 = args.next().unwrap().parse().unwrap();
    let inspect: bool = args.next().unwrap() == "inspect";
    
    let filename = String::from("random_graph.txt");
    let (nums,_nodes) = bigd_proj::getnodes(filename);
    
    timely::execute_from_args(std::env::args().skip(2), move |worker| {

        
        let index = worker.index();
        let peers = worker.peers();

        let (mut input, probe) = worker.dataflow::<u32,_,_>(|scope| {

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

            let probe =
            triangles
                .map(|(src, _cnt)| src)
                .filter(move |_| inspect)
                .inspect(|x| println!("observed: {:?}", x))
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