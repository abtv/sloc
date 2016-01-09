use std::path::{Path, PathBuf};
use std::fs::{File, read_dir};
use std::io::Read;

static EXTS: &'static [&'static str; 39] = &[".rs", "hs",
                                             ".go", ".rb", ".rbw",
                                             ".java", ".scala", ".clj",
                                             ".js", ".cljs",
                                             ".cpp", ".c", ".h", ".m", ".mm",
                                             ".cs", ".fs", ".vb",
                                             ".py", ".pyc", ".pyd", ".pyo", ".pyw", ".pyz",
                                             ".php", ".phtml", ".php3", ".php4", ".php5", ".phps",
                                             ".pas",
                                             ".lisp", ".cl",
                                             ".tcl", ".lua", 
                                             ".pl", ".pm", ".t", ".pod"];

fn is_src(file: &str) -> bool {
    EXTS.iter()
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
    match f {
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
