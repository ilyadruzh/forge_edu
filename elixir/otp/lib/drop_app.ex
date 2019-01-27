defmodule DropApp do
  @moduledoc false
use Application

  def start(_type, _args) do
    IO.puts("Starting the app.. ")
    DropSup.Supervisor.start_link()
  end
end
