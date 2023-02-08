extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote};

#[proc_macro_attribute]
pub fn requires_login(_attr: TokenStream, item: TokenStream) -> TokenStream {

    //NOTE: check for session
    let function_sig = item.to_string();
    if !(function_sig.contains("session") && function_sig.contains("Session")) {
	println!("WARNING, function implementing requires_login macro might not have passed in session, this is checked by parsing the function string so safe to ignore if it's called anything else");
    }

    //NOTE: parse the function
    let mut input = syn::parse_macro_input!(item as syn::ItemFn);
    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &mut input.sig;
    let body = &input.block;


    //NOTE: rebuild the function
    (quote! {
	#(#attrs)* #vis #sig
	{
	    //NOTE: Add to the head
	    match session.get::<i32>("user_id").unwrap()
	    {
		Some(user_id) => println!("user {} logged in", user_id),
		None =>  {
		    println!("Not logged in");
		    return(HttpResponse::Ok().body("not logged in"));
		}
	    }
	    #body
	}
    }).into()
}
