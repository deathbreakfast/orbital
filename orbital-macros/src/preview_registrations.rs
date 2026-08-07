//! `preview_registrations!` — crate-local static preview export table.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, Token};

struct PreviewRegistrationList {
    items: Punctuated<Expr, Token![,]>,
}

impl Parse for PreviewRegistrationList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            items: Punctuated::parse_terminated(input)?,
        })
    }
}

pub fn expand_preview_registrations(input: TokenStream) -> TokenStream {
    let PreviewRegistrationList { items } = parse_macro_input!(input as PreviewRegistrationList);
    let regs = items.iter();

    quote! {
        /// Static preview registrations exported by this crate for host catalogs.
        ///
        /// Behind `feature = "preview"`; empty when preview is disabled so production
        /// builds do not require registration symbols.
        #[cfg(feature = "preview")]
        pub fn all() -> &'static [&'static PreviewRegistration] {
            static REGS: &[&PreviewRegistration] = &[#(#regs),*];
            REGS
        }

        /// Empty table when the crate `preview` feature is off.
        #[cfg(not(feature = "preview"))]
        pub fn all() -> &'static [&'static PreviewRegistration] {
            &[]
        }
    }
    .into()
}
