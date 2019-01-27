defmodule DropSup.Supervisor do
  @moduledoc false



  use Supervisor

  def start_link do
    Supervisor.start_link(__MODULE__, [], [{:name, __MODULE__}])
  end

  def init([]) do
    children = [worker(DropServer, [], [])]

    supervise(children, [{:strategy, :one_for_one}, {:max_restarts, 1}, {:max_seconds, 5}])
  end
end