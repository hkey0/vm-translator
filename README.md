# Jack VM to Hack assembly translator

## Usage

Target directory must contain at least one .vm file. Translator will insert a `bootstrap code` to set the Stack Pointer value and call Sys.init. 

```bash
cargo run -- [--directory]
```
