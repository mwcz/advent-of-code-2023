use argh::FromArgs;
use std::fs::read_to_string;

fn main() {
    let args: Args = argh::from_env();

    dbg!(&args);

    let input_file = if args.input.is_some() {
        args.input.unwrap()
    } else if args.example {
        format!("./examples/d{}", args.day)
    } else {
        format!("./inputs/d{}", args.day)
    };

    dbg!(&input_file);

    let input = read_to_string(input_file).expect("couldn't read input file");

    println!("{input}");
}

/// The CLI arguments allowed.
#[derive(FromArgs, Debug)]
struct Args {
    /// specifies the day.
    #[argh(option, short = 'd')]
    day: u8,

    /// use the day's example input from examples/
    #[argh(switch, short = 'e')]
    example: bool,

    /// specify an alternate input file.
    #[argh(option, short = 'i')]
    input: Option<String>,
}
