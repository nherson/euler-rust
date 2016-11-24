extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
mod problem4;
mod problem7;
mod problem8;
mod problem9;
mod problem14;

static IMPLEMENTED_PROBLEMS: &'static [i32] = &[4, 7, 8, 9, 14];

const USAGE: &'static str = "
Project Euler problems, solved in Rust

Usage:
  euler solve <n>
  euler list

Options:
  -h --help         Show this screen.
  -p --problem=<p>  Solve problem p [required].
  -s --solved       Print solved problems and exit
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_list: bool,
    cmd_solve: bool,
    arg_n: i32,
}

fn main() {
    // Parse arguments
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    // Handle `euler list`
    if args.cmd_list {
        println!("The following problems are solvable: {:?}", IMPLEMENTED_PROBLEMS)
    }

    // Handle `euler solve <n>`
    // Solvable problems should live in modules and have a run() command that does necessary
    // computation and println! the answer to stdout
    if args.cmd_solve {
        match args.arg_n {
        	4 => problem4::run(),
            7 => problem7::run(),
            8 => problem8::run(),
            9 => problem9::run(),
            14 => problem14::run(),
            _ => println!("Unsolved problem! Use `euler list` for a list of solvable problems.")
        }
    }
}
