defmodule Resvg do
  use Rustler, otp_app: :resvg, crate: "resvg_nif"

  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end
