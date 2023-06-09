use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, required = true)]
    program: String,

    #[arg(short, long, default_value = "false")]
    debug: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("Running program: {}", cli.program);

    // read the file
    let code = std::fs::read_to_string(cli.program).expect("Unable to read file");

    // parse the code
    let program = lmc_assembly::parse(&code, cli.debug).expect("Unable to parse program");

    // assemble the program
    let program = lmc_assembly::assemble(program).expect("Unable to assemble program");

    // run the program
    lmc_assembly::run(program, &mut lmc_assembly::DefaultIO, cli.debug)
        .expect("Unable to run program");

    println!("\nProgram finished");
}
