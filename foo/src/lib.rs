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

use parity_codec::{Encode, Decode, HasCompact};

#[derive(Encode)]
enum EnumType {
	A,
	B(u32, u64),
	C {
		c: u32,
		d: u64,
	},
}


#[derive(Encode)]
struct StructType {
	a: u32,
	b: EnumType
}

#[test]
fn test() {
	println!("{:#?}", EnumType::metadata());
	println!("{:#?}", StructType::metadata());
}
