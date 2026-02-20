use anyhow::Result;
use lopdf::{Dictionary, Document, Object, Stream};

fn build_min_pdf(path: &str) -> Result<()> {
    let mut doc = Document::with_version("1.5");

    let pages_id = doc.new_object_id();
    let page_id = doc.new_object_id();
    let content_id = doc.new_object_id();
    let catalog_id = doc.new_object_id();

    let content = Stream::new(Dictionary::new(), Vec::new());
    doc.objects.insert(content_id, Object::Stream(content));

    let mut pages = Dictionary::new();
    pages.set("Type", "Pages");
    pages.set("Kids", vec![Object::Reference(page_id)]);
    pages.set("Count", 1);
    doc.objects.insert(pages_id, Object::Dictionary(pages));

    let mut page = Dictionary::new();
    page.set("Type", "Page");
    page.set("Parent", Object::Reference(pages_id));
    page.set("MediaBox", vec![0.into(), 0.into(), 200.into(), 200.into()]);
    page.set("Contents", Object::Reference(content_id));
    doc.objects.insert(page_id, Object::Dictionary(page));

    let mut catalog = Dictionary::new();
    catalog.set("Type", "Catalog");
    catalog.set("Pages", Object::Reference(pages_id));
    doc.objects.insert(catalog_id, Object::Dictionary(catalog));

    doc.trailer.set("Root", Object::Reference(catalog_id));
    doc.save(path)?;
    Ok(())
}

fn main() -> Result<()> {
    let in_path = "/tmp/in.pdf";
    let out_path = "/tmp/out.pdf";

    build_min_pdf(in_path)?;

    // Use the same library functions that the CLI uses.
    let mut doc = Document::load(in_path)?;
    pdfx_lib::set_info_value(&mut doc, b"Title".to_vec(), "MyTitle".to_string())?;
    doc.save(out_path)?;

    let doc2 = Document::load(out_path)?;
    let title = pdfx_lib::get_info_value(&doc2, b"Title")?.unwrap_or_default();
    println!("title={}", title);

    Ok(())
}
