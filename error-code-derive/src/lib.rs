mod error_info;

use error_info::process_error_info;
// 为什么注册宏的时候用proc_macro::TokenStream，而不是proc_macro2::TokenStream？
use proc_macro::TokenStream;

// 使用proc_macro 注册ToErrorInfo派生宏，参数有error_info 属性
// 使用darling 注册和解析属性error_info，darling(attributes(error_info))的信息在FromDeriveInput中，
// 变体variants 上的 darling(attributes(error_info))的信息在FromVariant中
#[proc_macro_derive(ToErrorInfo, attributes(error_info))]
pub fn derive_to_error_info(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    process_error_info(input).into()
}
