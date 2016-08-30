extern crate docopt;
extern crate rustc_serialize;

use std::env;
use std::fmt;
use std::io;
use std::process;

use docopt::Docopt;

mod cmd;
mod util;

macro_rules! wout {
    ($($arg:tt)*) => ({
        use std::io::Write;
        (writeln!(&mut ::std::io::stdout(), $($arg)*)).unwrap();
    });
}

macro_rules! werr {
    ($($arg:tt)*) => ({
        use std::io::Write;
        (writeln!(&mut ::std::io::stderr(), $($arg)*)).unwrap();
    });
}

macro_rules! fail {
    ($e:expr) => (Err(::std::convert::From::from($e)));
}

macro_rules! command_list {
    () => (
"
    new         Runs the new command.
"
    )
}

static USAGE: &'static str = concat!("
Usage:
    {{name}} <command> [<args>...]
    {{name}} [options]

Options:
    --list           List all commands available.
    -h, --help       Display this message
    --version    Print version info and exit

Commands:", command_list!());

#[derive(RustcDecodable)]
struct Args {
    arg_command: Option<Command>,
    flag_list: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.options_first(true)
                                           .version(Some(util::version()))
                                           .decode())
                            .unwrap_or_else(|e| e.exit());
    if args.flag_list {
        wout!(concat!("Installed commands:", command_list!()));
        return;
    }
    match args.arg_command {
        None => {
            werr!(concat!(
                "{{name}} is <your description here>
Please choose one of the following commands:",
                command_list!()));
            process::exit(0);
        }
        Some(cmd) => {
            match cmd.run() {
                Ok(()) => process::exit(0),
                Err(CliError::Flag(err)) => err.exit(),
                Err(CliError::Io(ref err))
                        if err.kind() == io::ErrorKind::BrokenPipe => {
                    process::exit(0);
                }
                Err(CliError::Io(err)) => {
                    werr!("{}", err);
                    process::exit(1);
                }
                Err(CliError::Other(msg)) => {
                    werr!("{}", msg);
                    process::exit(1);
                }
            }
        }
    }
}

#[derive(Debug, RustcDecodable)]
enum Command {
New
}

impl Command {
    fn run(self) -> CliResult<()> {
        let argv: Vec<_> = env::args().map(|v| v.to_owned()).collect();
        let argv: Vec<_> = argv.iter().map(|s| &**s).collect();
        let argv = &*argv;
        match self {
            Command::New => cmd::new::run(argv),
        }
    }
}

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug)]
pub enum CliError {
    Flag(docopt::Error),
    Io(io::Error),
    Other(String)
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Flag(ref e) => { e.fmt(f) }
            CliError::Io(ref e) => { e.fmt(f) }
            CliError::Other(ref s) => { f.write_str(&**s) }
        }
    }
}

impl From<docopt::Error> for CliError {
    fn from(err: docopt::Error) -> CliError {
        CliError::Flag(err)
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<String> for CliError {
    fn from(err: String) -> CliError {
        CliError::Other(err)
    }
}

impl<'a> From<&'a str> for CliError {
    fn from(err: &'a str) -> CliError {
        CliError::Other(err.to_owned())
    }
}


