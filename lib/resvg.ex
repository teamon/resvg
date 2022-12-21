defmodule Resvg do
  use Rustler, otp_app: :resvg, crate: "resvg_nif"

  def render(svg, :png) do
    render_svg_to_png(svg)
  end

  @spec render_svg_to_png(binary) :: binary
  def render_svg_to_png(_binary), do: :erlang.nif_error(:nif_not_loaded)
end
