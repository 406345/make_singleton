extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

// #[proc_macro_derive(MakeSingleton)]
// pub fn make_singleton(input: TokenStream) -> TokenStream {
//     // Construct a string representation of the type definition
//     let s = input.to_string();

//     // Parse the string representation
//     let ast = syn::parse_derive_input(&s).unwrap();

//     // Build the impl
//     let gen = impl_make_singelton(&ast);

//     // Return the generated impl
//     gen.parse().unwrap()
// }

// fn impl_make_singelton(ast: &syn::DeriveInput) -> quote::Tokens {
//     let name = &ast.ident;
//     quote! {

//         use std::sync::{Arc, Mutex, Once};
//         use std::{mem};
//         use std::boxed::Box;

//         trait MakeSingletonTemplateTrait {
//             fn instance_ptr() -> *const #name ;
//             fn instance() -> Arc<Box<#name>>;
//         }

//         impl MakeSingletonTemplateTrait for #name {
//             fn instance_ptr() -> *const #name {
//                 unsafe {
//                     static mut INSTANCE: *const #name = 0 as *const #name;
//                     static ONCE: Once = Once::new();

//                     ONCE.call_once(||{
//                         let tmp_instance = #name{.. Default::default()};
//                         INSTANCE = mem::transmute(Box::new(tmp_instance));
//                     });
//                     INSTANCE
//                 }
//             }

//             fn instance() -> Arc<Box<#name>> {
//                 unsafe {
//                     static mut INSTANCE: *const Arc<Box<#name>> = 0 as *const Arc<Box<#name>>;
//                     static ONCE: Once = Once::new();

//                     ONCE.call_once(||{
//                         let tmp_instance = Arc::new(Box::new (#name{.. Default::default()}));
//                         INSTANCE = mem::transmute(Box::new(tmp_instance));
//                     });

//                     (*INSTANCE).clone()
//                 }
//             }
//         }
//     }
// }

#[proc_macro_derive(MakeSingletonThreadSafe)]
pub fn make_singleton_thread_safe(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_make_singelton_thread_safe(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_make_singelton_thread_safe(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        use std::sync::{Arc, Mutex, Once};
        use std::{mem};

        trait MakeSingletonThreadSafeTemplateTrait {
            fn instance_thread_safe() -> Arc<Mutex<#name>>;
        }

        impl MakeSingletonThreadSafeTemplateTrait for #name {
            fn instance_thread_safe() -> Arc<Mutex<#name>> {
                unsafe {
                    static mut INSTANCE: *const Arc<Mutex<#name>> = 0 as *const Arc<Mutex<#name>>;
                    static ONCE: Once = Once::new();

                    ONCE.call_once(||{
                        let tmp_instance = Arc::new(Mutex::new(#name{.. Default::default()}));
                        INSTANCE = mem::transmute(Box::new(tmp_instance));
                    });

                    (*INSTANCE).clone()
                }
            }
        }
    }
}
