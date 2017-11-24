// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() {
    println!("Test", 123, 456, 789);

    println!("Test2",
        123,
        456,
        789
    );

    println!("Some stuff", UNUSED="args"); //~ ERROR named argument never used

    println!("Some more $STUFF",
        "woo!",
            STUFF=
       "things"
             , UNUSED="args");
}
