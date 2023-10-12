

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::ToTokens;

#[proc_macro_derive(HelloWorld, attributes(name))]
pub fn hello_worls(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_hello_world(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_hello_world(ast: &syn::MacroInput) -> quote::Tokens {
    // let attrs = &ast.attrs;
    let name = &ast.ident;
    quote! {
        impl HelloWorld for #name {
            fn hello_world() {
                println!("Hello, World! My name is {}", stringify!(#name));
            }
        }
    }

    // quote! {
    //     impl HelloWorld for #attrs {
    //         fn hello_world() {
    //
    //             for attr in &self.attrs {
    //                 println!("{:?}", attr);
    //             }
    //             // println!("attrs is  {}", stringify!(#attrs));
    //         }
    //     }
    // }
}
