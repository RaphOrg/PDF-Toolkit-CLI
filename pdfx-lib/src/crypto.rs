use anyhow::{anyhow, Context, Result};
use lopdf::Document;

/// Encrypt a PDF document.
///
/// lopdf v0.34 can *decrypt* encrypted PDFs, but does not expose a stable high-level
/// `Document::encrypt` API. For now, we return a clear error.
///
/// This keeps the CLI contract stable while avoiding a false sense of security.
pub fn encrypt(
    _input_path: &str,
    _output_path: &str,
    _user_password: &str,
    _owner_password: Option<&str>,
) -> Result<()> {
    Err(anyhow!(
        "encrypt is not supported by current lopdf version (v0.34); decrypt is supported"
    ))
}

/// Decrypt a PDF document using lopdf.
///
/// If password is None, tries empty password.
pub fn decrypt(input_path: &str, output_path: &str, password: Option<&str>) -> Result<()> {
    let mut doc = Document::load(input_path).context("load input pdf")?;

    if doc.is_encrypted() {
        let pw = password.unwrap_or("");
        doc.decrypt(pw)
            .map_err(|e| anyhow!("decrypt failed: {e:?}"))?;

        // lopdf's decrypt removes the trailer Encrypt entry.
        // If upstream behavior changes, we'd want a regression test.
    }

    doc.save(output_path).context("save decrypted pdf")?;
    Ok(())
}

/// Helper to check encryption flag in a saved PDF (used in tests).
pub fn is_encrypted_file(path: &str) -> Result<bool> {
    let doc = Document::load(path).context("load pdf")?;
    Ok(doc.is_encrypted())
}
