use std::borrow::Cow;
use argdef::{ArgDef, ArgDefKind};
use std_unicode::str::UnicodeStr;
use std::env;
use std::path::Path;

/// A collection of descriptions of the defined arguments.
#[derive(Debug)]
pub struct Help<'def> {
    /// Positional arguments.
    pub positional: Vec<(Cow<'def, str>, Option<Cow<'def, str>>)>,
    /// Trailing positional vararg.
    pub trail: Option<(Cow<'def, str>, bool, Option<Cow<'def, str>>)>,
    /// Optional arguments.
    pub options: Vec<(Cow<'def, str>, Option<Cow<'def, str>>, HelpOptKind, Option<Cow<'def, str>>)>,
    /// Is `--help` defined.
    pub help_defined: bool,
}

impl<'def> Help<'def> {
    /// Creates a new help object from the given descriptions.
    pub fn from_definitions<'tar>(definitions: &[ArgDef<'def, 'tar>]) -> Help<'def> {
        let mut positional = Vec::new();
        let mut trail = None;
        let mut options = Vec::new();
        let mut help_defined = false;
        for def in definitions {
            match def.kind {
                ArgDefKind::Positional { .. } => {
                    positional.push((def.name.clone(), def.help_desc.clone()));
                }
                ArgDefKind::Trail { optional, .. } => {
                    trail = Some((def.name.clone(), optional, def.help_desc.clone()));
                },
                ArgDefKind::Flag { ref short, .. } => {
                    options.push((
                        def.name.clone(), short.clone(), 
                        HelpOptKind::Flag, def.help_desc.clone()
                    ));
                }
                ArgDefKind::Count { ref short, .. } => {
                    options.push((
                        def.name.clone(), short.clone(), 
                        HelpOptKind::Count, def.help_desc.clone()
                    ));
                }
                ArgDefKind::OptArg { ref short, .. } => {
                    options.push((
                        def.name.clone(), short.clone(), 
                        HelpOptKind::OptArg, def.help_desc.clone()
                    ));
                }
                ArgDefKind::Interrupt { ref short, .. } => {
                    if def.name.as_ref() == "help" {
                        help_defined = true;
                    }
                    options.push((
                        def.name.clone(), short.clone(), 
                        HelpOptKind::Interrupt, def.help_desc.clone()
                    ));
                }
            }
        }
        Help { positional, trail, options, help_defined }
    }
    
    fn write_usage_into(&self, s: &mut String) {
        let bin = env::args().next().unwrap();
        let bin_name = Path::new(&bin).file_name().unwrap().to_string_lossy();
        s.push_str(bin_name.as_ref());
        
        if ! self.options.is_empty() {
            if self.help_defined {
                s.push_str(" [ --help | OPTIONS ]");
            } else {
                s.push_str(" [ OPTIONS ]");
            }
        }
        
        for &(ref name, _) in self.positional.iter() {
            s.push(' ');
            s.push_str(name.as_ref());
        }
        
        if let Some((ref name, optional, _)) = self.trail {
            s.push(' ');
            if optional {
                s.push_str(&format!("[{}...]", name));
            } else {
                s.push_str(&format!("{} [{}...]", name, name));
            }
        }
    }
    
    /// Generates a usage message for this program.
    pub fn usage_message(&self) -> String {
        let mut s = String::new();
        self.write_usage_into(&mut s);
        s
    }
    
    /// Prints a usage message for this program.
    pub fn print_usage(&self) {
        println!("Usage: {}", self.usage_message());
    }
    
    /// Generates a help message for this program, using the given program
    /// description. The description may be left blank.
    pub fn help_message(&self, description: &str) -> String {
        let mut s = String::from("Usage:\n  ");
        self.write_usage_into(&mut s);
        
        let has_description = description != "";
        let has_positional = (! self.positional.is_empty()) || self.trail.is_some();
        let has_optional = ! self.options.is_empty();
        if has_positional || has_optional || has_description {
            s.push_str("\n\n");
        }
        
        if has_description {
            // Trim the description
            let rev: Vec<_> = description.lines()
                .rev().skip_while(|&l| l == "" || l.is_whitespace()).collect();
            let lines = rev.into_iter()
                .rev().skip_while(|&l| l == "" || l.is_whitespace())
                .map(|line| line.trim());
            s.push_str("Description:\n");
            for line in lines {
                s.push_str("  ");
                s.push_str(line.trim());
                s.push('\n');
            }
        }
        
        if has_positional {
            s.push('\n');
            s.push_str("Positional arguments:\n");
            for &(ref name, ref help) in self.positional.iter() {
                s.push_str(&format!("  {}\n", name));
                if let &Some(ref help) = help {
                    s.push_str(&format!("    {}\n", help));
                }
                s.push('\n');
            }
            if let Some((ref name, optional, ref help)) = self.trail {
                s.push_str("  ");
                if optional {
                    s.push_str(&format!("[{}...]\n", name));
                } else {
                    s.push_str(&format!("{} [{}...]\n", name, name));
                }
                if let &Some(ref help) = help {
                    s.push_str(&format!("    {}\n", help));
                }
                s.push('\n');
            }
        }
        if has_optional {
            if ! has_positional {
                s.push('\n');
            }
            s.push_str("Optional arguments:\n");
            for &(ref name, ref short, kind, ref help) in self.options.iter() {
                s.push_str("  ");
                s.push_str("--");
                s.push_str(name.as_ref());
                if let &Some(ref short) = short {
                    s.push_str(", ");
                    s.push('-');
                    s.push_str(short.as_ref());
                }
                match kind {
                    HelpOptKind::OptArg => {
                        s.push(' ');
                        s.push_str(&name.as_ref().to_uppercase());
                    }
                    _ => {}
                }
                s.push('\n');
                if let &Some(ref help) = help {
                    s.push_str(&format!("      {}\n", help));
                    s.push('\n');
                }
            }
        }
        
        s
    }
    
    /// Prints a help message for this program, using the given program
    /// description. The description may be left blank.
    pub fn print_help(&self, description: &str) {
        println!("{}", self.help_message(description));
    }
}

/// Describes what kind of argument is expected.
#[derive(Debug, Clone, Copy)]
pub enum HelpOptKind {
    /// A flag. `./bin --verbose` => `true`
    Flag,
    /// A count. `./bin -v -v -v -v` => `4`
    Count,
    /// An option with a value. `./bin --eat-cake yes`
    OptArg,
    /// An interrupt. `./bin --help`
    Interrupt,
}
