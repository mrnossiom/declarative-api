mod symbols;

#[proc_macro]
pub fn symbols(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	symbols::symbols(input.into()).into()
}
