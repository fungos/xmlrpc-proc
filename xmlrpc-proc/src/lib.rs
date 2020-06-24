use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemTrait, TraitItem, FnArg, Attribute, AttributeArgs, Meta, NestedMeta, Lit};

fn parse_child_attribute(attr: &Attribute, ident: &str, key: &str) -> Option<String> {
    if let Ok(meta) = attr.parse_meta() {
        if let Meta::List(list) = meta {
            if list.path.is_ident(ident) {
                for nested_meta in &list.nested {
                    if let NestedMeta::Meta(ref submeta) = nested_meta {
                        if submeta.path().is_ident(key) {
                            if let Meta::NameValue(nv) = submeta {
                                match &nv.lit {
                                    Lit::Str(s) => return Some(s.value()),
                                    _ => return None,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

#[proc_macro_attribute]
pub fn xmlrpc(args: TokenStream, item_input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item_input as ItemTrait);
    let args = parse_macro_input!(args as AttributeArgs);
    let derive = args.first();
    let mut funcs = vec![];
    let mut prototypes = vec![];
    for each in &item.items {
        match each {
            TraitItem::Method(method) => {
                let mut method_override: Option<String> = None;
                for attr in &method.attrs {
                    if let Some(s) = parse_child_attribute(&attr, "xmlrpc", "method") {
                        method_override = Some(s);
                        break;
                    }
                }
                let ident = &method.sig.ident;
                let rpc_call = match method_override {
                    Some(v) => v,
                    None => ident.to_string(),
                };
                let inp = &method.sig.inputs;
                let oup = &method.sig.output;
                let mut args = vec![];
                for i in inp {
                    match i {
                        FnArg::Receiver(_) => {
                        },
                        FnArg::Typed(tt) => {
                            let t = &tt.pat;
                            args.push(quote!{ .arg(#t) });
                        }, 
                    };
                }
                // FIXME: not sure what happens, but when we have attributes in methods
                // the methods will be consumed and disappear. Here we rebuild them
                // to keep the trait definition intact.
                prototypes.push(quote!{
                    fn #ident(#inp) #oup;
                });
                funcs.push(quote!{ 
                    fn #ident(#inp) #oup {
                        let req = Request::new(#rpc_call)
                        #(#args)*
                        ;
                        req.call_url(&self.endpoint)
                    }
                });
            },
            _ => ()
        };
    };
    let name = &item.ident;
    let output = quote!{
        trait #name {
            #(#prototypes)*
        }
        impl #name for #derive {
            #(#funcs)*
        }
    };
    output.into()
}
