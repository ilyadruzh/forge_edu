case File.read("doroga.txt") do
  {:ok, config} -> IO.puts(config)
  {:error, :enoent} -> IO.puts("Error")
end
