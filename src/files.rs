use std::path::{Path, PathBuf};
use std::fs::{File, read_dir};
use std::io::Read;

fn is_src(file: &str) -> bool {
    let exts = [".rs", ".clj", ".cljs", ".scala", ".java", ".js", ".cpp", ".c", ".h", ".cs", ".fs"];
    exts.iter()
    .filter(|x| file.ends_with(*x))
    .count() > 0
}

fn get_path(pb: PathBuf) -> String {
    let path = pb.as_path().to_str();
    match path {
        Some(s) => s,
        None    => ""
    }.to_string()
}

pub fn get_files(folder: &str) -> Vec<String> {
    let mut files = Vec::new();

    match read_dir(&Path::new(&folder)) {
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

pub fn read_file(file_name: &str) -> Option<String> {
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
