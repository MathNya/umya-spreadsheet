macro_rules! data_validation_formula {
    ($struct_name:ident, $tag:expr) => {
        #[derive(Default, Debug, Clone)]
        pub struct $struct_name {
            value: crate::structs::office::excel::Formula,
        }
        impl $struct_name {
            const TAG: &str = $tag;

            #[inline]
            #[must_use]
            pub fn value(&self) -> &crate::structs::office::excel::Formula {
                &self.value
            }

            #[inline]
            #[must_use]
            #[deprecated(since = "3.0.0", note = "Use value()")]
            pub fn get_value(&self) -> &crate::structs::office::excel::Formula {
                self.value()
            }

            #[inline]
            pub fn value_mut(&mut self) -> &mut crate::structs::office::excel::Formula {
                &mut self.value
            }

            #[inline]
            #[deprecated(since = "3.0.0", note = "Use value_mut()")]
            pub fn get_value_mut(&mut self) -> &mut crate::structs::office::excel::Formula {
                self.value_mut()
            }

            #[inline]
            pub fn set_value(
                &mut self,
                value: crate::structs::office::excel::Formula,
            ) -> &mut Self {
                self.value = value;
                self
            }

            pub(crate) fn set_attributes<R: std::io::BufRead>(
                &mut self,
                reader: &mut quick_xml::Reader<R>,
                _e: &quick_xml::events::BytesStart,
            ) {
                let mut buf = Vec::new();
                loop {
                    match reader.read_event_into(&mut buf) {
                        Ok(quick_xml::events::Event::Start(ref e)) => {
                            if e.name().into_inner() == b"xm:f" {
                                let mut obj = crate::structs::office::excel::Formula::default();
                                obj.set_attributes(reader, e);
                                self.value = obj;
                                return;
                            }
                        }
                        Ok(quick_xml::events::Event::End(ref e)) => {
                            if e.name().into_inner() == Self::TAG.as_bytes() {
                                return;
                            }
                        }
                        Ok(quick_xml::events::Event::Eof) => {
                            panic!("Error: Could not find {} end element", Self::TAG)
                        }
                        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                        _ => (),
                    }
                    buf.clear();
                }
            }

            #[inline]
            pub(crate) fn write_to(
                &self,
                writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
            ) {
                crate::writer::driver::write_start_tag(writer, Self::TAG, vec![], false);
                self.value.write_to(writer);
                crate::writer::driver::write_end_tag(writer, Self::TAG);
            }
        }
    };
}

pub(crate) use data_validation_formula;
