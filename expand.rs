#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
// Copyright 2017, 2018 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate parity_codec;

#[macro_use]
extern crate parity_codec_derive;

use parity_codec::{Decode, Encode, HasCompact};

enum EnumType {
    A,
    B(u32, u64),
    C { c: u32, d: u64 },
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_EnumType: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate parity_codec as _parity_codec;
    impl _parity_codec::Encode for EnumType {
        fn encode_to<EncOut: _parity_codec::Output>(&self, dest: &mut EncOut) {
            match *self {
                EnumType::A => {
                    dest.push_byte(0usize as u8);
                }
                EnumType::B(ref aa, ref ba) => {
                    dest.push_byte(1usize as u8);
                    dest.push(aa);
                    dest.push(ba);
                }
                EnumType::C { ref c, ref d } => {
                    dest.push_byte(2usize as u8);
                    dest.push(c);
                    dest.push(d);
                }
            }
        }
        fn metadata_to<EncOut: _parity_codec::Output>(dest: &mut EncOut) {
            dest.push("EnumType");
            let test = "EnumType :: A";
            let test = "EnumType :: B";
            let vec = <[_]>::into_vec(box ["0usize u32", "1usize u64"]);
            let test = "EnumType :: C";
            let vec = <[_]>::into_vec(box ["c u32", "d u64"]);
        }
    }
};
