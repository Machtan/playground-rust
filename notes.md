
# Macros

## `:ty` vs. `:ident` for type arguments.
When the macro generates a module, `ident` should be preferred, as it is possible to generate use paths from identifiers, but not from types 

Valid: `use super::$ident`
Invalid: `use super::$ty`

`:ty` should probably be preferred for the general case, as the syntax hint is more specific about the expected argument.

## Importing traits into namespaced module
It seems as if `use super::<stuff>` shouldn't be used for things like components, since it means that declaring the same component twice results in a failed import, rather than a more meaningful error message.

# Current problem


# Previous problems
## Disjoint borrow of component stores
Currently the borrow checker is apparently not aware that members of 'Sim' are disjoint (cannot read proc members at same time as having write access to a component (here CAge)). 

It might be that my trait system doesn't describe this properly, so I'll have to look into that.

## Multiple accesses of component store with wrong process declaration
If the arguments to a macro-generated process contain the same component twice, it might be borrowed mutably twice...

**solution**
I *did* add a trait to mark it... But since the macro already imported each component from the `super` scope, it lead to an invalid import anyway.
I'll leave the traits in, since their help message is a bit better.
