extern crate playground;

use std::env;
use std::iter;
use playground::{ArgDef, parse, parse_subcommand, ParseError, help_arg, version_arg};
use std::process;

fn main() {
    if let Some(exit_code) = epub_main() {
        process::exit(exit_code);
    }
}

#[allow(unused)]
fn epub_main() -> Option<i32> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let description = "
        Program to create ePub e-book files.
    ";
    
    match parse("epub", &args, vec![
        ArgDef::cmd("create", |program, args| {
            let mut spec_file = String::new();
            let mut target_path: Option<String> = None;
            let mut is_raw_spec = false;
            
            parse_subcommand(program, args, vec![
                ArgDef::pos("spec_file", &mut spec_file)
                    .help("The TOML specification of the book"),
                ArgDef::option("target_path", &mut target_path).short("t")
                    .help("
                        A specific path to compile the ePub to. Defaults to a
                        name/author coupling in the current working directory
                    "),
                ArgDef::flag("is_raw_spec", &mut is_raw_spec).short("r")
                    .help("
                        Interpret the spec-file argument as the contents of the 
                        specification file, instead of a path to it.
                    "),
                help_arg("
                    Compiles an ePub from a markdown source and a TOML specification. The files in
                    the specification are sought relatively to the location of the specification
                    file, so use absolute paths when needed. If no arguments are given, the
                    created file will be found in the active working directory.
                "),
            ])?;
            
            println!("Creating epub from spec: '{}' (target_path: {:?}, is raw spec?: {})", 
                spec_file, target_path, is_raw_spec);
            
            Ok(())
        })
        .help("Creates a new ePub from a given specification."),
        
        ArgDef::cmd("example", |program, args| {
            parse_subcommand(program, args, vec![])?;
            Ok(())
        })
        .help("Prints a template for an ePub specification file."),
        
        ArgDef::cmd("from_folder", |program, args| {
            let mut folder = String::new();
            
            parse(program, args, vec![
                ArgDef::pos("folder", &mut folder)
                    .help("The folder to load images from."),
                
                help_arg(
                "
                    Creates a simple epub from the images in the given folder.
                    This is useful for quickly creating rather bad comic epubs.
                "),
            ])?;
            
            println!("Creating epub from folder '{}'...", folder);
            
            Ok(())
        })
        .help("Creates a simple ePub from the images in the given folder."),
        
        help_arg(description).short("h"),
        version_arg(),
    ]) {
        Ok(_) => {},
        Err(ParseError::Interrupted(name)) => {
            println!("Parse interrupted: <{}>", name);
            return None;
        }
        Err(ParseError::InvalidDefinitions(msg)) => {
            panic!(msg);
        }
        Err(ParseError::ParseFailed(msg, help)) => {
            println!("Parse failed: {}", msg);
            help.print_usage();
            return Some(1);
        }
        Err(ParseError::SubParseFailed) => {
            return Some(1);
        }
    };
    
    None
}

#[allow(unused)]
fn argonaut_main() -> Option<i32> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    
    let mut first = String::new();
    let mut second = String::new();
    let mut third = false;
    let mut cool_thing: Option<String> = None;
    let mut stars = 0;
    let mut verbose = false;
    let mut numbers: Vec<i32> = Vec::new();
    
    let description = "
        Test program for an argument parsing library.
        
        Showcases the features of the library.
    ";
    
    match parse("argonaut", &args, vec![
        ArgDef::pos("first", &mut first)
            .help("The first argument."),
        ArgDef::pos("second", &mut second)
            .help("The second argument."),
        ArgDef::pos("third-is-better", &mut third)
            .help("Whether the third argument is better than the rest."),
        ArgDef::trail("numbers", true, &mut numbers)
            .help("A bunch of numbers used for nefarious machinations."),
        
        ArgDef::option("cool", &mut cool_thing)
            .help("Something that you think is cool enough to pass."),
        ArgDef::count("star", &mut stars).short("s")
            .help("How many stars does this library deserve?"),
        ArgDef::flag("verbose", &mut verbose).short("v")
            .help("Print as much information as possible."),
        
        help_arg(description).short("h"),
        version_arg(),
    ]) {
        Ok(_) => {},
        Err(ParseError::Interrupted(name)) => {
            println!("Parse interrupted: <{}>", name);
            return None;
        }
        Err(ParseError::InvalidDefinitions(msg)) => {
            panic!(msg);
        }
        Err(ParseError::ParseFailed(msg, help)) => {
            println!("Parse failed: {}", msg);
            help.print_usage();
            return Some(1);
        }
        Err(ParseError::SubParseFailed) => {
            return Some(1);
        }
    };
    
    println!("First:   {}", first);
    println!("Second:  {}", second);
    println!("Third is better?:   {}", third);
    println!("");
    println!("Numbers: {:?}", numbers);
    if verbose {
        println!("VERBOSE!");
    }
    if let Some(cool) = cool_thing {
        println!("Got a real cool {} :u!", cool);
    } else {
        println!("Nothing's cool anymore");
    }
    println!("Library rating: {}", iter::repeat('*').take(stars).collect::<String>());
    
    None
}
