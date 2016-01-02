mod files;
use files::get_files;
mod counting;
use counting::{Stats, Counter, get_counters, get_stats};

fn show_stats(stats: Stats) {
    println!("Total files: {}", stats.files_count);
    println!("Total loc: {}", stats.total_loc);
    println!("Empty loc: {}", stats.empty_loc);
}

fn show_counters(counters: &Vec<Counter>) {
    let show_count = 10;
    let len = counters.len();
    let max = if show_count > len {len} else {show_count};
    let mut i = 0;
    if max > 0 {
        println!("{} biggest files:", max);
    }
    while i < max { 
        println!("{}. {} loc in {}", (i + 1), counters[i].total_loc, counters[i].file);
        i += 1;
    } 
}

fn main() {
    println!("Source lines of code program...");
    
    let files = get_files(".");
    let counters = get_counters(files);

    show_counters(&counters);

    let stats = get_stats(&counters);
    show_stats(stats);
}

#[test]
#[ignore]
fn it_works() {
  assert_eq!(4, 2*2 + 1);
}
