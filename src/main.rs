use crate::hashes::Hashes;
use anyhow::Context;
use clap::{Parser, Subcommand};
use serde::Deserialize;
use serde_json::{self, Value};
use std::path::PathBuf;

pub mod hashes;

#[derive(Debug, Clone, Deserialize)]
struct Torrent {
    announce: String,
    info: Info,
}

#[derive(Debug, Clone, Deserialize)]
struct Info {
    name: String,
    #[serde(rename = "piece length")]
    plength: usize,
    pieces: Hashes,
    #[serde(flatten)]
    keys: Keys,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum Keys {
    SingleFile { length: usize },
    MultiFile { files: File },
}

#[derive(Debug, Clone, Deserialize)]
struct File {
    length: usize,
    path: Vec<String>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    commend: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Decode { value: String },
    Info { torrent: PathBuf },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.commend {
        Command::Decode { value } => {
            let value = decode_bencoded_value(&value).0;
            println!("{value}");
        }
        Command::Info { torrent } => {
            let dot_torrent = std::fs::read(torrent).context("read torrent file")?;
            let t =
                serde_bencode::from_bytes::<Torrent>(&dot_torrent).context("parse torrent file")?;
            println!("Tracker URL: {}", t.announce);
            if let Keys::SingleFile { length } = t.info.keys {
                println!("Length: {length}");
            } else {
                unimplemented!();
            }
        }
    }

    anyhow::Ok(())
}

fn decode_bencoded_value(encoded_value: &str) -> (Value, &str) {
    match encoded_value.chars().next() {
        Some('i') => {
            if let Some((n, rest)) =
                encoded_value
                    .split_at(1)
                    .1
                    .split_once('e')
                    .and_then(|(digits, rest)| {
                        let n = digits.parse::<i64>().ok()?;
                        Some((n, rest))
                    })
            {
                return (n.into(), rest);
            }
        }
        Some('d') => {
            let mut dict = serde_json::Map::new();
            let mut rest = encoded_value.split_at(1).1;
            while !rest.is_empty() && !rest.starts_with('e') {
                let (k, reminder) = decode_bencoded_value(rest);
                let k = match k {
                    serde_json::Value::String(k) => k,
                    k => {
                        panic!("dict keys must be strings, not {k:?}");
                    }
                };
                let (v, reminder) = decode_bencoded_value(reminder);
                dict.insert(k, v);
                rest = reminder;
            }
            return (dict.into(), &rest[1..]);
        }
        Some('l') => {
            let mut val = Vec::new();
            let mut rest = encoded_value.split_at(1).1;
            while !rest.is_empty() && !rest.starts_with('e') {
                let (v, reminder) = decode_bencoded_value(rest);
                val.push(v);
                rest = reminder;
            }
            return (val.into(), &rest[1..]);
        }
        Some('0'..='9') => {
            if let Some((len, rest)) = encoded_value.split_once(':') {
                if let Ok(len) = len.parse::<usize>() {
                    return (rest[..len].to_string().into(), &rest[len..]);
                }
            }
        }
        _ => {}
    }
    panic!("Unhandled encoded value: {}", encoded_value)
}
