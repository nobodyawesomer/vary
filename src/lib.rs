//! There are three patterns that vary recognizes.
//! ```rust
//! // 1. Variadic Collection.
//! #[varargs]
//! fn expand<T>(...b: Vec<T>) {
//!     println!("{b}");
//! }
//! expand(1, 2, 3);
//! // ^ This might require some magic though. May add last.
//!
//! // 2. Tuple expansion
//! let tuple = ("{}, {}", 2, "hello");
//! println!(tuple...);
//! // Expands to:
//! println!(tuple.0, tuple.1, tuple.2);
//!
//! println!(("Anonymous tuple: {}", 2)...);
//! // Expands to:
//! println!("Anonymous tuple: {}", 2);
//! ```
#![allow(unused)] // just for now...

mod varargs;

use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::fold::Fold;
use syn::parse::Parse;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::Expr;
use syn::ExprTuple;
use syn::Ident;
use syn::ItemFn;
use syn::PatType;
use syn::Token;

/// Walks AST and replaces syntax
struct Variadic;

impl Fold for Variadic {
	fn fold_expr(&mut self, node: Expr) -> Expr {
		match node {
			Expr::Tuple(expr) => Expr::Tuple(expr),
			Expr::Verbatim(expr) => Expr::Verbatim(dbg!(expr)),
			_ => {
				println!("NODE: {}", node.to_token_stream());
				node
			}
		}
	}
}

// struct VariadicExpr;
// struct VariadicIdent;

struct VariadicTuple {
	tuple: ExprTuple,
	variad: Token![...],
}

impl Parse for VariadicTuple {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		todo!()
	}
}

#[proc_macro_attribute]
pub fn vary(args: TokenStream, item: TokenStream) -> TokenStream {
	println!("\nINPUT:\t{:?}", item.to_string());
	let mut varparse = Variadic {}; //parse_macro_input!(args as Variadic);
	let item = parse_macro_input!(item as syn::Item);

	let output = varparse.fold_item(item);
	let output = quote!(#output);
	println!("OUTPUT:\t{:?}\n", output.to_string());
	TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn varargs(_input: TokenStream, annotated_item: TokenStream) -> TokenStream {
	// let input = parse_macro_input!(input as Args); // <- need to custom implement Args.... not yet
	println!("\nINPUT:\t{:?}", annotated_item.to_string());

	let mut item = parse_macro_input!(annotated_item as ItemFn);
	let mut sig = item.sig;
	println!("SIG: [{}]", sig.to_token_stream());
	for arg in &sig.inputs {
		if let syn::FnArg::Typed(pt) = arg {
			println!("ARG: {}", pt.to_token_stream());
		}
	}

	// Construct Output
	let vis = item.vis;
	let body = item.block;
	let output: TokenStream = quote! {
		#vis #sig {
			let testing: ();
			#body
		}
	} // TODO: reconfigure to real function
	.into();

	println!("OUTPUT:\t{:?}\n", output.to_string());
	output
}
