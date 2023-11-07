pub const HELP: &str = "\
Usage: aoc2021 -d <day> [-p <part>] [-e] [-i <input>]

The CLI arguments allowed.

Options:
  -d, --day         specifies the day
  -p, --part        specifies the part
  -e, --example     use the day's example input from examples/
  -i, --input       specify an alternate input file
  -h, --help        display usage information
";

pub const INPUT_CONFLICT: &str = "\
Error: -i/--input and -e/--example can't be used together.
";

/// The CLI arguments allowed.
pub struct Args {
    /// specifies the day (255 runs all parts)
    pub day: u8,
    /// specifies the part
    pub part: u8,
    /// use the day's example input from examples/
    pub example: bool,
    /// specify an alternate input file
    pub input: Option<String>,
}

pub fn parse_args() -> Result<Args, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{HELP}");
        std::process::exit(0);
    }

    let args = Args {
        day: pargs.value_from_str(["-d", "--day"])?,
        part: pargs.value_from_str(["-p", "--part"]).or(Ok(1))?,
        example: pargs.contains(["-e", "--example"]),
        input: pargs.opt_value_from_str(["-i", "--input"])?,
    };

    if pargs.contains(["-e", "--example"]) && pargs.contains(["-i", "--input"]) {
        print!("{INPUT_CONFLICT}");
        std::process::exit(1);
    }

    Ok(args)
}
