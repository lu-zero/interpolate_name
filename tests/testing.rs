#![feature(proc_macro)]

extern crate interpolate_name;

use interpolate_name::interpolate_test;

#[interpolate_test(foo, "foo")]
#[interpolate_test(bar, "bar")]
#[interpolate_test(baz, "baz")]
fn testme(f: &str) {
    println!("testing {}", f);
}
