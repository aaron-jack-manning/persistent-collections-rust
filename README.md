# Persistent Collections Rust

A couple of immutable and persistent collections in Rust. The implementations here are those that would naturally arise in languages such as OCaml, F# and Haskell.

All data structures within this repository use reference counting to be persistent through changes, which means that the operations do not mutate the instance they are callled on, but rather return new collections which share data with the original. For example, in the following code:

```
let list = List::singleton(4);

let second_list = list.prepend(9);

println!("{:?}", list); [4]
println!("{:?}", second_list); // [9, 4]
```

`list` contains `4` only, but `second_list` contains `9` and `4`. The node containing `4` is shared but that doesn't matter given the immutability of each node. This is important because on its own the line `list.insert(9)` is not useful unless the result is stored and does not mutated `list`.

