defmodule RustlerTest.Native do
  use Rustler, otp_app: :rustler_test

  def mandel_rgb(_w, _h, _x, _y, _z), do: :erlang.nif_error(:nif_not_loaded)

  def testii(x) do
    x * 2
  end
  
end
