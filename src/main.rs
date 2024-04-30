use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64Subcommand, Opts,
    SubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };

            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64Subcommand::Encode(opts) => {
                process_encode(opts.input.as_str(), opts.format)?;
                println!("{:?}", opts);
            }

            Base64Subcommand::Decode(opts) => {
                process_decode(opts.input.as_str(), opts.format)?;
                println!("{:?}", opts);
            }
        },
    }

    Ok(())
}
