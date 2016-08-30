use CliResult;
use util;

static USAGE: &'static str = "
Runs the new command

Usage:
   {{name}} new [<input>...]

Common options:
    -h, --help    Display this message
";


#[derive(RustcDecodable)]
struct Args {
    arg_input: Vec<String>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = try!(util::get_args(USAGE, argv));
    args.new()
}

impl Args {
    fn new(&self) -> CliResult<()> {
        print!("New command run with arguments: ");
        for token in self.arg_input.iter() {
            print!("{}", token);
        }
        println!("");
        Ok(())
    }
}
