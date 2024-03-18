// implement a processor that transforms a list of transactions in a supplied CSV into a list of accounts and their current state.
mod tx;
mod engine;

use engine::Engine;
use tx::Tx;

use anyhow::Result;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use clap::Parser;

fn stream_records_from_reader<'r>(
    reader: &'r mut dyn Read,
) -> impl Iterator<Item = Result<Tx>> + 'r {
    csv::Reader::from_reader(reader)
        .into_deserialize()
        .map(|r| r.map_err(Into::into))
}

fn write_cols(writer: &mut csv::Writer<&mut dyn Write>) -> Result<()> {
    writer.write_record(&[
        "client",
        "available",
        "held",
        "total",
        "locked",
    ])?;

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut file = File::open(&args.file)?;
    let tx_stream = stream_records_from_reader(&mut file);
    let mut stdout = io::stdout();
    let mut writer = csv::Writer::from_writer(&mut stdout as &mut dyn Write);
    let mut engine = Engine::new();

    for tx in tx_stream {
        if let Err(err) = engine.apply(tx?) {
            eprintln!("{}", err);
        }
    }

    write_cols(&mut writer)?;
    for account in engine.accounts() {
        writer.serialize(account)?;
    }

    writer.flush()?;
    Ok(())
}
