// implement a processor that transforms a list of transactions in a supplied CSV into a list of accounts and their current state.
mod tx;
mod engine;

use engine::Engine;
use tx::Tx;

use anyhow::Result;
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
fn main() {
    unimplemented!()
}
