extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprArray, parse_macro_input};

#[proc_macro]
pub fn gen_reserved_tok_methods(input: TokenStream) -> TokenStream {
  let array = parse_macro_input!(input as ExprArray);

  let mut methods = Vec::new();
  let mut keyword_literals = Vec::new();

  for expr in array.elems.iter() {
    if let Expr::Lit(expr_lit) = expr {
      if let syn::Lit::Str(lit_str) = &expr_lit.lit {
        // 生成 `<xxx>_tok` 方法
        let word = lit_str.value();
        let method_name = syn::Ident::new(&format!("{}_tok", word), lit_str.span());
        let len = word.len();

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

        // 收集 reserved-words <xxx> 字面量
        keyword_literals.push(quote! { #word });
      }
    }
  }

  let expanded = quote! {
      // 生成的 `<xxx>_tok` 方法, 实现为 Token 的 `静态方法`
      impl<'src> Token<'src> {
          #(#methods)*
      }
  };

  TokenStream::from(expanded)
}
