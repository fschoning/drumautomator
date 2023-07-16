use fehler::throws;

use notation_proto::prelude::{Track, TrackKind};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Error, ParseStream};
use syn::Ident;

use crate::proto::entry::EntryDsl;

use super::id::IdDsl;

pub struct TrackDsl {
    pub id: IdDsl,
    pub kind: Ident,
    pub entries: Vec<EntryDsl>,
}

impl TrackDsl {
    #[throws(Error)]
    pub fn parse_without_brace(input: ParseStream) -> Self {
        let id = input.parse()?;
        let kind = input.parse()?;
        let entries = EntryDsl::parse_vec(input)?;
        TrackDsl { id, kind, entries }
    }
}

impl ToTokens for TrackDsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let TrackDsl { id, kind, entries } = self;
        let kind_quote = kind.to_string();
        let entries_quote = EntryDsl::quote_vec(entries);
        tokens.extend(quote! {
            Track::new(#id.into(), TrackKind::from_ident(#kind_quote), #entries_quote)
        });
    }
}

impl TrackDsl {
    pub fn to_proto(&self) -> Track {
        let mut entries = Vec::new();
        for entry in self.entries.iter() {
            entry.add_proto(&mut entries);
        }
        Track::new(
            self.id.id.clone(),
            TrackKind::from_ident(self.kind.to_string().as_str()),
            entries,
        )
    }
}
