
![Tileline](/logo.png "Tileline")

Tileline is a simple library to generate SVG block graph that represents a set of events, it has API for generic
block graphs, and a year block graph, with the ability to associate links to specific blocks.


## Status 

### Supported Features

- Simple Block Graph
- Simple Block Graph with labels
- Year Block Graph (year_line feature flag)

### Potential Features

- API with pre-defined colors by value instead of RGB value
- Multiple preconfigured set of colors 


## Use

Just add it as dependency in your project toml.

```toml
tileline = "0.1.0"
```

With additional year block graph

```toml
tileline = {version="0.1.0", features=["year_line"]} 
```

## Development

Being a rust project just make sure to have your rust compiler setup and then:

Base build:
```sh
cargo build --all-features
```

Run Tests:
```sh
cargo test --all-features
```


## License

Unless explicitly stated otherwise, all work is subject to the terms of the MIT/Apache-2.0.
