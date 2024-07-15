# Jack VM to Hack assembly translator

This translator passes all tests provided in the project files.
(FunctionCalls & ProgramFlow)

## Usage

Target directory must contain at least one .vm file. Translator will insert a `bootstrap code` to set the Stack Pointer value and call Sys.init. 

```bash
cargo run -- [--directory]
```
