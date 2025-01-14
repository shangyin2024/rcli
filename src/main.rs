use clap::Parser;
use rcli::{process_csv, process_genpass, Commands, Opts};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.command {
        Commands::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            // csv_convert(&opts.input, &output, opts.format)?;
            process_csv(&opts.input, output, opts.format)?;
        }
        Commands::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
    }

    Ok(())
}

// test
#[test]
fn test() {
    println!("test");
}
