#![allow(non_snake_case, unused)]
use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use quote::quote_spanned;
use syn::parse_macro_input;
use syn::DeriveInput;
use syn::Ident;
use syn::Lit;
use syn::{Expr, Token, Type};

macro_rules! my_quote {
    ($($t:tt)*) => (quote_spanned!(proc_macro2::Span::call_site() => $($t)*))
}

#[proc_macro_attribute]
pub fn default(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemStruct);
    let name = &input.ident;
    let field_name = input.fields.iter().map(|field| &field.ident);

    let output = quote! {
        impl Default for #name {
            fn default() -> Self {
                #name {
                    #(
                        #field_name: Default::default(),
                    )*
                }
            }
        }
    };

    TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn ftdc_field_string(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemStruct);
    let name = &input.ident;

    let FieldName0 = input.fields.iter().map(|field| &field.ident);
    let FieldName1 = FieldName0.clone();
    let field_name = input.fields.iter().map(|field| match field.ident {
        Some(ref ident) => {
            let name = ident.to_string().to_snake_case();
            Some(Ident::new(&name, ident.span()))
        }
        None => None,
    });
    let with_field_name = input.fields.iter().map(|field| match field.ident {
        Some(ref ident) => {
            let name = ident.to_string().to_snake_case();
            let name = format!("with_{}", name);
            Some(Ident::new(&name, ident.span()))
        }
        None => None,
    });
    let field_type = input.fields.iter().map(|field| &field.ty);

    let output = quote! {
        impl #name {
            pub fn new() -> Self {
                Default::default()
            }

            #(
                pub fn #with_field_name<S: AsRef<str>>(&mut self, #field_name: S) -> &mut Self
                {
                    let #field_name = #field_name.as_ref();

                    let mut data = [0; size_of::<#field_type>()];

                    debug_assert!(#field_name.len() <= size_of::<#field_type>());

                    for (idx, v) in #field_name.chars().enumerate() {
                        data[idx] = v as _;
                    }

                    self.#FieldName0 = data;
                    self
                }
            )*
        }
    };

    TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn ftdc_field_builder(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemStruct);
    let name = &input.ident;

    let FieldName0 = input.fields.iter().map(|field| &field.ident);
    let FieldName1 = FieldName0.clone();
    let field_name = input.fields.iter().map(|field| match field.ident {
        Some(ref ident) => {
            let name = ident.to_string().to_snake_case();
            Some(Ident::new(&name, ident.span()))
        }
        None => None,
    });
    let with_field_name = input.fields.iter().map(|field| match field.ident {
        Some(ref ident) => {
            let name = ident.to_string().to_snake_case();
            let name = format!("with_{}", name);
            Some(Ident::new(&name, ident.span()))
        }
        None => None,
    });
    let field_type = input.fields.iter().map(|field| &field.ty);

    let output = quote! {
        impl #name {
            pub fn new() -> Self {
                Default::default()
            }

            #(
                pub fn #with_field_name<S: Into<String>>(&mut self, #field_name: S) -> &mut Self
                {
                    let #field_name = #field_name.into();

                    let mut data = [0; size_of::<#field_type>()];

                    debug_assert!(#field_name.len() <= size_of::<#field_type>());

                    for (idx, v) in #field_name.chars().enumerate() {
                        data[idx] = v as _;
                    }

                    self.#FieldName0 = data;
                    self
                }
            )*

            pub fn build(&self) -> Self {
                #name {
                    #(
                        #FieldName1: self.#FieldName1,
                    )*
                    ..Default::default()
                }
            }
        }
    };

    TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn ftdc_field(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemStruct);
    let name = &input.ident;

    TokenStream::from(quote!())
}

#[proc_macro_attribute]
pub fn ftdc_field_serde(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemStruct);
    let name = &input.ident;

    let FieldName = input.fields.iter().map(|field| &field.ident);
    let FieldName0 = input.fields.iter().map(|field| &field.ident);
    let FieldName1 = FieldName.clone();
    let FieldName2 = FieldName.clone();
    let FieldName3 = FieldName.clone();
    let FieldName4 = FieldName.clone();
    let size = FieldName.len();
    let _field_type = input.fields.iter().map(|field| &field.ty);

    let output = quote! {

        impl Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: Serializer,
            {
                let mut state = serializer.serialize_struct(
                    stringify!(#name), #size)?;
                #(
                    state.serialize_field(stringify!(#FieldName), &self.#FieldName)?;
                )*
                state.end()
            }
        }

        impl<'de> Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: Deserializer<'de>,
            {
                enum Field {
                    #(
                        #FieldName0,
                    )*
                };

                impl<'de> Deserialize<'de> for Field {
                    fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                        where D: Deserializer<'de>
                    {
                        struct FieldVisitor;
                        impl<'de> Visitor<'de> for FieldVisitor {
                            type Value = Field;

                            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                                formatter.write_str("")
                            }

                            fn visit_str<E>(self, value: &str) -> Result<Field, E>
                                where E: de::Error
                            {
                                match value {
                                    #(
                                        stringify!(#FieldName1) => Ok(Field::#FieldName1),
                                    )*
                                    _ => Err(de::Error::unknown_field(value, FIELDS)),
                                }
                            }
                        }

                        deserializer.deserialize_identifier(FieldVisitor)
                    }
                }

                struct InstrumentVisitor;
                impl <'de> Visitor<'de> for InstrumentVisitor {
                    type Value = #name;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("struct")
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<#name, V::Error>
                        where V: SeqAccess<'de>,
                    {
                        let mut r: #name = Default::default();
                        #(
                            let vm = seq.next_element()?
                                .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                            r.#FieldName2 = vm;
                        )*
                        Ok(r)
                    }

                    fn visit_map<V>(self, mut map: V) -> Result<#name, V::Error>
                        where V: MapAccess<'de>
                    {
                        let mut r: #name = Default::default();

                        while let Some(key) = map.next_key::<Field>()? {
                            match key {
                                #(
                                    Field::#FieldName3 => {
                                        r.#FieldName3 = map.next_value()?;
                                    }
                                )*
                            }
                        }

                        Ok(r)
                    }
                }

                const FIELDS: &'static [&'static str] = &[
                    #(
                        stringify!(#FieldName4),
                    )*
                ];

                deserializer.deserialize_struct(stringify!(#name), FIELDS, InstrumentVisitor)
            }
        }
    };

    TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn ftdc_field_ser(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemStruct);
    let name = &input.ident;

    let FieldName = input.fields.iter().map(|field| &field.ident);
    let size = FieldName.len();
    let _field_type = input.fields.iter().map(|field| &field.ty);

    let output = quote! {
        impl Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: Serializer,
            {
                let mut state = serializer.serialize_struct(
                    stringify!(#name), #size)?;
                #(
                    state.serialize_field(stringify!(#FieldName), &self.#FieldName)?;
                )*
                state.end()
            }
        }
    };

    TokenStream::from(output)
}

fn big_array_p(ty: &Type) -> bool {
    match ty {
        Type::Array(n) => match &n.len {
            Expr::Lit(lit) => match &lit.lit {
                Lit::Int(size) => {
                    if let Ok(size) = size.base10_parse::<u32>() {
                        // big array size > 32
                        if size > 32 {
                            return true;
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        },
        _ => {}
    }

    false
}

#[proc_macro_derive(FtdcSerialize)]
pub fn ftdc_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemStruct);
    let name = &input.ident;
    let size = input.fields.len();

    let field_name = input
        .fields
        .iter()
        .filter(|&field| !big_array_p(&field.ty))
        .map(|field| &field.ident);

    let n_field_name = input
        .fields
        .iter()
        .filter(|&field| big_array_p(&field.ty))
        .map(|field| &field.ident);

    let n_field_ty = input
        .fields
        .iter()
        .filter(|&field| big_array_p(&field.ty))
        .map(|field| &field.ty);

    let output = quote! {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: _serde::Serializer,
            {
                let mut state = serializer.serialize_struct(
                    stringify!(#name), #size)?;
                #(
                    state.serialize_field(stringify!(#field_name), &self.#field_name)?;
                )*

                #(
                    state.serialize_field(stringify!(#n_field_name), {
                        #[doc(hidden)]
                        struct __SerializeWith<'__a> {
                            values: (&'__a #n_field_ty,),
                            phantom: _serde::__private::PhantomData<#name>,
                        }
                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                            fn serialize<__S>(
                                &self,
                                __s: __S,
                            ) -> _serde::__private::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                BigArray::serialize(self.values.0, __s)
                            }
                        }

                        &__SerializeWith {
                            values: (&self.#n_field_name,),
                            phantom: _serde::__private::PhantomData::<#name>,
                        }

                    })?;
                )*

                state.end()
            }
        }
    };

    TokenStream::from(output)
}
