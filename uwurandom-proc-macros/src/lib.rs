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
        let name = state.name.to_case(Case::Pascal);
        let name = Ident::new(&name, Span::call_site().into());
        variants = quote!(
            #variants
            #name,
        );
        let mut inner_match_arms = quote!();
        if state.total_probability == 1 {
            let choice = &state.choices[0];
            let next_state = input[choice.next_ngram].name.to_case(Case::Pascal);
            let next_state = Ident::new(&next_state, Span::call_site().into());
            let next_char = LitChar::new(choice.next_char, Span::call_site().into());
            match_arms = quote!(
                #match_arms
                Self::#name => (Self::#next_state, #next_char),
            );
            continue;
        }
        for choice in &state.choices {
            let next_state = input[choice.next_ngram].name.to_case(Case::Pascal);
            let next_state = Ident::new(&next_state, Span::call_site().into());
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
        enum StateMachine {
            #variants
        }
        impl StateMachine {
            fn generate(self, mut rng: impl ::rand_core::RngCore) -> (Self, char) {
                match self {
                    #match_arms
                }
            }
        }
    ).into()
}
