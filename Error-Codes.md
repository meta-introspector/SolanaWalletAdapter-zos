### List of error codes and their explanation

- `WE01` - The `accounts` key was not found in the JavaScript object returned from invoking the event `change` dispathced from invoking `on` method from `[standard:events]` namespace.
- `WE02` - The JavaScript object returned from invoking the event `change` dispathced from invoking `on` method from `[standard:events]` namespace is either undefined or null
- `WE03` - The Uint8Array returned from the result of the JavaScript object returned from invoking the event `change` dispathced from invoking `on` method from `[standard:events]` namespace is not 32 bytes in length therefore it is automatically and invalid Ed25519 public key
- `WE04` - The Uint8Array returned from the result of the JavaScript object returned from invoking the event `change` dispathced from invoking `on` method from `[standard:events]` namespace returned 32 bytes that are invalid for converting them to an Ed25519 public key
- `WE05` - unable to convert the callback function for the `on` method from `[standard:events]` namespace to a JavaScript function