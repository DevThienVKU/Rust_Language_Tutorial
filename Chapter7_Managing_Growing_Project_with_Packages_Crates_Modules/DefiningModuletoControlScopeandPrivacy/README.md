# Defining Module to Control Scope and Privacy
## Modules Cheat Sheet

- Start from the crate root: (src/lib.rs for a library crate or src/main.rs for a binary crate)
- Declaring modules: In the crate root, declare new modules.
  - Declare a "garden" module with mod garden
  - Complier will look for the module's code in this places
    - Inline, within curly brackets that replace the semicolon following mod garden
    - In the file src/garden.rs
    - In the file src/garden/mod.rs
- Declaring submodules: any file other than the crate root. Can declare submodules.
  - Declare mod vegetables; in src/gaden.rs
  - Complier will look for the submodules's code within the directory named for the parent module in these places
    - Inline, directly following mod vegetables, within curly brackets instead of the semicolon
    - In the file src/garden/vegetables.rs
    - In the file src/garden/vegetables/mod.rs
- Paths to code in modules: Once a module is part of crate. Can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow.
  - Asparagus type in the garden/vegetables module would be found crate::garden::vegetables::Asparagus
- Public and Private
  - Code within a module is private from it's parent modules
  - To module public, declare it with pub mod instead of mod
  - To make items within a public module, use pub before their declarations
- The use keyword
  - Within a scope, the use keyword creates shortcuts to items to deduce repetition of long paths.
  - With crate::garden::vegetables::Asparagus, can create a shortcut with use crate::garden::vegetables::Asparagus;, and write Asparagus to make use

## Grouping Related Code in Modules

