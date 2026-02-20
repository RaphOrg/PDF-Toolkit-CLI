use anyhow::Result;
use clap::{Args, Parser, Subcommand};

use crate::commands;

#[derive(Parser, Debug)]
#[command(name = "pdfx", version, about = "PDF Toolkit CLI")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Inspect or modify PDF metadata (Info dictionary)
    Meta {
        #[command(subcommand)]
        command: MetaCmd,
    },
    /// Encrypt/decrypt PDFs
    Crypto {
        #[command(subcommand)]
        command: CryptoCmd,
    },
}

#[derive(Subcommand, Debug)]
enum MetaCmd {
    /// Print a metadata value
    Get(MetaGetArgs),
    /// Set a metadata value
    Set(MetaSetArgs),
}

#[derive(Args, Debug)]
struct MetaGetArgs {
    #[arg(short, long)]
    input: String,
    /// Metadata key, e.g. Title, Author
    key: String,
}

#[derive(Args, Debug)]
struct MetaSetArgs {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
    /// Metadata key, e.g. Title, Author
    key: String,
    /// Value to write
    value: String,
}

#[derive(Subcommand, Debug)]
enum CryptoCmd {
    /// Encrypt a PDF
    Encrypt(CryptoEncryptArgs),
    /// Decrypt a PDF
    Decrypt(CryptoDecryptArgs),
}

#[derive(Args, Debug)]
struct CryptoEncryptArgs {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
    /// User password
    #[arg(long)]
    user_password: String,
    /// Owner password (defaults to user password)
    #[arg(long)]
    owner_password: Option<String>,
}

#[derive(Args, Debug)]
struct CryptoDecryptArgs {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
    /// Password (optional; if omitted, tries empty password)
    #[arg(long)]
    password: Option<String>,
}

impl Cli {
    pub fn dispatch(self) -> Result<()> {
        match self.command {
            Command::Meta { command } => match command {
                MetaCmd::Get(args) => commands::meta::get(args.input, args.key),
                MetaCmd::Set(args) => {
                    commands::meta::set(args.input, args.output, args.key, args.value)
                }
            },
            Command::Crypto { command } => match command {
                CryptoCmd::Encrypt(args) => commands::crypto::encrypt(
                    args.input,
                    args.output,
                    args.user_password,
                    args.owner_password,
                ),
                CryptoCmd::Decrypt(args) => {
                    commands::crypto::decrypt(args.input, args.output, args.password)
                }
            },
        }
    }
}
