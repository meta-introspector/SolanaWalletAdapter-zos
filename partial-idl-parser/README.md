## Simple anchor IDL Parser
A simple parser for partial part of anchor IDL without importing anchor types or causing build panics for WASM targets.

It only parses the `address` of the program and the `instructions` containing the instruction `name` and the `discriminant` which are useful for the scope of the `wallet-adapter` templates.


### Macro for creating a path to the IDL
The frontend is in the same workspace as the target directory which contains the anchor IDL directory.

The directory is in path `../../target/idl/`.

Pass the name of the program from `/programs` so that the file is located successfully.

Get the `AnchorIdlPartialData` data structure containing the IDL of an anchor example called `temp`

```rust,ignore
use partial_idl_parser::*;

const IDL: &str = idl_path!("temp");
```

If the directory is different you can use `idl_custom_path` macro:
```rust,ignore
use partial_idl_parser::*;

const IDL: &str = idl_custom_path!("../../target/custom_idl_dir", "temp");
```

### The `AnchorIdlPartialData`
Parsed IDL is stored within a `AnchorIdlPartialData` struct.

```rust,ignore
fn foo() -> Result<(), serde_json::Error> {
    use partial_idl_parser::*;

    // Get the JSON IDL data
    const IDL_RAW_DATA: &str = idl_path!("temp");

    // Parse the JSON IDL
    let parsed_idl = AnchorIdlPartialData::parse(IDL_RAW_DATA)?;

    // Get program ID
    parsed_idl.program_id();


    // Get An Instruction by it's identifier assuming the instruction is
    // labeled by the name `initialize`
    parsed_idl.get_instruction("initialize");


    // Get the instruction discriminant assuming the instruction is
    // labeled by the name `initialize`
    parsed_idl.get_discriminant("initialize");

    Ok(())
}
```

### LICENSE
MIT OR APACHE-2.0