use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Meta, MetaNameValue, Type, TypePath};

fn to_snake_case(s: &str) -> String {
	let mut result = String::new();
	for (i, c) in s.chars().enumerate() {
		if c.is_uppercase() {
			if i > 0 {
				result.push('_');
			}
			result.push(c.to_lowercase().next().unwrap());
		} else {
			result.push(c);
		}
	}
	result
}

fn is_primitive_type(ty: &Type) -> bool {
	if let Type::Path(TypePath { qself: None, path }) = ty {
		if let Some(ident) = path.get_ident() {
			let name = ident.to_string();
			return matches!(
				name.as_str(),
				"i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64"
					| "u128" | "f32" | "f64" | "bool" | "char"
			);
		}
	}
	false
}

fn is_string_type(ty: &Type) -> bool {
	if let Type::Path(TypePath { qself: None, path }) = ty {
		if let Some(ident) = path.get_ident() {
			return ident == "String";
		}
	}
	false
}

fn extract_box_inner(ty: &Type) -> Option<&Type> {
	if let Type::Path(TypePath { qself: None, path }) = ty {
		if let Some(segment) = path.segments.first() {
			if segment.ident == "Box" {
				if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
					if args.args.len() == 1 {
						if let syn::GenericArgument::Type(inner) = &args.args[0] {
							return Some(inner);
						}
					}
				}
			}
		}
	}
	None
}

fn get_filter_param_type(ty: &Type) -> TokenStream2 {
	if let Some(inner) = extract_box_inner(ty) {
		return quote!(&#inner);
	}
	if is_string_type(ty) {
		quote!(&str)
	} else if is_primitive_type(ty) {
		quote!(#ty)
	} else {
		quote!(&#ty)
	}
}

fn get_field_access(ty: &Type, field_name: &Ident) -> TokenStream2 {
	if extract_box_inner(ty).is_some() {
		quote!(&*data.#field_name)
	} else if is_string_type(ty) {
		quote!(&data.#field_name)
	} else if is_primitive_type(ty) {
		quote!(data.#field_name)
	} else {
		quote!(&data.#field_name)
	}
}

fn parse_variants(attrs: &[syn::Attribute]) -> Option<Vec<Ident>> {
	for attr in attrs {
		if attr.path().is_ident("selector") {
			let nested = match attr
				.parse_args_with(syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated)
			{
				Ok(n) => n,
				Err(_) => continue,
			};
			for meta in nested {
				if let Meta::List(list) = &meta {
					if list.path.is_ident("variants") {
						let idents = match list.parse_args_with(
							syn::punctuated::Punctuated::<Ident, syn::Token![,]>::parse_terminated,
						) {
							Ok(i) => i.into_iter().collect(),
							Err(_) => continue,
						};
						return Some(idents);
					}
				}
			}
		}
	}
	None
}

fn parse_through(attrs: &[syn::Attribute]) -> Option<Ident> {
	for attr in attrs {
		if attr.path().is_ident("selector") {
			let nested = match attr
				.parse_args_with(syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated)
			{
				Ok(n) => n,
				Err(_) => continue,
			};
			for meta in nested {
				if let Meta::NameValue(MetaNameValue { path, value, .. }) = &meta {
					if path.is_ident("through") {
						if let syn::Expr::Lit(syn::ExprLit {
							lit: syn::Lit::Str(lit_str),
							..
						}) = value
						{
							let method_name = Ident::new(&lit_str.value(), lit_str.span());
							return Some(method_name);
						}
					}
				}
			}
		}
	}
	None
}

fn get_through_return_type(field_ty: &Type) -> Type {
	if let Some(inner) = extract_box_inner(field_ty) {
		inner.clone()
	} else {
		field_ty.clone()
	}
}

fn generate_struct_selector(
	name: &Ident,
	generics: &syn::Generics,
	data: &syn::DataStruct,
) -> TokenStream {
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let selector_impl = quote! {
		#[cfg(feature = "selector")]
		impl #impl_generics #name #ty_generics #where_clause {
			pub const fn selector(&'_ self) -> crate::selector::Selector<'_, Self> {
				crate::selector::Selector { data: Some(self) }
			}
		}
	};

	let mut filter_methods = Vec::new();

	for field in &data.fields {
		let field_name = field.ident.as_ref().unwrap();
		let field_ty = &field.ty;
		let param_ty = get_filter_param_type(field_ty);
		let field_access = get_field_access(field_ty, field_name);

		let filter_name = format_ident!("filter_{}", field_name);
		let and_filter_name = format_ident!("and_filter_{}", field_name);
		let filter_async_name = format_ident!("filter_{}_async", field_name);
		let and_filter_async_name = format_ident!("and_filter_{}_async", field_name);

		filter_methods.push(quote! {
			pub fn #filter_name(&mut self, f: impl FnOnce(#param_ty) -> bool) {
				if let Some(data) = self.data
					&& !f(#field_access)
				{
					self.data = None
				}
			}

			pub fn #and_filter_name(mut self, f: impl FnOnce(#param_ty) -> bool) -> Self {
				self.#filter_name(f);
				self
			}

			pub async fn #filter_async_name(&mut self, f: impl AsyncFnOnce(#param_ty) -> bool) {
				if let Some(data) = self.data
					&& !f(#field_access).await
				{
					self.data = None
				}
			}

			pub async fn #and_filter_async_name(mut self, f: impl AsyncFnOnce(#param_ty) -> bool) -> Self {
				self.#filter_async_name(f).await;
				self
			}
		});

		if let Some(variants) = parse_variants(&field.attrs) {
			for variant in variants {
				let is_method = Ident::new(
					&format!("is_{}", variant),
					variant.span(),
				);
				let and_variant = format_ident!("and_{}", variant);
				let not_variant = format_ident!("not_{}", variant);
				let and_not_variant = format_ident!("and_not_{}", variant);

				filter_methods.push(quote! {
					pub const fn #variant(&mut self) {
						if let Some(data) = self.data
							&& !data.#field_name.#is_method()
						{
							self.data = None
						}
					}

					pub const fn #and_variant(mut self) -> Self {
						self.#variant();
						self
					}

					pub const fn #not_variant(&mut self) {
						if let Some(data) = self.data
							&& data.#field_name.#is_method()
						{
							self.data = None
						}
					}

					pub const fn #and_not_variant(mut self) -> Self {
						self.#not_variant();
						self
					}
				});
			}
		}

		if let Some(method_name) = parse_through(&field.attrs) {
			let return_ty = get_through_return_type(field_ty);
			let access = if extract_box_inner(field_ty).is_some() {
				quote!(self.data.map(|d| &*d.#field_name))
			} else {
				quote!(self.data.map(|d| &d.#field_name))
			};

			filter_methods.push(quote! {
				pub fn #method_name(&self) -> crate::selector::Selector<'a, #return_ty> {
					crate::selector::Selector {
						data: #access,
					}
				}
			});
		}
	}

	let selector_methods = quote! {
		#[cfg(feature = "selector")]
		impl<'a> crate::selector::Selector<'a, #name> {
			#(#filter_methods)*
		}
	};

	let expanded = quote! {
		#selector_impl
		#selector_methods
	};

	expanded.into()
}

fn generate_enum_selector(
	name: &Ident,
	generics: &syn::Generics,
	data: &syn::DataEnum,
) -> TokenStream {
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	let mut match_methods = Vec::new();
	let mut selector_variant_methods = Vec::new();

	for variant in &data.variants {
		let variant_name = &variant.ident;
		let snake_name_str = to_snake_case(&variant_name.to_string());
		let snake_name = Ident::new(&snake_name_str, variant_name.span());

		if let Fields::Unnamed(fields) = &variant.fields {
			if fields.unnamed.len() == 1 {
				let field_ty = &fields.unnamed[0].ty;

				let match_name = format_ident!("match_{}", snake_name);
				let on_name = format_ident!("on_{}", snake_name);
				let on_async_name = format_ident!("on_{}_async", snake_name);

				match_methods.push(quote! {
					pub const fn #match_name(&self) -> Option<&#field_ty> {
						if let Self::#variant_name(data) = self {
							Some(data)
						} else {
							None
						}
					}

					pub fn #on_name<T>(&self, handler: impl FnOnce(&#field_ty) -> T) -> Option<T> {
						if let Self::#variant_name(data) = self {
							Some(handler(data))
						} else {
							None
						}
					}

					pub async fn #on_async_name<T>(&self, handler: impl AsyncFnOnce(&#field_ty) -> T) -> Option<T> {
						if let Self::#variant_name(data) = self {
							Some(handler(data).await)
						} else {
							None
						}
					}
				});

				selector_variant_methods.push(quote! {
					pub fn #snake_name(&self) -> crate::selector::Selector<'a, #field_ty> {
						crate::selector::Selector {
							data: self.data.and_then(|d| d.#match_name()),
						}
					}
				});
			}
		}
	}

	let expanded = quote! {
		#[cfg(feature = "selector")]
		impl #impl_generics #name #ty_generics #where_clause {
			pub const fn selector(&'_ self) -> crate::selector::Selector<'_, Self> {
				crate::selector::Selector { data: Some(self) }
			}

			#(#match_methods)*
		}

		#[cfg(feature = "selector")]
		impl<'a> crate::selector::Selector<'a, #name> {
			#(#selector_variant_methods)*
		}
	};

	expanded.into()
}

pub fn derive_selector(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = &input.ident;
	let generics = &input.generics;

	match &input.data {
		Data::Struct(data) => generate_struct_selector(name, generics, data),
		Data::Enum(data) => generate_enum_selector(name, generics, data),
		Data::Union(_) => panic!("Selector cannot be derived for unions"),
	}
}
