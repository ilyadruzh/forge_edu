defmodule FirstApp do
  @moduledoc """
  Documentation for FirstApp.
  """

  @doc """
  Hello world.

  ## Examples

      iex> FirstApp.hello()
      :world

  """
  def hello do
    :world
  end

  def gen do
    with {public_key, private_key} <- :crypto.generate_key(:ecdh, :secp256k1)
    |> Base.encode16(public_key)
    end

  def keys do
    case gen do
       {public_key}-> IO.puts(public_key)

       end
       end
end