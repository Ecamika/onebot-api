use proc_macro::TokenStream;
use quote::quote;
use syn::{
	Attribute, Expr, FnArg, ImplItem, ImplItemFn, ItemImpl, Meta, Pat, Type, parse_macro_input,
};

#[derive(Default)]
struct ApiAction {
	extract: Option<String>,
	response_type: Option<Type>,
	renames: Vec<(String, String)>,
}

fn take_api_attr(attrs: &mut Vec<Attribute>) -> Option<Attribute> {
	let idx = attrs.iter().position(|a| a.path().is_ident("api"))?;
	Some(attrs.remove(idx))
}

fn parse_string_meta(nv: &syn::MetaNameValue) -> Option<String> {
	let Expr::Lit(lit) = &nv.value else {
		return None;
	};
	let syn::Lit::Str(s) = &lit.lit else {
		return None;
	};
	Some(s.value())
}

fn parse_type_meta(nv: &syn::MetaNameValue) -> Option<Type> {
	let Expr::Path(expr_path) = &nv.value else {
		return None;
	};
	Some(Type::Path(syn::TypePath {
		qself: None,
		path: expr_path.path.clone(),
	}))
}

fn parse_map_list(list: &syn::MetaList) -> Vec<(String, String)> {
	let Ok(inner) = list
		.parse_args_with(syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated)
	else {
		return Vec::new();
	};

	let mut renames = Vec::new();
	for m in inner {
		let Meta::NameValue(nv) = m else {
			continue;
		};
		let Some(param_name) = nv.path.get_ident().map(|i| i.to_string()) else {
			continue;
		};
		if let Some(value) = parse_string_meta(&nv) {
			renames.push((param_name, value));
		}
	}
	renames
}

fn parse_api_meta(meta: Meta, action: &mut ApiAction) {
	match meta {
		Meta::NameValue(nv) if nv.path.is_ident("extract") => {
			action.extract = parse_string_meta(&nv);
		}
		Meta::NameValue(nv) if nv.path.is_ident("response") => {
			action.response_type = parse_type_meta(&nv);
		}
		Meta::List(list) if list.path.is_ident("map") => {
			action.renames = parse_map_list(&list);
		}
		_ => {}
	}
}

fn take_api_action(attrs: &mut Vec<Attribute>) -> ApiAction {
	let attr = match take_api_attr(attrs) {
		Some(a) => a,
		None => return ApiAction::default(),
	};

	let meta_list = match attr.meta.require_list() {
		Ok(list) => list,
		Err(_) => return ApiAction::default(),
	};

	let nested = match meta_list
		.parse_args_with(syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated)
	{
		Ok(n) => n,
		Err(_) => return ApiAction::default(),
	};

	let mut action = ApiAction::default();
	for meta in nested {
		parse_api_meta(meta, &mut action);
	}

	action
}

fn generate_method_body(method: &ImplItemFn, action: &ApiAction) -> proc_macro2::TokenStream {
	let action_name = method.sig.ident.to_string();
	let action_lit = syn::LitStr::new(&action_name, proc_macro2::Span::call_site());

	let json_entries: Vec<proc_macro2::TokenStream> = method
		.sig
		.inputs
		.iter()
		.filter_map(|arg| {
			if let FnArg::Typed(pat_type) = arg
				&& let Pat::Ident(pat_ident) = &*pat_type.pat
			{
				let param_name = pat_ident.ident.to_string();
				let json_key = action
					.renames
					.iter()
					.find(|(p, _)| p == &param_name)
					.map(|(_, k)| k.clone())
					.unwrap_or_else(|| param_name.clone());

				let param_ident = &pat_ident.ident;
				let json_key_lit = syn::LitStr::new(&json_key, proc_macro2::Span::call_site());

				return Some(quote! {
					#json_key_lit: #param_ident,
				});
			}
			None
		})
		.collect();

	if let Some(ref extract_field) = action.extract {
		let response_type = action
			.response_type
			.as_ref()
			.expect("#[api(extract = \"...\")] 需要同时指定 #[api(response = Type)] 以确定中间响应类型");
		let extract_ident = syn::Ident::new(extract_field, proc_macro2::Span::call_site());

		quote!({
			let params = ::serde_json::json!({
				#(#json_entries)*
			});
			let response: #response_type = self.send_and_parse(#action_lit, params).await?;
			Ok(response.#extract_ident)
		})
	} else {
		quote!({
			let params = ::serde_json::json!({
				#(#json_entries)*
			});
			self.send_and_parse(#action_lit, params).await
		})
	}
}

#[proc_macro_attribute]
pub fn api_sender(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let mut impl_block = parse_macro_input!(item as ItemImpl);

	let mut new_items: Vec<ImplItem> = Vec::with_capacity(impl_block.items.len());

	for item in std::mem::take(&mut impl_block.items) {
		if let ImplItem::Fn(mut method) = item {
			let api_action = take_api_action(&mut method.attrs);
			let body = generate_method_body(&method, &api_action);
			method.block = syn::parse2(body).expect("生成方法体失败");
			new_items.push(ImplItem::Fn(method));
		} else {
			new_items.push(item);
		}
	}

	impl_block.items = new_items;

	quote!(#impl_block).into()
}
