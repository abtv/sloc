mod files;
use files::get_files;
mod counting;
use counting::{Stats, Counter, get_counters, get_stats};

fn show_stats(stats: &Stats) {
    println!("Total files: {}", stats.files_count);
    println!("Total loc: {}", stats.total_loc);
    println!("Empty loc: {}", stats.empty_loc);
}

fn show_counters(counters: &Vec<Counter>) {
    const SHOW_COUNT: usize = 10;
    let len = counters.len();
    let max = if len < SHOW_COUNT {
        len
    }
    else{
        SHOW_COUNT
    };
    if max > 0 {
        println!("{} biggest files:", max);
    }
    let mut i = 0;
    while i < max { 
        println!("{position}. {total_loc} loc in {file_name}",
                 position = (i + 1),
                 total_loc = counters[i].total_loc,
                 file_name = counters[i].file);
        i += 1;
    } 
}

fn main() {
    println!("Source lines of code program...");

    //TODO Implement a producer (get_files) and a consumer (get_counters and then get_stats)
    //in parallel, say with 2 threads
    let mut files: Vec<String> = Vec::new();
    get_files(".", &mut files);
    let counters = get_counters(files);
    let stats = get_stats(&counters);
    
    show_counters(&counters);
    show_stats(&stats);
}

#[test]
fn get_files_test() {
    let mut files: Vec<String> = Vec::new();
    get_files("./test_data/", &mut files);
    assert_eq!(2, files.len());
}

#[test]
fn get_counters_test() {
    let mut files: Vec<String> = Vec::new();
    get_files("./test_data/", &mut files);
    let counters = get_counters(files);
    assert_eq!(2, counters.len());
}

#[test]
fn get_stats_test(){
    let mut files: Vec<String> = Vec::new();
    get_files("./test_data/", &mut files); 
    let counters = get_counters(files);
    let stats = get_stats(&counters);

    assert_eq!(2, stats.files_count);
    assert_eq!(10, stats.total_loc);
    assert_eq!(12, stats.empty_loc);
}
