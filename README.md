# AnyBox

(rust) Library providing a plain and simple datatype that can hold data of arbitrary type.
In order to receive data providing the type again is required.

Designed to do one thing and do that well.

## Usage

```rust
let data = AnyBox::new(7usize);

let retrieved: &usize = data.get();
```

## Is this safe to use?

This is very experimental, it is not safe to use.
