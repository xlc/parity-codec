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

use std::str::from_utf8;

use proc_macro2::{Span, TokenStream};
use syn::{
	Data, Field, Fields, Ident, Index, Type,
	punctuated::Punctuated,
	spanned::Spanned,
	token::Comma,
};
use utils;

type FieldsList = Punctuated<Field, Comma>;

fn encode_fields(
	dest: &TokenStream,
	fields: &FieldsList,
) -> TokenStream
{
	let recurse = fields.iter().enumerate().map(|(i, f)| {
		let name = f.ident.as_ref().map(|iden| iden.to_string()).unwrap_or(i.to_string());
		// let ty = format!("{:?}", f.ty);
		quote_spanned! { f.span() =>
			// #dest.push(b"\"");
			#dest.push(#name.as_bytes());
			// #dest.push(b"\" : ");
			// #dest.push(#ty)
			// #dest.push(b"\n");
		}
	});

	quote! {
		#( #recurse )*
	}
	// quote! {
	// 	#dest.push(b"\" : ");
	// }
}

pub fn quote(data: &Data, type_name: &Ident, dest: &TokenStream) -> TokenStream {
	let call_site = Span::call_site();
	let res = match *data {
		Data::Struct(ref data) => {
			match data.fields {
				Fields::Named(ref fields) => encode_fields(
					dest,
					&fields.named,
				),
				Fields::Unnamed(ref fields) => encode_fields(
					dest,
					&fields.unnamed,
				),
				Fields::Unit => quote_spanned! { call_site =>
					drop(#dest);
				},
			}
		},
		Data::Enum(ref data) => {
			let recurse = data.variants.iter().enumerate().map(|(i, f)| {
				let name = &f.ident;
				let index = utils::index(f, i);
				match f.fields {
					Fields::Named(ref fields) => {
						let field_name = |_, ident: &Option<Ident>, ty: &Type| quote_spanned!(call_site => #ident #ty);
						let names = fields.named
							.iter()
							.enumerate()
							.map(|(i, f)| field_name(i, &f.ident, &f.ty));

						quote_spanned! { f.span() =>
							let test = stringify!(#type_name :: #name);
							let vec = vec!(#( stringify!(#names), )*);
						}
					},
					Fields::Unnamed(ref fields) => {
						let field_name = |i, ty: &Type| {
							quote_spanned!(call_site => #i #ty)
						};
						let names = fields.unnamed
							.iter()
							.enumerate()
							.map(|(i, f)| field_name(i, &f.ty));

						quote_spanned! { f.span() =>
							let test = stringify!(#type_name :: #name);
							let vec = vec!(#( stringify!(#names), )*);
						}
					},
					Fields::Unit => {
						quote_spanned! { f.span() =>
							let test = stringify!(#type_name :: #name);
						}
					},
				}
			});

			quote! {
				#( #recurse )*
			}
		},
		Data::Union(_) => panic!("Union types are not supported."),
	};
	quote! {
		#dest.push(stringify!(#type_name));
		#res
	}
}
pub fn stringify(id: u8) -> [u8; 2] {
	const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
	let len = CHARS.len() as u8;
	let symbol = |id: u8| CHARS[(id % len) as usize];
	let a = symbol(id);
	let b = symbol(id / len);

	[a, b]
}
