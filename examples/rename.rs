#![feature(proc_macro)]

extern crate interpolate_name;

use interpolate_name::interpolate_name;

fn callme(f: &str) {
    println!("called {}", f);
}

#[interpolate_name(one)]
fn try() {

}

macro_rules! rep {
    ($($sample:ident),+) => {
        $(
            #[interpolate_name($sample)]
            fn repetitive_call() {
                callme(stringify!($sample));
            }
        )+
    }
}

rep! {foo1, bar1, baz1}

fn main() {
    try_one();

    repetitive_call_foo1();
    repetitive_call_bar1();
    repetitive_call_baz1();
}
