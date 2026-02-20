use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use lopdf::Document;

#[derive(Parser, Debug)]
#[command(name = "pdfx")]
#[command(about = "PDF Toolkit CLI (metadata + encryption)", long_about = None)]
struct Cli {
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
    /// Get a metadata value by key
    Get(MetaGetArgs),
    /// Set a metadata value by key
    Set(MetaSetArgs),
}

#[derive(Args, Debug)]
struct MetaGetArgs {
    /// Input PDF
    #[arg(short, long)]
    input: String,
    /// Metadata key name (e.g. Title, Author, Subject)
    #[arg(short, long)]
    key: String,
}

#[derive(Args, Debug)]
struct MetaSetArgs {
    /// Input PDF
    #[arg(short, long)]
    input: String,
    /// Output PDF
    #[arg(short, long)]
    output: String,
    /// Metadata key name (e.g. Title, Author, Subject)
    #[arg(short, long)]
    key: String,
    /// New metadata value
    #[arg(short, long)]
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Meta { command } => match command {
            MetaCmd::Get(args) => meta_get(args),
            MetaCmd::Set(args) => meta_set(args),
        },
        Command::Crypto { command } => match command {
            CryptoCmd::Encrypt(args) => crypto_encrypt(args),
            CryptoCmd::Decrypt(args) => crypto_decrypt(args),
        },
    }
}

fn meta_get(args: MetaGetArgs) -> Result<()> {
    let doc = Document::load(&args.input).context("load input pdf")?;
    let value = pdfx_lib::get_info_value(&doc, args.key.as_bytes())?;

    if let Some(v) = value {
        println!("{v}");
    }

    Ok(())
}

fn meta_set(args: MetaSetArgs) -> Result<()> {
    let mut doc = Document::load(&args.input).context("load input pdf")?;
    pdfx_lib::set_info_value(&mut doc, args.key.as_bytes().to_vec(), args.value)?;
    doc.save(&args.output).context("save output pdf")?;
    Ok(())
}

fn crypto_encrypt(args: CryptoEncryptArgs) -> Result<()> {
    pdfx_lib::encrypt(
        &args.input,
        &args.output,
        &args.user_password,
        args.owner_password.as_deref(),
    )
}

fn crypto_decrypt(args: CryptoDecryptArgs) -> Result<()> {
    pdfx_lib::decrypt(&args.input, &args.output, args.password.as_deref())
}
