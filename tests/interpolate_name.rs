#![feature(proc_macro)]

extern crate interpolate_name;

use interpolate_name::interpolate_name;

fn callme(f: &str) {
    println!("called {}", f);
}

#[interpolate_name(1)]
#[test]
fn try() {

}

macro_rules! rep {
    ($($sample:ident),+) => {
        $(
            #[test]
            #[interpolate_name($sample)]
            fn repetitive_call() {
                callme(stringify!($sample));
            }
        )+
    }
}

rep! {foo1, bar1, baz1}


