defmodule AdventOfCode.Day06 do
  def set_from_input(input) do
    String.split(input, ~r/\n\n/)
    |> Enum.map(fn x -> String.split(x)
                        |> Enum.map(&(String.to_charlist(&1)
                                      |> MapSet.new)) end)
  end
    
  def part1(args) do
    set_from_input(args)
    |> Enum.map(fn x -> Enum.reduce(x, &MapSet.union(&1, &2)) |> Enum.count end)
    |> Enum.sum
  end

  def part2(args) do
    set_from_input(args)
    |> Enum.map(fn x -> Enum.reduce(x, &MapSet.intersection(&1, &2)) |> Enum.count end)
    |> Enum.sum
  end
end
