use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

fn err_quit(msg: &str) {
    writeln!(io::stderr(), "{}", msg).unwrap();
    process::exit(1);
}

fn err_sys(msg: &str, arg: &str) {
    writeln!(io::stderr(), "{}: {}", msg, arg).unwrap();
    writeln!(io::stderr(), "{}", io::Error::last_os_error()).unwrap();
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        err_quit("usage: ls directory name");
    }

    let path = &args[1];
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("{}", entry.file_name().to_string_lossy());
                }
            }
        }
        Err(e) => {
            err_sys("can't open", path);
        }
    }
}
