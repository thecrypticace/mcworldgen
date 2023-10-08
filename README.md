# MC World Gen

This is a world-gen analyzer for Minecraft dimensions. The only thing you can do right now (without editing the source) is a wildcard search for Veins of certain ore types.

This tool is written in Rust which means you'll need Rust installed to run it.
I recommend you use [rustup](https://rustup.rs) to install Rust on your system.

1. Clone this repo
2. Run the following command:

```shell
cargo run --release {path_to_dimension} {blocks_to_find} {origin} [threshold]
```

| arg                 | format    | description                                                                                                                                                            |
| ------------------- | --------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `path_to_dimension` | file path | The path on disk to the dimension. If you're looking at the overworld then it's just the path to the world folder.                                                     |
| `blocks_to_find`    | string    | The ore to look for. Supports wildcards using `*` which matches any number of characters                                                                               |
| `origin`            | `x,y,z`   | Tthe `x,y,z` coordinates used for distance calculations.                                                                                                               |
| `threshold`         | `x,y,z`   | Expand the origin point into a region to search e.g. `10,10,10` creates a search area 10 blocks in every direction from the origin resulting in a 21x21x21 search area |
