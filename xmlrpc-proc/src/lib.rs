use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemTrait, TraitItem, FnArg, AttributeArgs};

#[proc_macro_attribute]
pub fn xmlrpc(args: TokenStream, item_input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item_input as ItemTrait);
    let args = parse_macro_input!(args as AttributeArgs);
    let derive = args.first();
    let mut funcs = vec![];
    for each in &item.items {
        match each {
            TraitItem::Method(method) => {
                for _attr in &method.attrs {
                    //println!("{:#?}", syn::parse2(attr.tokens));
                }
                let ident = &method.sig.ident;
                let s = ident.to_string();
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
                funcs.push(quote!{ 
                    fn #ident(#inp) #oup {
                        let req = Request::new(#s)
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
        #item
        impl #name for #derive {
            #(#funcs)*
        }
    };
    output.into()
}
