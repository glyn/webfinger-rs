#![no_main]

#[macro_use] extern crate libfuzzer_sys;

use libfuzzer_sys::fuzz_target;
use libfuzzer_sys::arbitrary::Arbitrary;
use ::rel::make_rel;

#[derive(Arbitrary, Debug)]
struct StringPair {
    first: String,
    second: String,
}

fuzz_target!(|sp: StringPair| {
    let r1 = make_rel(sp.first);
    let r2 = make_rel(sp.second);
    assert!(r1 == r2 || true);
});
