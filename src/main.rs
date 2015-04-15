use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::fs::{self};

struct Counter{
    file: String,
    total_loc: u32,
    empty_loc: u32
}

fn is_src(file: &str) -> bool {
    let exts = [".rs", ".clj", ".scala", ".java", ".js", ".cpp", ".c", ".h", ".cs", ".fs"];
    exts.iter()
        .filter(|x| file.ends_with(*x))
        .count() > 0
}

fn get_files(folder: &str) -> Vec<String> {
    let mut files = Vec::new();
    
    match fs::read_dir(&Path::new(&folder)) {
        Err(_)    => (),
        Ok(paths) => for path in paths {
            let file = get_path(path.unwrap().path());
            if is_src(&file){
                files.push(file.clone());
            }
            
            let sub_files = get_files(&file);
            for sf in sub_files {
                files.push(sf);
            }
        }
    }

    files
}

fn read_file(file_name: &str) -> Option<String> {
    let path = Path::new(&file_name);

    let f = File::open(&path);
    match f{
        Err(_)       => None,
        Ok(mut file) => {
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(_) => None,
                Ok(_)  => Some(s)
            }
        }
    }
}

fn get_loc(file: &str, src_txt: String) -> Counter {
    let mut total_loc = 0;
    let mut empty_loc = 0;
    let chunks = src_txt.split("\n");
    for chunk in chunks {
        if chunk != "" {
            total_loc += 1;
            let chars: Vec<char> = chunk.chars().collect();
            if chars.len() <= 1 {
                empty_loc += 1;
            }
        }
    }

    Counter{file: file.to_string(), total_loc: total_loc, empty_loc: empty_loc}
}

fn get_file_loc(file_name: &str) -> Option<Counter> {
    let src_txt = read_file(file_name);
    match src_txt {
        Some(s) => Some(get_loc(file_name, s)),
        None    => None
    }
}

fn get_path(pb: PathBuf) -> String {
    let path = pb.as_path().to_str();
    match path {
        Some(s) => s,
        None    => ""
    }.to_string()
}

fn get_counters(files: Vec<String>) -> Vec<Counter> {
    let mut counters: Vec<_> = files.iter()
        .map(|x| get_file_loc(&x))
        .filter(|x| match *x {
            Some(_) => true,
            None    => false
        })
        .map(|x| x.unwrap())
        .collect();

    counters.sort_by(|a, b| b.total_loc.cmp(&a.total_loc));
    
    counters
}

struct Stats{
    files_count: u32,
    total_loc: u32,
    empty_loc: u32
}

fn get_stats(counters: &Vec<Counter>) -> Stats {
    let mut files_count = 0;
    let mut total_loc = 0;
    let mut empty_loc = 0;

    for counter in counters {
        files_count += 1;
        total_loc += counter.total_loc;
        empty_loc += counter.empty_loc;
    }

    Stats{files_count: files_count, total_loc: total_loc, empty_loc: empty_loc}
}

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
