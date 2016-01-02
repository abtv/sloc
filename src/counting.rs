use files::{read_file};

pub struct Counter{
    pub file: String,
    pub total_loc: u64,
    pub empty_loc: u64
}

pub struct Stats{
    pub files_count: u64,
    pub total_loc: u64,
    pub empty_loc: u64
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

pub fn get_counters(files: Vec<String>) -> Vec<Counter> {
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

pub fn get_stats(counters: &Vec<Counter>) -> Stats {
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
