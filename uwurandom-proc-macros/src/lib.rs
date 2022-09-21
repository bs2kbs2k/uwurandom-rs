use convert_case::{Case, Casing};
use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{Ident, LitChar};

use crate::json::MarkovArr;

mod json;

#[proc_macro]
pub fn gen_fsm(item: TokenStream) -> TokenStream {
    let input: Vec<MarkovArr> = serde_json::from_str(&format!("[{}]", item)).unwrap();
    let mut match_arms = quote!();
    let mut variants = quote!();
    for state in input.iter() {
        let name = to_ident(&state.name);
        variants = quote!(
            #variants
            #name,
        );
        let mut inner_match_arms = quote!();
        if state.total_probability == 1 {
            let choice = &state.choices[0];
            let next_state = to_ident(&input[choice.next_ngram].name);
            let next_char = LitChar::new(choice.next_char, Span::call_site().into());
            match_arms = quote!(
                #match_arms
                Self::#name => (Self::#next_state, #next_char),
            );
            continue;
        }
        for choice in &state.choices {
            let next_state = to_ident(&input[choice.next_ngram].name);
            let cumulative_probability = choice.cumulative_probability - 1;
            let next_char = LitChar::new(choice.next_char, Span::call_site().into());
            inner_match_arms = quote!(
                #inner_match_arms
                0..=#cumulative_probability => (Self::#next_state, #next_char),
            )
        }
        let total_probability = state.total_probability;
        match_arms = quote!(
            #match_arms
            Self::#name => match rng.next_u32() % #total_probability {
                #inner_match_arms
                _ => unreachable!(),
            },
        );
    }
    quote!(
        #[derive(Debug, Clone, Copy)]
        pub enum StateMachine {
            #variants
        }
        impl StateMachine {
            pub fn generate(self, mut rng: impl ::rand_core::RngCore) -> (Self, char) {
                match self {
                    #match_arms
                }
            }
        }
    )
    .into()
}

fn to_ident(name: &str) -> Ident {
    Ident::new(&name.replace(';', " semicolon").to_case(Case::Pascal), Span::call_site().into())
}
