
# Macros

## `:ty` vs. `:ident` for type arguments.
When the macro generates a module, `ident` should be preferred, as it is possible to generate use paths from identifiers, but not from types 

Valid: `use super::$ident`
Invalid: `use super::$ty`

`:ty` should probably be preferred for the general case, as the syntax hint is more specific about the expected argument.

