defmodule AdventOfCode.Day04 do

  defp valid_password(words) do
    processed = String.split(words)
    (MapSet.new(processed) |> MapSet.size()) == Enum.count(processed)
  end

  defp valid_password_part2(words) do
    processed = String.split(words) |> Enum.map(fn (x) -> String.to_charlist(x) |> Enum.sort() end)
    (MapSet.new(processed) |> MapSet.size()) == Enum.count(processed)
  end
  
  def part1(args) do
    String.trim(args)
    |> String.split("\n")
    |> Enum.filter(&valid_password/1)
    |> Enum.count()
  end

  def part2(args) do
    String.trim(args)
    |> String.split("\n")
    |> Enum.filter(&valid_password_part2/1)
    |> Enum.count()
  end
end
