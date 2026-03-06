use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "rgrep",
    about = "The fzf line implementation for grep in my own style"
)]
struct Cli {
    #[arg(short, long)]
    recursive: bool,

    #[arg(short, long)]
    ignore_case: bool,

    // ignore evrything after the flags
    #[arg(last = true, allow_hyphen_values = true)]
    pattern: Option<String>,
}

fn main() {
    let args = Cli::parse();

    if args.recursive {
        println!("Trigger recursive tui");
    }
    if args.ignore_case {
        println!("Trigger ignore_case tui");
    }
}
