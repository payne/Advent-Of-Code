
#![feature(slice_patterns)]

use std::env;

macro_rules! days {
    {
        $($Mod:ident),+; $Latest:ident
    } => {
        $(
            pub mod $Mod;
        )+
        pub mod $Latest;

        static CHALLENGES: &'static [fn()] = &[
            $( $Mod::main ),+, $Latest::main
        ];

        fn run_latest() {
            println!("+=================================+");
            println!("|Running Latest Challenge (Day {:0>2?})|", CHALLENGES.len());
            println!("+=================================+");
            $Latest::main();
            println!("");
        }
    };
}

days!(
    day1, day2, day3, day4; day5
);

fn main() {
    let mut args = env::args().skip(1).peekable();
    if args.peek().is_none() {
        run_latest()
    } else {
        for day in args {
            match day.parse::<usize>() {
                Ok(0) => run_latest(),
                Ok(day) if day <= CHALLENGES.len() => {
                    println!("+=============================+");
                    println!("|Running challenge from day {:0>2?}|", day);
                    println!("+=============================+");
                    CHALLENGES[day-1]();
                    println!("");
                },
                Ok(day) => println!("Day {:?}'s challenge has not been completed yet", day),
                Err(e) => println!("Unexpected argument. Expected day number: {:?}", e)
            }
        }
    }
}
