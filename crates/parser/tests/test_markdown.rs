use std::sync::Arc;
use std::path::PathBuf;
use std::fs;
use chumsky::Parser;
use parser::{native_pile_parser, native_hoon_parser};
use parser::utils::{LineMap, diff_noun, print_noun};
use parser::noun::{pile_to_noun, hoon_to_noun};
use nockvm::noun::{D, T, Noun};
use nockapp::noun::slab::{slab_mug, slab_noun_equality, NockJammer, NounSlab};
use nockvm_macros::tas;
use bytes::Bytes;

pub static MARKDOWNJAM: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/markdown.jam"
));

#[test]
fn test_markdown() {
    let source_path = PathBuf::from("../../hoon/common/markdown/markdown.hoon");
    let source = fs::read_to_string(&source_path).unwrap();

    let hoon = match native_hoon_parser(vec![], false, Arc::new(LineMap::new(&source)))
        .parse(source.as_str())
        .into_result()
        Ok(h) => h,
        Err(err) => {
            eprintln!("parse_block error: {err:?}");
            panic!()
        }};

    let mut slab = NounSlab::new();

    let jammed = Bytes::from(MARKDOWNJAM);

    let mut expected_hoon = slab.cue_into(jammed).unwrap();
    let output = print_noun(&expected_hoon, 4000, 0);
    fs::write("expected", output).unwrap_or_else(|e| {
        eprintln!("Failed to write '{}': {}", "expected", e);
        std::process::exit(1);
    });
    let mut actual_hoon = hoon_to_noun(&mut slab, &hoon);

    unsafe {
        assert!(diff_noun(&mut expected_hoon, &mut actual_hoon, &mut false).is_ok())
    }
}
