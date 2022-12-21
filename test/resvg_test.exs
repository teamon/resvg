defmodule ResvgTest do
  use ExUnit.Case

  test "render/2" do
    svg = File.read!("test/examples/ubots.svg")
    png = File.read!("test/examples/ubots.png")

    assert Resvg.render(svg, :png) == png
  end
end
