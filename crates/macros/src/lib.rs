use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Span;
use quote::quote;
use serde_json::{Value, json};
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{FnArg, Ident, ItemFn, LitStr, Pat, Token, parse_macro_input};

struct IdentList {
	idents: Vec<Ident>,
}

#[derive(Clone)]
struct IdentName {
	pub name: LitStr,
	pub ident: Ident,
}

impl Parse for IdentList {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let mut idents = Vec::new();
		while !input.is_empty() {
			let ident = input.parse::<Ident>()?;
			idents.push(ident);
		}
		Ok(IdentList { idents })
	}
}

#[proc_macro]
pub fn auto_send_ws_msg(input: TokenStream) -> TokenStream {
	let IdentList { idents } = parse_macro_input!(input as IdentList);
	let ident_name_list: Vec<IdentName> = idents
		.iter()
		.map(|ident| IdentName {
			name: LitStr::new(&ident.to_string(), Span::call_site()),
			ident: ident.clone(),
		})
		.collect();
	let (action, params) = ident_name_list.split_at(1);
	let action = &action[0];
	let action_name = action.name.clone();
	let insert_stmts = params
		.iter()
		.map(|ident| {
			let k = ident.name.clone();
			let v = ident.ident.clone();
			quote! {
				map.insert(#k.to_string(), ::serde_json::to_value(#v)?);
			}
		})
		.collect::<Vec<_>>();
	let expended = quote! {
		{
			let mut map: ::std::collections::HashMap<String, ::serde_json::Value> = ::std::collections::HashMap::new();
			#(#insert_stmts)*

			let uuid = ::uuid::Uuid::new_v4().to_string();
			let call_json = ::serde_json::to_string(&Self::generate_api_call_json(#action_name.to_string(), map, uuid.clone()))?;

			let mut receiver = self.get_receiver();
			let max_waiting_times = self.max_waiting_times.clone();
			let task = ::tokio::spawn(async move {
				Self::wait_for_echo(&mut receiver, uuid, Some(max_waiting_times)).await
			});

			self.api_sender.send_async(call_json).await?;

			task.await?
		}
	};
	expended.into()
}

// #[proc_macro_attribute]
// pub fn auto_send(args: TokenStream, input: TokenStream) -> TokenStream {
// 	let input_fn = parse_macro_input!(input as ItemFn);
// 	let sig = &input_fn.sig;
// 	let body = &input_fn.block;
// 	let action_name = &sig.ident;
// 	let inputs = &sig.inputs.iter().filter(|input| {
// 		if let FnArg::Typed(input) = input && let Pat::Ident(ident) = &*input.pat {
// 			return true
// 		}
// 		false
// 	}).collect::<Punctuated<_, Token![,]>>();
// 	let args = sig.inputs.iter().filter(|arg| {
// 		if let FnArg::Typed(arg) = arg && let Pat::Ident(ident) = &*arg.pat {
// 			return true
// 		}
// 		false
// 	}).map(|arg| {
// 		if let FnArg::Typed(arg) = arg && let Pat::Ident(ident) = &*arg.pat {
// 			return Some(ident.ident.clone())
// 		}
// 		None
// 	}).map(|arg| arg.unwrap()).collect::<Vec<_>>();
//
// 	let ident_stmts = args.iter().map(|ident| quote! {
// 		#ident
// 	});
//
// 	let send_msg_stmt = quote! {
// 		::macros::auto_send_ws_msg!(
// 			#action_name
// 			#(#ident_stmts)*
// 		)
// 	};
//
// 	let expended = quote! {
// 		#sig {
// 			async fn __inner<T>(#inputs) -> ::anyhow::Result<T> {
// 				#send_msg_stmt
// 			}
// 			#body
// 		}
// 	};
// 	expended.into()
// }

#[proc_macro_attribute]
pub fn generate_json(args: TokenStream, input: TokenStream) -> TokenStream {
	let input_fn = parse_macro_input!(input as ItemFn);
	let sig = &input_fn.sig;
	let body = &input_fn.block;
	let action_name = &sig.ident;
	let action_name_str = LitStr::new(action_name.to_string().as_str(), Span::call_site());
	let args = sig
		.inputs
		.iter()
		.filter(|arg| {
			if let FnArg::Typed(arg) = arg
				&& let Pat::Ident(ident) = &*arg.pat
			{
				return true;
			}
			false
		})
		.map(|arg| {
			if let FnArg::Typed(arg) = arg
				&& let Pat::Ident(ident) = &*arg.pat
			{
				return Some(ident.ident.clone());
			}
			None
		})
		.map(|arg| arg.unwrap())
		.collect::<Vec<_>>();

	let params = args
		.iter()
		.map(|arg| IdentName {
			name: LitStr::new(arg.to_string().as_str(), Span::call_site()),
			ident: arg.clone(),
		})
		.collect::<Vec<_>>();

	let insert_stmts = params
		.iter()
		.map(|ident| {
			let k = ident.name.clone();
			let v = ident.ident.clone();
			quote! {
				map.insert(#k.to_string(), ::serde_json::to_value(&#v).unwrap());
			}
		})
		.collect::<Vec<_>>();

	let expended = quote! {
		#sig {
			let __echo = ::uuid::Uuid::new_v4().to_string();
			let __json = Self::generate_api_call_json(#action_name_str.to_string(), {
				let mut map: ::std::collections::HashMap<String, ::serde_json::Value> = ::std::collections::HashMap::new();
				#(#insert_stmts)*
				map
			}, __echo.clone());
			#body
		}
	};
	expended.into()
}
