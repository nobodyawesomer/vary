use syn::{
	parenthesized,
	parse::{Parse, ParseStream},
	punctuated::Punctuated,
	token, Abi, Attribute, Block, FnArg, Generics, Ident, Pat, PatType, ReturnType, Token, Type,
	Visibility,
};

pub struct VariadicFn {
	pub attrs: Vec<Attribute>,
	pub vis: Visibility,
	pub sig: VariadicSignature,
	pub block: Box<Block>,
}

pub struct VariadicSignature {
	pub constness: Option<Token![const]>,
	pub asyncness: Option<Token![async]>,
	pub unsafety: Option<Token![unsafe]>,
	pub abi: Option<Abi>,
	pub fn_token: Token![fn],
	pub ident: Ident,
	pub generics: Generics,
	pub paren_token: token::Paren,
	pub inputs: Punctuated<FnArg, Token![,]>,
	pub variadic: Option<VariadicPatType>,
	pub output: ReturnType,
}

pub struct VariadicPatType {
	pub attrs: Vec<Attribute>,
	pub dot3_token: Token![...],
	pub pat: Box<Pat>,
	pub colon_token: Token![:],
	pub ty: Box<Type>,
}

impl Parse for VariadicSignature {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let constness: Option<Token![const]> = input.parse()?;
		let asyncness: Option<Token![async]> = input.parse()?;
		let unsafety: Option<Token![unsafe]> = input.parse()?;
		let abi: Option<Abi> = input.parse()?;
		let fn_token: Token![fn] = input.parse()?;
		let ident: Ident = input.parse()?;
		let mut generics: Generics = input.parse()?;

		let content;
		let paren_token = parenthesized!(content in input);
		let mut inputs = parse_fn_args(&content)?;
		let variadic = pop_variadic(&mut inputs);

		let output: ReturnType = input.parse()?;
		generics.where_clause = input.parse()?;

		Ok(VariadicSignature {
			constness,
			asyncness,
			unsafety,
			abi,
			fn_token,
			ident,
			generics,
			paren_token,
			inputs,
			variadic,
			output,
		})
	}
}

fn pop_variadic(inputs: _) -> _ {
	todo!()
}

fn parse_fn_args(input: ParseStream) -> Result<Punctuated<FnArg, Token![,]>> {
	todo!()
}
