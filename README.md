# Resvg

> ⚠️ **WARNING**: This is an experimental proof of concept. Read the code, use
> at your own risk ⚠️

Elixir library to render SVG into PNG using
[resvg](https://github.com/RazrFalcon/resvg).

## Usage

```ex
# Get some SVG (read from file, generate in memory, etc.)
svg = File.read!("example.svg")

# Render into PNG
png = Resvg.render(svg, :png)

# Write to file (or serve via phoenix/plug)
File.write!("example.png", png)
```

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `resvg` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:resvg, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with
[ExDoc](https://github.com/elixir-lang/ex_doc) and published on
[HexDocs](https://hexdocs.pm). Once published, the docs can be found at
<https://hexdocs.pm/resvg>.
