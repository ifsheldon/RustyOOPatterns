# Rusty Design Pattern Implementation Samples

## Note

Though these codes are simple, I think it's a good way to get familiar with Rust, especially some tricks and features only belonged to Rust, like **lifetime** with references. 

## Reference

Most but not all of the examples are from the book *Head First Design Pattern* which is reallllly juicy and worth reading.

Some other sources are referred in code comments.

## Documentation

Please run `cargo doc --open` in the root of this directory to see the auto-generated fancy documentation.

## Recommended Environment

`Jetbrains' IDEs` + `Rust plugin by Jetbrains`

For you can see the static type inference of Rust, which is really helpful

## TODO

Implement more patterns

### Tough Ones

* Singleton
  * The crate `lazy-static` helps in `main.rs` and it works, but it seems that this dose not work in other mods.
  * Singleton is discouraged by the community for its global accessibility