#![no_main]

use libfuzzer_sys::fuzz_target;
use aho_corasick::AhoCorasick;

fuzz_target!(|data: &[u8]| {
    if data.len() < 2 {
        return;
    }
    let split = data[0] as usize % data.len().max(1);
    let (patterns_bytes, haystack) = data.split_at(split);
    let patterns: Vec<&[u8]> = patterns_bytes.split(|&b| b == 0).filter(|p| !p.is_empty()).collect();
    if patterns.is_empty() {
        return;
    }
    if let Ok(ac) = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .build(patterns)
    {
        let _ = ac.find_iter(haystack).count();
    }
});
