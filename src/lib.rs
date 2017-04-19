/*!
A simple argument parsing library.

# Lifetimes
`'def`: `argument definition`

The lifetime of argument definition passed to `parse`


`'tar`: `target`

The lifetime of target pointers used when defining arguments.
*/

#![feature(conservative_impl_trait)]
#![feature(unicode)]

extern crate std_unicode;

mod argdef;
mod help;
mod parse;

pub use argdef::{ArgDef, ArgDefKind, SingleTarget, CollectionTarget, OptionTarget};
pub use parse::{parse, ParseError};

use std::borrow::{Cow};

/*
DESIGN: Do I wait with assigning values until all arguments have been 'satisfied'?
Or do I just start parsing/assigning as soon as possible so that bad arguments
are caught faster?
For now it'll be 2, since that seems simpler

# option 1
read through the arguments and assign each to a matching option
if an interrupt is encountered: 
    run the callback and return the interrupt
validate each argument (add 'validate' to the interface)
go through and parse every value into its target
return success
*/

/*
Tasks
- DONE Usage generator (printer)
- DONE Help generator (printer)
- Simple subcommand abstraction
*/

/// Creates a default help interrupt for `--help`.
pub fn default_help_interrupt<'def, 'tar, P, D>(progname: P, description: D)
        -> ArgDef<'def, 'tar> 
  where D: Into<Cow<'static, str>>,
        P: Into<Cow<'static, str>>
{
    let progname = progname.into();
    let description = description.into();
    ArgDef::interrupt("help", move |help| {
        help.print_help(progname.as_ref(), description.as_ref());
    }).help("Print this message and abort.")
}

/// Creates a default version interrupt for `--version`.
pub fn default_version_interrupt<'def, 'tar>() -> ArgDef<'def, 'tar> {
    ArgDef::interrupt("version", |_| {
        println!("{}", option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0"));
    }).help("Print version string and abort.")
}
