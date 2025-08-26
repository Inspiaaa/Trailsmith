use crate::error_messages;
use anyhow::Context;
use log::info;
use std::borrow::Cow;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use xml::attribute::{Attribute, OwnedAttribute};
use xml::reader::XmlEvent;
use xml::{EmitterConfig, EventReader};

use crate::error_messages::GPX_SERIALIZE_ERROR;
use xml::writer::XmlEvent as WriterEvent;

pub fn minify(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    info!("Writing output to {}...", output_path.display());

    let input_file =
        File::open(input_path).with_context(|| error_messages::INPUT_FILE_READ_ERROR)?;
    let output_file =
        File::create(output_path).with_context(|| error_messages::OUTPUT_FILE_CREATION_ERROR)?;

    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    let parser = EventReader::new(reader);

    let mut emitter = EmitterConfig::new()
        .perform_indent(false)
        .create_writer(&mut writer);

    for event in parser {
        let event = event.with_context(|| error_messages::GPX_PARSE_ERROR)?;
        let result = match event {
            XmlEvent::StartElement {
                name,
                attributes,
                namespace,
            } => {
                let cloned_attributes: Vec<Attribute> =
                    attributes.iter().map(OwnedAttribute::borrow).collect();
                emitter.write(WriterEvent::StartElement {
                    name: name.borrow(),
                    attributes: Cow::Borrowed(cloned_attributes.as_slice()),
                    namespace: Cow::Owned(namespace),
                })
            }
            XmlEvent::EndElement { name } => emitter.write(WriterEvent::EndElement {
                name: Some(name.borrow()),
            }),
            XmlEvent::Characters(text) => emitter.write(WriterEvent::Characters(&text)),
            XmlEvent::CData(text) => emitter.write(WriterEvent::CData(&text)),
            XmlEvent::ProcessingInstruction { name, data } => {
                emitter.write(WriterEvent::ProcessingInstruction {
                    name: &name,
                    data: data.as_deref(),
                })
            }
            XmlEvent::StartDocument {
                version,
                encoding,
                standalone,
            } => emitter.write(WriterEvent::StartDocument {
                version,
                encoding: Some(&encoding),
                standalone,
            }),
            XmlEvent::EndDocument => Ok(()),
            XmlEvent::Whitespace(_) => Ok(()),
            XmlEvent::Comment(_) => Ok(()),
            XmlEvent::Doctype { .. } => Ok(()),
        };

        result.with_context(|| GPX_SERIALIZE_ERROR)?;
    }

    writer
        .flush()
        .with_context(|| error_messages::OUTPUT_FILE_WRITE_ERROR)?;

    Ok(())
}
