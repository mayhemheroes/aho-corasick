#![no_main]

use libfuzzer_sys::fuzz_target;
use aho_corasick::AhoCorasick;

fuzz_target!(|data: &[u8]| {
    if data.len() < 3 {
        return;
    }
    let split1 = (data[0] as usize % (data.len() - 1)).max(1);
    let split2 = split1 + ((data[1] as usize % (data.len() - split1)).max(1));
    let split2 = split2.min(data.len());
    let patterns_bytes = &data[..split1];
    let replace_bytes = &data[split1..split2];
    let haystack = &data[split2..];
    let patterns: Vec<&[u8]> = patterns_bytes.split(|&b| b == 0).filter(|p| !p.is_empty()).collect();
    let replacements: Vec<&[u8]> = replace_bytes.split(|&b| b == 0).collect();
    if patterns.is_empty() {
        return;
    }
    if let Ok(ac) = AhoCorasick::new(patterns.clone()) {
        if replacements.len() >= patterns.len() {
            let _ = ac.replace_all_bytes(haystack, &replacements[..patterns.len()]);
        }
    }
});
