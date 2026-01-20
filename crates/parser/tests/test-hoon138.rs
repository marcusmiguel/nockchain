use std::sync::Arc;
use std::path::PathBuf;
use std::fs;
use chumsky::Parser;
use parser::native_hoon_parser;
use parser::utils::{LineMap, diff_noun};
use parser::noun::{hoon_to_noun};
use nockvm::noun::{D, T, Noun};
use nockapp::noun::slab::{slab_mug, slab_noun_equality, NockJammer, NounSlab};
use nockvm_macros::tas;
use bytes::Bytes;

pub static HOON138JAM: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/parsed-hoon138.jam"
));

#[test]
fn test_hoon_138() {
    let source_path = PathBuf::from("../hoonc/hoon/hoon-138.hoon");
    let source = fs::read_to_string(&source_path).unwrap();

    let hoon = native_hoon_parser(vec![], false, Arc::new(LineMap::new(&"")))
        .parse(source.as_str())
        .into_result()
        .unwrap();

    let mut slab = NounSlab::new();

    let jammed = Bytes::from(HOON138JAM);
    let mut expected_hoon = slab.cue_into(jammed).unwrap();

    let mut actual_hoon = hoon_to_noun(&mut slab, &hoon);

    unsafe {
        assert!(diff_noun(&mut expected_hoon, &mut actual_hoon, &mut false).is_ok())
    }
}
