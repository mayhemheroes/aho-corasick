#![no_main]
use libfuzzer_sys::fuzz_target;

use aho_corasick::{AhoCorasickBuilder};

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let patterns = &["apple", "maple", "Snapple"];
    let haystack = data;

    let ac = AhoCorasickBuilder::new()
        .ascii_case_insensitive(true)
        .build(patterns);
    let mut matches = vec![];
    for mat in ac.find_iter(haystack) {
        matches.push((mat.pattern(), mat.start(), mat.end()));
    }
});
