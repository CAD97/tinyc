extern crate proc_macro;

use {
    proc_macro2::{Span, TokenStream},
    quote::{quote, quote_spanned},
    std::{env, fs::File, io::prelude::*, path::PathBuf},
    syn::parse::Parse,
};

struct AttrArgs {
    ser: syn::ExprPath,
    de: syn::ExprPath,
    file: syn::LitStr,
}

impl Parse for AttrArgs {
    fn parse(input: &syn::parse::ParseBuffer<'_>) -> syn::parse::Result<Self> {
        mod kw {
            syn::custom_keyword!(exact);
            syn::custom_keyword!(file);
            syn::custom_keyword!(ser);
            syn::custom_keyword!(de);
        }

        // TODO: add `superset` mode where actual is "at least" expected
        let _: kw::exact = input.parse()?;
        let _: syn::Token![,] = input.parse()?;

        let _: kw::ser = input.parse()?;
        let _: syn::Token![=] = input.parse()?;
        let ser: syn::ExprPath = input.parse()?;
        let _: syn::Token![,] = input.parse()?;

        let _: kw::de = input.parse()?;
        let _: syn::Token![=] = input.parse()?;
        let de: syn::ExprPath = input.parse()?;
        let _: syn::Token![,] = input.parse()?;

        let _: kw::file = input.parse()?;
        let _: syn::Token![=] = input.parse()?;
        let file: syn::LitStr = input.parse()?;

        Ok(AttrArgs { ser, de, file })
    }
}

struct Test<'a> {
    name: &'a str,
    input: &'a str,
    output: &'a str,
}

// TODO: make this actually give useful parse errors
fn slice_tests(s: &str) -> Vec<Test<'_>> {
    assert!(
        s.ends_with('\n'),
        "test file needs to end with trailing newline"
    );
    let (tests, tail) = s.split_at(s.rfind("\n...").unwrap_or(0));
    assert!(
        tail.trim() == "..." || tail.trim().is_empty(),
        "test file should end with test terminator `...`"
    );

    tests
        .split("\n...\n")
        .map(|s| {
            let (head, output) =
                s.split_at(s.find("\n---\n").expect(
                    "test should have document marker `---` separating header and expected",
                ));
            let output = &output[5..]; // strip separator
            let (name, input) = head.split_at(
                head.find("\n===\n")
                    .expect("test should have header marker `===` separating name and input"),
            );
            let input = &input[5..]; // strip separator
            Test {
                name: name.trim(),
                input: input.trim(),
                output: output.trim(),
            }
        })
        .collect()
}

#[proc_macro_attribute]
pub fn tests(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut errs = TokenStream::new();

    let fun = syn::parse_macro_input!(item as syn::ItemFn);
    let AttrArgs{ ser, de, file } = syn::parse_macro_input!(attr as AttrArgs);
    let fn_name = &fun.sig.ident;
    let tested_type = match &fun.sig.output {
        syn::ReturnType::Type(_, r#type) => (**r#type).clone(),
        syn::ReturnType::Default => syn::parse_str("()").unwrap(),
    };

    let tests_path = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR")
            .unwrap_or_else(|e| panic!("expected `CARGO_MANIFEST_DIR`, {}", e)),
    )
    .join(file.value());
    let tests_source = match File::open(&tests_path) {
        Ok(mut f) => {
            let mut s =
                String::with_capacity(f.metadata().map(|m| m.len() as usize + 1).unwrap_or(0));
            match f.read_to_string(&mut s) {
                Ok(_) => {}
                Err(e) => {
                    let e = format!("failed to read file: {}", e);
                    errs.extend(quote_spanned! {file.span()=>
                        compile_error! { #e }
                    });
                    return errs.into();
                }
            }
            s
        }
        Err(e) => {
            let e = format!("failed to open file: {}", e);
            errs.extend(quote_spanned! {file.span()=>
                compile_error! { #e }
            });
            return errs.into();
        }
    };
    let tests = slice_tests(&tests_source);

    let filepath = tests_path.to_string_lossy().to_string();
    let filename = tests_path.file_stem().unwrap().to_string_lossy();
    let testing_fn = syn::Ident::new(&filename, Span::call_site());

    let mut tts = quote!(#fun);
    tts.extend(quote! {
        fn #testing_fn(expected: &str, actual: &str) -> Result<(), Box<dyn ::std::error::Error>> {
            const _: &str = include_str!(#filepath);

            #[derive(Debug)]
            struct TestFailure;
            impl ::std::error::Error for TestFailure {}
            impl ::std::fmt::Display for TestFailure {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }

            let actual = #ser(&#fn_name(actual))?;
            let expected = #ser(&#de::<#tested_type>(expected)?)?; // normalize
            assert_eq!(actual, expected);
            Ok(())
        }
    });

    for test in tests {
        let Test {
            name,
            input,
            output,
        } = test;
        let test_name = syn::Ident::new(
            &format!("{}_{}", filename, name.replace(' ', "_")),
            Span::call_site(),
        );
        tts.extend(quote! {
            #[test]
            fn #test_name() -> Result<(), Box<dyn ::std::error::Error>> {
                #testing_fn(#output, #input)
            }
        })
    }

    if errs.is_empty() {
        tts.into()
    } else {
        errs.into()
    }
}
