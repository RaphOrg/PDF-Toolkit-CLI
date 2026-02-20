# PDF-Toolkit-CLI

Rust workspace providing `pdfx` CLI for basic PDF metadata and encryption operations.

## Build

```bash
cargo build --release
```

## Usage

### Metadata

Get a value from the PDF Info dictionary:

```bash
pdfx meta get -i input.pdf -k Title
```

Set a value in the PDF Info dictionary:

```bash
pdfx meta set -i input.pdf -o output.pdf -k Title -v "My Title"
```

### Crypto

Encrypt:

```bash
pdfx crypto encrypt -i input.pdf -o encrypted.pdf --user-password secret
```

Decrypt:

```bash
pdfx crypto decrypt -i encrypted.pdf -o decrypted.pdf --password secret
```
