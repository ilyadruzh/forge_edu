defmodule Bitcointest do
  @moduledoc """
  Documentation for Bitcointest.
  """

  @doc """
  Hello world.

  ## Examples

      iex> Bitcointest.hello()
      :world

  """
  def hello do
    :world
  end

  def generate_bitcoin_keypair do
    {public_key, private_key} = :crypto.generate_key(:ecdh, :secp256k1)
    IO.puts(Base.encode16(public_key)) # {Base.encode16(public_key), Base.encode16(private_key)}
    IO.puts(Base.encode16(private_key)) # {Base.encode16(public_key), Base.encode16(private_key)}
  end
end
