use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input, parse_quote};

mod config;

/// Macro that helps structures implement [`Serializable`](::config::Serializable)
#[proc_macro_derive(Serialize, attributes(config))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = input.ident;
  let generic = config::add_trait_bounds(input.generics.clone(), parse_quote!(Serializable));
  let (impl_generics, ty_generics, where_clause) = generic.split_for_impl();

  let fields = match input.data {
    Data::Struct(ref data) => match data.fields {
      Fields::Named(ref f) => &f.named,
      _ => panic!("Only support named structs"),
    },
    _ => panic!("Only support structs"),
  };

  let serialize_fields = fields.iter().map(|field| {
    let field_name = &field.ident;
    let rename = config::get_rename(&field.attrs, field_name.as_ref().unwrap().to_string());

    quote! {
      map.insert(String::from(#rename), self.#field_name.serialize());
    }
  });

  let expanded = quote! {
    impl #impl_generics ::config::Serializable for #name #ty_generics #where_clause {
      fn serialize(&self) -> ::config::value::Value{
        let mut map = ::config::map::ObjectImpl::new();
        #(#serialize_fields)*
        ::config::value::Value::Object(map)
      }
    }
  };

  TokenStream::from(expanded)
}

/// Macro that helps structures implement [`Deserializable`](::config::Deserializable)
#[proc_macro_derive(Deserialize, attributes(config))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = input.ident;
  let generic = config::add_trait_bounds(input.generics.clone(), parse_quote!(Serializable));
  let (impl_generics, ty_generics, where_clause) = generic.split_for_impl();

  let fields = match input.data {
    Data::Struct(ref data) => match data.fields {
      Fields::Named(ref f) => &f.named,
      _ => panic!("Only support named structs"),
    },
    _ => panic!("Only support structs"),
  };

  let deserialize_fields = fields.iter().map(|field| {
    let field_name = &field.ident;
    let rename = config::get_rename(&field.attrs, field_name.as_ref().unwrap().to_string());
    let ty = &field.ty;

    if let Some(expr) = field.attrs.iter().find(|c| c.path().is_ident("config")).and_then(|attr| config::get_default(attr, ty)) {
      return quote! {
          #field_name: {
              if let Some(data) = map.get(#rename){
                  ::config::Deserializable::deserialize(data)?
              }else{
                  #expr
              }
          }
      };
    }

    quote! {
      #field_name: {
        if let Some(data) = map.get(#rename){
          ::config::Deserializable::deserialize(data)?
        }else{
          return Err(::config::DeserializableError::MissingField(#rename));
        }
      }
    }
  });

  let expanded = quote! {
    impl #impl_generics ::config::Deserializable for #name #ty_generics #where_clause {
      fn deserialize(v: &::config::value::Value) -> Result<Self, ::config::DeserializableError>{
        match v{
          ::config::value::Value::Object(map) => {
            Ok(Self {
              #(#deserialize_fields),*
            })
          },
          _ => Err(::config::DeserializableError::UnmatchedValue("Object"))
        }
      }
    }
  };
  TokenStream::from(expanded)
}
