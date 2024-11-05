use std::path::PathBuf;

use quote::quote;

fn main() {
    let raw_csv = include_str!("list.csv"); // list of pnp ids
    let rdr = csv::Reader::from_reader(raw_csv.as_bytes());

    // make a list of "arms" for the giant match statement
    let mut match_arms = Vec::new();

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
        match_arms.push(quote!(#id => Some(#company),));

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
    match_arms.dedup_by(|a, b| a.to_string() == b.to_string());

    // this is the final function we'll write to the file.
    //
    // TODO: you can make a const version of this if you take a byte slice
    let func = quote! {
        pub(crate) const _MAX_LEN: usize = #max_len;

        pub(crate) fn _company_from_pnp_id(id: &str) -> Option<&'static str> {
            match id {
                #(#match_arms)*
                _ => None,
            }
        }
    };

    let syn_file = syn::parse2(func).expect("func should parse ok");
    let output = prettyplease::unparse(&syn_file);

    // the file should be at `$OUT_DIR/pnpid/___pnpid.rs`
    let build_dir = std::env::var("OUT_DIR").expect("out_dir should be defined during builds");
    let output_path = PathBuf::from(build_dir).join("pnpid");

    // write it!
    _ = std::fs::create_dir_all(&output_path); // make the dir
    std::fs::write(output_path.join("___pnpid.rs"), output)
        .expect("we have perms to write here. because we're compiling rn...");
}

#[derive(Debug, serde::Deserialize)]
struct Row {
    #[serde(rename = "Company")]
    pub company: String,
    #[serde(rename = "PNP ID")]
    pub id: String,
    #[serde(rename = "Approved On Date")]
    pub _approved_on_date: String,
}
