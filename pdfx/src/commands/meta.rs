use anyhow::{Context, Result};
use lopdf::Document;

pub fn get(input: String, key: String) -> Result<()> {
    let doc = Document::load(&input).context("load input pdf")?;
    let value = pdfx_lib::get_info_value(&doc, key.as_bytes())?;

    if let Some(v) = value {
        println!("{v}");
    }

    Ok(())
}

pub fn set(input: String, output: String, key: String, value: String) -> Result<()> {
    let mut doc = Document::load(&input).context("load input pdf")?;
    pdfx_lib::set_info_value(&mut doc, key.as_bytes().to_vec(), value)?;
    doc.save(&output).context("save output pdf")?;
    Ok(())
}
