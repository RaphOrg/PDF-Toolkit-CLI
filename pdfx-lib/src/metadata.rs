use anyhow::{anyhow, Context, Result};
use lopdf::{Document, Object, ObjectId, StringFormat};

/// Return a clone of the Info dictionary if present.
pub fn get_info_dict(doc: &Document) -> Result<Option<lopdf::Dictionary>> {
    let trailer = &doc.trailer;

    let info = match trailer.get(b"Info") {
        Ok(obj) => obj,
        Err(_) => return Ok(None),
    };

    let dict = match info {
        Object::Reference(oid) => doc
            .get_object(*oid)
            .context("resolve Info reference")?
            .as_dict()
            .context("Info is not a dict")?
            .clone(),
        Object::Dictionary(d) => d.clone(),
        _ => return Err(anyhow!("Info is not a dictionary/reference")),
    };

    Ok(Some(dict))
}

pub fn get_info_value(doc: &Document, key: &[u8]) -> Result<Option<String>> {
    let Some(info) = get_info_dict(doc)? else {
        return Ok(None);
    };

    let Some(obj) = info.get(key).ok() else {
        return Ok(None);
    };

    Ok(Some(object_to_string(doc, obj)?))
}

pub fn set_info_value(doc: &mut Document, key: Vec<u8>, value: String) -> Result<()> {
    let info_oid = ensure_info_object(doc)?;

    let mut dict = doc
        .get_object(info_oid)
        .context("get Info object")?
        .as_dict()
        .context("Info object not dict")?
        .clone();

    dict.set(
        key,
        Object::String(value.into_bytes(), StringFormat::Literal),
    );
    doc.objects.insert(info_oid, Object::Dictionary(dict));

    Ok(())
}

fn ensure_info_object(doc: &mut Document) -> Result<ObjectId> {
    // Avoid borrowing doc.trailer across new_object_id()/objects insert.
    let existing = doc.trailer.get(b"Info").ok().cloned();

    match existing {
        Some(Object::Reference(oid)) => Ok(oid),
        Some(Object::Dictionary(d)) => {
            let oid = doc.new_object_id();
            doc.objects.insert(oid, Object::Dictionary(d));
            doc.trailer.set(b"Info", Object::Reference(oid));
            Ok(oid)
        }
        Some(_) => {
            let oid = doc.new_object_id();
            doc.objects
                .insert(oid, Object::Dictionary(lopdf::Dictionary::new()));
            doc.trailer.set(b"Info", Object::Reference(oid));
            Ok(oid)
        }
        None => {
            let oid = doc.new_object_id();
            doc.objects
                .insert(oid, Object::Dictionary(lopdf::Dictionary::new()));
            doc.trailer.set(b"Info", Object::Reference(oid));
            Ok(oid)
        }
    }
}

fn object_to_string(doc: &Document, obj: &Object) -> Result<String> {
    let resolved = match obj {
        Object::Reference(oid) => doc.get_object(*oid).context("resolve ref")?,
        _ => obj,
    };

    match resolved {
        Object::String(bytes, _) => Ok(String::from_utf8_lossy(bytes).to_string()),
        Object::Name(n) => Ok(String::from_utf8_lossy(n).to_string()),
        Object::Integer(i) => Ok(i.to_string()),
        Object::Real(f) => Ok(f.to_string()),
        Object::Boolean(b) => Ok(b.to_string()),
        Object::Null => Ok("null".to_string()),
        other => Ok(format!("{other:?}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get_info_value() -> Result<()> {
        let mut doc = Document::with_version("1.5");
        set_info_value(&mut doc, b"Title".to_vec(), "Hello".to_string())?;
        let title = get_info_value(&doc, b"Title")?;
        assert_eq!(title.as_deref(), Some("Hello"));
        Ok(())
    }
}
