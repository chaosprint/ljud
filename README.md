# ljud (wip)

`ljud`: Swedish word for `audio`

experimental ideas with [lyd](https://github.com/chaosprint/lyd)

The main difference is that `ljud` uses `Box<dyn Trait>` while `lyd` uses only `Enum`.

Both can run on Daisy Seed rev5 while `lyd` has better performance on Embedded Rust so far. 