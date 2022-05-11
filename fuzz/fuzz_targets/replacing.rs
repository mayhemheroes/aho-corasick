#![no_main]
use libfuzzer_sys::fuzz_target;

use aho_corasick::AhoCorasick;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let patterns = &["fox", "brown", "quick"];
    let replace_with = &["sloth", "grey", "slow"];

    // In a real example, these might be `std::fs::File`s instead. All you need to
    // do is supply a pair of `std::io::Read` and `std::io::Write` implementations.
    let mut wtr = vec![];

    let ac = AhoCorasick::new(patterns);
    match ac.stream_replace_all(data, &mut wtr, replace_with) {
        Ok(_) => {},
        Err(_) => {},
    }
});
