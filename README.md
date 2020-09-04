# toml-rs-bind
Rust toml crate bindings for Deno

# Example
```typescript
import * as toml from 'https://cdn.jsdelivr.net/gh/juzi5201314/toml-rs-bind/mod.ts'

toml.stringify({
    a: 1,
    b: { c: ["2", "3"] }
})

toml.parse(`
a = 1
[b]
c = ["2", "3"]
`)
```