use row::Row;
use std::path::PathBuf;

fn main() {
    let raw_csv = include_str!("list.csv"); // list of pnp ids
    let rdr = csv::Reader::from_reader(raw_csv.as_bytes());

    // make a list of "arms" for the giant match statement
    let mut match_arms = Vec::new();

    // a list of array entries
    let mut array_entries = Vec::new();

    // we keep track of the longest company name
    let mut max_len = 0_usize;

    // we'll add a new match line for each row of csv
    for row in rdr.into_deserialize::<Row>().flatten() {
        // destructure
        let Row {
            company,
            id,
            _approved_on_date,
        } = row;

        // replace weird "non-break space" characters
        let company = company.replace('\u{a0}', " ");
        let id = id.replace('\u{a0}', ""); // EVEN THE IDS HAVE IT??? see: `AMS`

        // trim the value
        let company = company.trim();

        // add to the list of const match arms
        match_arms.push(format!("\"{id}\" => Some(\"{company}\"),"));

        // and to the list of array entries
        array_entries.push(format!("(\"{id}\", \"{company}\"),"));

        // check if we're the longest company so far
        if company.len() > max_len {
            max_len = company.len();
        }
    }

    // make sure the list isn't empty
    if match_arms.is_empty() {
        panic!("Didn't detect any entries in the given file.");
    }

    // remove consecutive duplicates (i'm looking at you, DemoPad Software Ltd!)
    match_arms.dedup_by(|a, b| *a == *b);
    #[cfg(feature = "array")]
    array_entries.dedup_by(|a, b| *a == *b);

    // grab the length of the list (used to make array constant)
    let num_of_entries = match_arms.len();

    let array = if cfg!(feature = "array") {
        // create array
        let mut s =
            format!("pub(crate) const _ALL_COMPANIES: [(&str, &str); {num_of_entries}] = [");

        // each entry goes here
        for entry in array_entries {
            s.push_str("    ");
            s.push_str(&entry);
            s.push('\n');
        }

        // close array
        s.push_str("];");

        s
    } else {
        String::new()
    };

    // this is the final output we'll write to the file.
    let output = {
        let mut s = String::new();

        // start with non-list stuff
        s.push_str(&format!(
            "pub(crate) const _MAX_LEN: usize = {max_len};

pub(crate) const _NUM_OF_ENTRIES: usize = {num_of_entries};

pub(crate) fn _company_from_pnp_id(id: &str) -> Option<&'static str> {{
    match id {{",
        ));

        // add the list (match)
        for arm in match_arms {
            s.push_str("        ");
            s.push_str(&arm);
            s.push('\n');
        }

        // add none arm
        s.push_str("        _ => None,\n");

        // close func
        s.push_str("    }\n}\n\n");

        // add array
        s.push_str(&array);

        // return total file
        s
    };

    // the file should be at `$OUT_DIR/pnpid/___pnpid.rs`
    let build_dir = std::env::var("OUT_DIR").expect("out_dir should be defined during builds");
    let output_path = PathBuf::from(build_dir).join("pnpid");

    // write it!
    _ = std::fs::create_dir_all(&output_path); // make the dir
    std::fs::write(output_path.join("___pnpid.rs"), output)
        .expect("we have perms to write here. because we're compiling rn...");
}

mod row {
    use core::marker::PhantomData;
    use serde::de::Error as _;

    #[derive(Debug)]
    pub struct Row {
        pub company: String,
        pub id: String,
        pub _approved_on_date: String,
    }

    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        #[automatically_derived]
        impl<'de> serde::Deserialize<'de> for Row {
            fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut core::fmt::Formatter,
                    ) -> core::fmt::Result {
                        core::fmt::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(self, __value: u64) -> Result<Self::Value, __E>
                    where
                        __E: serde::de::Error,
                    {
                        match __value {
                            0u64 => Ok(__Field::__field0),
                            1u64 => Ok(__Field::__field1),
                            2u64 => Ok(__Field::__field2),
                            _ => Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(self, __value: &str) -> Result<Self::Value, __E>
                    where
                        __E: serde::de::Error,
                    {
                        match __value {
                            "Company" => Ok(__Field::__field0),
                            "PNP ID" => Ok(__Field::__field1),
                            "Approved On Date" => Ok(__Field::__field2),
                            _ => Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(self, __value: &[u8]) -> Result<Self::Value, __E>
                    where
                        __E: serde::de::Error,
                    {
                        match __value {
                            b"Company" => Ok(__Field::__field0),
                            b"PNP ID" => Ok(__Field::__field1),
                            b"Approved On Date" => Ok(__Field::__field2),
                            _ => Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
                    where
                        __D: serde::Deserializer<'de>,
                    {
                        serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: PhantomData<Row>,
                    lifetime: PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Row;
                    fn expecting(
                        &self,
                        __formatter: &mut core::fmt::Formatter,
                    ) -> core::fmt::Result {
                        core::fmt::Formatter::write_str(__formatter, "struct Row")
                    }
                    #[inline]
                    fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                    where
                        __A: serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match serde::de::SeqAccess::next_element::<String>(&mut __seq)? {
                                Some(__value) => __value,
                                None => {
                                    return Err(serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Row with 3 elements",
                                    ));
                                }
                            };
                        let __field1 =
                            match serde::de::SeqAccess::next_element::<String>(&mut __seq)? {
                                Some(__value) => __value,
                                None => {
                                    return Err(serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Row with 3 elements",
                                    ));
                                }
                            };
                        let __field2 =
                            match serde::de::SeqAccess::next_element::<String>(&mut __seq)? {
                                Some(__value) => __value,
                                None => {
                                    return Err(serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Row with 3 elements",
                                    ));
                                }
                            };
                        Ok(Row {
                            company: __field0,
                            id: __field1,
                            _approved_on_date: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                    where
                        __A: serde::de::MapAccess<'de>,
                    {
                        let mut __field0: Option<String> = None;
                        let mut __field1: Option<String> = None;
                        let mut __field2: Option<String> = None;
                        while let Some(__key) =
                            serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                        {
                            match __key {
                                __Field::__field0 => {
                                    if Option::is_some(&__field0) {
                                        return Err(
                                            <__A::Error as serde::de::Error>::duplicate_field(
                                                "Company",
                                            ),
                                        );
                                    }
                                    __field0 = Some(serde::de::MapAccess::next_value::<String>(
                                        &mut __map,
                                    )?);
                                }
                                __Field::__field1 => {
                                    if Option::is_some(&__field1) {
                                        return Err(
                                            <__A::Error as serde::de::Error>::duplicate_field(
                                                "PNP ID",
                                            ),
                                        );
                                    }
                                    __field1 = Some(serde::de::MapAccess::next_value::<String>(
                                        &mut __map,
                                    )?);
                                }
                                __Field::__field2 => {
                                    if Option::is_some(&__field2) {
                                        return Err(
                                            <__A::Error as serde::de::Error>::duplicate_field(
                                                "Approved On Date",
                                            ),
                                        );
                                    }
                                    __field2 = Some(serde::de::MapAccess::next_value::<String>(
                                        &mut __map,
                                    )?);
                                }
                                _ => {
                                    let _ = serde::de::MapAccess::next_value::<
                                        serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            Some(__field0) => __field0,
                            None => Err(__A::Error::missing_field("Company"))?,
                        };
                        let __field1 = match __field1 {
                            Some(__field1) => __field1,
                            None => Err(__A::Error::missing_field("PNP ID"))?,
                        };
                        let __field2 = match __field2 {
                            Some(__field2) => __field2,
                            None => Err(__A::Error::missing_field("Approved On Date"))?,
                        };
                        Ok(Row {
                            company: __field0,
                            id: __field1,
                            _approved_on_date: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &[&str] = &["Company", "PNP ID", "Approved On Date"];
                serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Row",
                    FIELDS,
                    __Visitor {
                        marker: PhantomData::<Row>,
                        lifetime: PhantomData,
                    },
                )
            }
        }
    };
}
