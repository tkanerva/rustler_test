defmodule RustlerTest do
  @moduledoc """
  Documentation for `RustlerTest`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> RustlerTest.hello()
      :world

  """
  def hello do
    RustlerTest.Native.testii(42) |> IO.inspect
    RustlerTest.Native.mandel_rgb(100, 100, 10, 20, 1.0)
    
  end


  
end
