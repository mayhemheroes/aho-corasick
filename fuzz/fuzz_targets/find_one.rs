#![no_main]
use libfuzzer_sys::fuzz_target;

use aho_corasick::AhoCorasick;

fuzz_target!(|data: &[u8]| {
    let patterns = &["Samwise", "Sam"];

    let ac = AhoCorasick::new(patterns);
    let mat = ac.find(data);
});
