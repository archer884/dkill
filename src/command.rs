use std::env;
use std::path::PathBuf;

pub enum Command {
    List(PathBuf),
    Clean(PathBuf),
    Invalid,
}

pub fn read() -> Command {
    let args: Vec<_> = env::args().skip(1).collect();
    match (args.get(0), args.get(1)) {
        (Some(ref path), Some(ref force)) if is_force_arg(force) => Command::Clean(path.into()),
        (Some(ref path), _) => Command::List(path.into()),
        _ => Command::Invalid,
    }
}

fn is_force_arg(arg: &str) -> bool {
    arg == "-f" || arg == "--force"
}