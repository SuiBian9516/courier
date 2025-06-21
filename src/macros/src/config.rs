use syn::{punctuated::Punctuated, Attribute, Expr, GenericParam, Generics, Lit, Meta, Token, Type, TypeParamBound};

pub fn get_rename(attrs: &[Attribute], ident_name: String) -> String {
    for attr in attrs {
        if !attr.path().is_ident("config") {
            continue;
        }

        match &attr.meta {
            Meta::List(list) => {
                for meta in list
                    .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    .unwrap_or_default()
                {
                    if let Meta::NameValue(name_value) = meta {
                        if name_value.path.is_ident("rename") {
                            if let Expr::Lit(expr_lit) = &name_value.value {
                                if let Lit::Str(s) = &expr_lit.lit {
                                    return s.value();
                                }
                            }
                            panic!("`rename` value must be a string literal");
                        }
                    }
                }
            }
            _ => continue,
        }
    }

    ident_name
}

pub fn get_default(attr: &Attribute, ty: &Type) -> Option<Expr> {
    if !attr.path().is_ident("config") {
        return None;
    }

    match &attr.meta {
        Meta::List(list) => {
            for meta in list
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap_or_default()
            {
                match meta {
                    Meta::NameValue(name_value) if name_value.path.is_ident("default") => {
                        return Some(name_value.value.clone());
                    }
                    Meta::Path(path) if path.is_ident("default") => {
                        return Some(syn::parse_quote!(<#ty as Default>::default()));
                    }
                    _ => continue,
                }
            }
        }
        _ => (),
    }
    None
}

pub fn add_trait_bounds(mut generics:Generics, bound: TypeParamBound) -> Generics{
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = param {
            type_param.bounds.push(bound.clone());
        }
    }
    generics
}