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

use proc_macro2::{Span, TokenStream};
use syn::{
	Data, Field, Fields, Ident, Type,
	punctuated::Punctuated,
	spanned::Spanned,
	token::Comma,
};

type FieldsList = Punctuated<Field, Comma>;

fn encode_fields(
	fields: &FieldsList,
) -> TokenStream
{
	let recurse = fields.iter().enumerate().map(|(i, f)| {
		let name = f.ident.as_ref().map(|iden| quote! {
			_parity_codec::FieldName::Named(stringify!(#iden))
		})
		.unwrap_or(quote! {
			_parity_codec::FieldName::Unnamed(#i as u32)
		});
		let ty = &f.ty;
		quote_spanned! { f.span() =>
			_parity_codec::FieldMetadata {
				name: #name,
				ty: #ty::metadata()
			}
		}
	});

	quote! {
		_parity_codec::TypeMetadata::Struct(vec![#( #recurse, )*])
	}
}

pub fn quote(data: &Data, type_name: &Ident) -> TokenStream {
	let call_site = Span::call_site();
	let res = match *data {
		Data::Struct(ref data) => {
			match data.fields {
				Fields::Named(ref fields) => encode_fields(
					&fields.named,
				),
				Fields::Unnamed(ref fields) => encode_fields(
					&fields.unnamed,
				),
				Fields::Unit => quote_spanned! { call_site =>
					_parity_codec::TypeMetadata::Struct(vec![])
				},
			}
		},
		Data::Enum(ref data) => {
			let recurse = data.variants.iter().enumerate().map(|(i, f)| {
				let name = &f.ident;
				match f.fields {
					Fields::Named(ref fields) => {
						let field_name = |ty: &Type| {
							quote_spanned!(call_site => #ty)
						};
						let fields = fields.named
							.iter()
							.map(|f| {
								let ty = field_name(&f.ty);
								let name = &f.ident;
								quote_spanned! { f.span() =>
									_parity_codec::FieldMetadata {
										name: _parity_codec::FieldName::Named(stringify!(#name)),
										ty: #ty::metadata()
									}
								}
							});

						quote_spanned! { f.span() =>
							_parity_codec::EnumVariantMetadata {
								name: stringify!(#name),
								variants: vec![#( #fields, )*]
							}
						}
					},
					Fields::Unnamed(ref fields) => {
						let field_name = |ty: &Type| {
							quote_spanned!(call_site => #ty)
						};
						let fields = fields.unnamed
							.iter()
							.map(|f| field_name(&f.ty))
							.enumerate()
							.map(|(i, ty)| quote! {
								_parity_codec::FieldMetadata {
									name: _parity_codec::FieldName::Unnamed(#i as u32),
									ty: #ty::metadata()
								}
							});

						quote_spanned! { f.span() =>
							_parity_codec::EnumVariantMetadata {
								name: stringify!(#name),
								variants: vec![#( #fields, )*]
							}
						}
					},
					Fields::Unit => {
						quote_spanned! { f.span() =>
							_parity_codec::EnumVariantMetadata {
								name: stringify!(#name),
								variants: Vec::new()
							}
						}
					},
				}
			});

			quote! {
				_parity_codec::TypeMetadata::Enum(vec![#( #recurse, )*])
			}
		},
		Data::Union(_) => panic!("Union types are not supported."),
	};
	quote! {
		_parity_codec::Metadata {
			name: stringify!(#type_name),
			kind: #res
		}
	}
}
