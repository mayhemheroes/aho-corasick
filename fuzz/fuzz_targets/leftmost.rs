#![no_main]
use libfuzzer_sys::fuzz_target;

use aho_corasick::{AhoCorasickBuilder, MatchKind};

fuzz_target!(|data: &[u8]| {
    let patterns = &["Samwise", "Sam"];

    let ac = AhoCorasickBuilder::new()
        .match_kind(MatchKind::LeftmostFirst)
        .build(patterns);
    let mat = ac.find(data);
});
