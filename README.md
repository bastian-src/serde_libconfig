# Serde libconfig

## Under construction!

I'm new to Rust development and came to write a Serializer for [libconfig](https://github.com/hyperrealm/libconfig).
It's basically the official [serde json example](https://serde.rs/data-format.html) with some
changes to make it serialize into libconfig format. I'm planning on adding the Deserializer
too.

So, you're welcome to make PRs, leave comments, and just give me hints on how to do things better.

## Usage

```
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MySubStruct {
    sub_d: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct MyStruct {
    a: u16,
    b: String,
    c: MySubStruct,
}

fn main() {
    let my_struct = MyStruct {
        a: 123,
        b: "ajo".to_string(),
        c: MySubStruct { sub_d: 456},
    }

    let serialized = serde_libconfig::to_string(&my_struct).unwrap();
    println!("libconfig serialized: {}", serialized);
}
```
