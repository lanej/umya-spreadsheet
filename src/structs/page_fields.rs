// pageFields
use crate::reader::driver::*;
use crate::structs::Field;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use thin_vec::ThinVec;

#[derive(Clone, Default, Debug)]
pub struct PageFields {
    list: ThinVec<Field>,
}
impl PageFields {
    #[inline]
    pub fn get_list(&self) -> &[Field] {
        &self.list
    }

    #[inline]
    pub fn get_list_mut(&mut self) -> &mut ThinVec<Field> {
        &mut self.list
    }

    #[inline]
    pub fn add_list_mut(&mut self, value: Field) -> &mut Self {
        self.list.push(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"field" {
                    let mut obj = Field::default();
                    obj.set_attributes(reader, e);
                    self.add_list_mut(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"pageFields" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "pageFields")
        );
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // pageFields
        write_start_tag(
            writer,
            "pageFields",
            vec![("count", &self.list.len().to_string())],
            false,
        );

        // field elements
        for field in &self.list {
            field.write_to(writer);
        }

        write_end_tag(writer, "pageFields");
    }
}
