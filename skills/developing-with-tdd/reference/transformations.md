# Transformation Priority Premise

Robert C. Martin's list of the transformations that take production code from red to
green, simplest first. A green takes the highest one that is enough; a test that needs a
transformation low in the list is a sign another test should come before it.

1. **{} → nil**: from no code at all to code that returns nil, or the language's empty
   value.
2. **nil → constant**: a hardcoded value.
3. **constant → constant+**: a simple constant to a more complex one, such as a string to
   a longer string or a single value to a fixed list.
4. **constant → scalar**: a constant replaced by a variable or an argument.
5. **statement → statements**: more unconditional statements.
6. **unconditional → if**: the execution path split.
7. **scalar → array**: a single value to a collection.
8. **array → container**: a plain collection to a richer structure, such as a map or a
   set.
9. **statement → tail recursion**.
10. **if → while**: a condition turned into a loop.
11. **statement → non-tail recursion**.
12. **expression → function**: an expression replaced by a function or an algorithm.
13. **variable → assignment**: a variable's value changed after it is set.
14. **case**: one more case added to an existing `if` or `switch`.

The last one is the one to watch: a new case keyed on a single test's data is code
mirroring the tests instead of generalising them.
