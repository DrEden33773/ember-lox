use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ExprArray};

#[proc_macro]
pub fn gen_reserved_tok_methods(input: TokenStream) -> TokenStream {
  let array = parse_macro_input!(input as ExprArray);

  let mut methods = Vec::new();

  for expr in array.elems.iter() {
    if let Expr::Lit(expr_lit) = expr {
      if let syn::Lit::Str(lit_str) = &expr_lit.lit {
        let word = lit_str.value();
        let method_name = syn::Ident::new(&format!("{}_tok", word), lit_str.span());
        let len = word.len();

        // declare `<xxx>_tok` method
        methods.push(quote! {
          pub fn #method_name() -> Self {
            Token {
              tag: TagToken {
                kind: Identifier,
                len: #len,
                line: 0,
              },
              val: #word,
            }
          }
        });
      }
    }
  }

  let expanded = quote! {
    // impl generated `<xxx>_tok` methods for Token
    impl<'src> Token<'src> {
      #(#methods)*
    }
  };

  TokenStream::from(expanded)
}
