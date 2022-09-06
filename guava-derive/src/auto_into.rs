use proc_macro2::{TokenStream, Ident};
use quote::quote;

use syn::{DeriveInput, Type, Data, DataStruct, FieldsNamed, Fields, Field, GenericArgument, Path, TypePath};


struct Fd {
  name: Ident,
  ty: Type,
  optional: bool
}
/// 把一个 Field 转换成 Fd
impl From<Field> for Fd {
    fn from(f: Field) -> Self {
        let (optional, ty) = get_option_inner(&f.ty);
        Self {
            // 此时，我们拿到的是 NamedFields，所以 ident 必然存在
            name: f.ident.unwrap(),
            optional,
            ty: ty.to_owned(),
        }
    }
}

pub struct AutoIntoContext {
  name: Ident,
  fields: Vec<Fd>
}


impl AutoIntoContext {
    
    pub fn render(&self) -> TokenStream {
      let name = &self.name;
      let clean_name = format!("{}", name).trim_end_matches("VO").to_string();
      let struct_name = Ident::new(&format!("{}",name), name.span());
      // TODO 目标的 ident 需要传入, 也可以
      let target_name = Ident::new(&format!("{}Input",clean_name), name.span());

      let assigns = self.gen_assigns();

      quote!{
        impl std::convert::From<#struct_name> for #target_name {
          fn from(s: #struct_name) -> Self {
              #target_name {
                #(#assigns,)*
                ..#target_name::default()
              }
          }
        }
      }
    }

        // 比如：#field_name: self.#field_name.take().ok_or(" xxx need to be set!")
    fn gen_assigns(&self) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|Fd { name, optional, ty}| {
                if *optional {
                    return quote! {
                        #name: s.#name.take()
                    };
                }

                quote! {
                    #name: s.#name.into()
                }
            })
            .collect()
    }
}

impl From<DeriveInput> for AutoIntoContext {
    fn from(input: DeriveInput) -> Self {
        let name = input.ident;

        let fields = if let Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) = input.data
        {
            named
        } else {
            panic!("Unsupported data type");
        };

        let fds = fields.into_iter().map(Fd::from).collect();
        Self { name, fields: fds }
    }
}

// 如果是 T = Option<Inner>，返回 (true, Inner)；否则返回 (false, T)
fn get_option_inner(ty: &Type) -> (bool, &Type) {
    // 首先模式匹配出 segments
    if let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = ty
    {
        if let Some(v) = segments.iter().next() {
            if v.ident == "Option" {
                // 如果 PathSegment 第一个是 Option，那么它内部应该是 AngleBracketed，比如 <T>
                // 获取其第一个值，如果是 GenericArgument::Type，则返回
                let t = match &v.arguments {
                    syn::PathArguments::AngleBracketed(a) => match a.args.iter().next() {
                        Some(GenericArgument::Type(t)) => t,
                        _ => panic!("Not sure what to do with other GenericArgument"),
                    },
                    _ => panic!("Not sure what to do with other PathArguments"),
                };
                return (true, t);
            }
        }
    }
    (false, ty)
}