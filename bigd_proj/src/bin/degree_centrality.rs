extern crate rand;
extern crate timely;
extern crate differential_dataflow;

use differential_dataflow::input::Input;
use differential_dataflow::operators::count::CountTotal;

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

        // create a degree counting differential dataflow
        let (mut input, probe) = worker.dataflow::<u32,_,_>(|scope| {

            // create edge input, count a few ways.
            let (input, edges) = scope.new_collection::<_,isize>();

            let out_degr_distr_in =
            edges
                .map(|(src, _dst)| src);

            let out_degr_distr_out =
            edges
                .map(|(_src, dst)| dst);

            let out_degr_distr = out_degr_distr_in.concat(&out_degr_distr_out)
                                .count_total()
                                //.map(|(_src, cnt)| cnt as usize)
                                .map(|(_src, _cnt)| 0 as usize) // auth h grammh ousiastika einai gia na emfanizei mono to plhthos twn komvwn
                                                                // kai th xrhsimopoioume sto treksimo gia na mhn gemizei to terminal me grammes
                                                                // kai na exei ena mono apotelesma gia ta benchmarks
                                                                // gia swsth leitourgia prepei na svhstei kai na gine uncomment h apo panw
                                .count_total();

            // show us something about the collection, notice when done.
            // mas deixnei gia kathe diaforetiko vathmo pou uparxei posoi komvoi tou grafhmatos ton exoun
            let probe =
            out_degr_distr
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
                        //println!("no changes {}", changes_temp);
                        for _i in 0..changes_temp {
                            let helper = changes.pop_front().unwrap();
                            input.update((helper.0,helper.1),helper.2 as isize);
                            //println!("{:?}",helper);
                        }
                    }           
                }
            }
        }
    }).unwrap();
}