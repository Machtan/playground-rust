
# Macros

## `:ty` vs. `:ident` for type arguments.
When the macro generates a module, `ident` should be preferred, as it is possible to generate use paths from identifiers, but not from types 

Valid: `use super::$ident`
Invalid: `use super::$ty`

`:ty` should probably be preferred for the general case, as the syntax hint is more specific about the expected argument.

# Current problem
Currently the borrow checker is apparently not aware that members of 'Sim' are disjoint (cannot read proc members at same time as having write access to a component (here CAge)). 

It might be that my trait system doesn't describe this properly, so I'll have to look into that.
