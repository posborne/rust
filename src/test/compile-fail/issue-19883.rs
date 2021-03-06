// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait From<Src> {
    type Output;

    fn from(src: Src) -> <Self as From<Src>>::Output;
}

trait To {
    fn to<Dst: From<Self>>(self) ->
        <Dst as From<Self>>::Dst
        //~^ ERROR use of undeclared associated type `From::Dst`
    {
        From::from(self)
    }
}

fn main() {}
