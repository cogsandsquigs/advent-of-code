use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{meta, parse_macro_input, ItemFn, LitInt};

#[proc_macro_attribute]
pub fn solution(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut day: usize = 0;
    let mut part: usize = 0;

    let arg_parser = meta::parser(|meta| {
        if meta.path.is_ident("day") {
            let raw: LitInt = meta.value()?.parse()?;
            day = raw
                .token()
                .to_string()
                .parse()
                .expect("This should be a number!");

            Ok(())
        } else if meta.path.is_ident("part") {
            let raw: LitInt = meta.value()?.parse()?;
            part = raw
                .token()
                .to_string()
                .parse()
                .expect("This should be a number!");

            Ok(())
        } else {
            Err(meta.error("Unsupported property"))
        }
    });

    parse_macro_input!(args with arg_parser);
    let input: ItemFn = parse_macro_input!(item);

    // Actual function name so we can replace it with our wrapper
    let fn_name = input.clone().sig.ident;
    let fn_args = input.clone().sig.inputs;

    // The inner function stuff that actually computes the solution
    let inner_fn_name = format_ident!("{}_inner", fn_name);
    let inner_fn_block = input.block;
    let inner_fn_return = input.sig.output;

    quote! {
        pub fn #fn_name(is_test: bool) {
            use utils::files::read;

            let year: usize = env!("CARGO_PKG_NAME")
                .split('-')
                .nth(1)
                .expect("Format of the year folder should be 'aoc-YYYY'")
                .parse()
                .expect("Format of the year folder should be 'aoc-YYYY'");
            let input = read(&format!("{}/input/day-{:0>2}/input{}.txt", year, #day, if is_test { ".test" } else { "" }))
                .or_else(|_| read(&format!("{}/input/day-{}/input{}.txt", year, #day, if is_test { ".test" } else { "" })))
                .or_else(|_| read(&format!("aoc-{}/input/day-{}/input{}.txt", year, #day, if is_test { ".test" } else { "" })))
                .or_else(|_| read(&format!("aoc-{}/input/day-{}/input{}.txt", year, #day, if is_test { ".test" } else { "" })))
                .unwrap();

            let start = std::time::Instant::now();
            let result = #inner_fn_name(&input);
            let elapsed = start.elapsed();

            println!("{} day {}, part {} solution: {}", year, #day, #part, result);
            println!(
                "Time elapsed: {}s {}ms {}µs {}ns",
                elapsed.as_secs(),
                elapsed.as_millis() % 1000, // get only the last 3 digits, which are the milliseconds
                elapsed.as_micros() % 1000, // get only the last 3 digits, which are the microseconds
                elapsed.as_nanos() % 1000, // get only the last 3 digits, which are the nanoseconds
            );

            // result.to_string()
        }

        fn #inner_fn_name(#fn_args) #inner_fn_return {
            #inner_fn_block
        }
    }
    .into()
}
