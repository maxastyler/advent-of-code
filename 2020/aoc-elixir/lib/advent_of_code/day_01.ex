defmodule AdventOfCode.Day01 do

  def format_input(inp) do
    inp
    |> String.trim()
    |> String.split()
    |> Enum.map(&(String.to_integer(&1)))
  end

  def part1(args) do
    inp = format_input(args)
    for x <- inp, y <- inp, x + y == 2020 do
                                x * y
    end |> Enum.take(1)
  end

  def part2(args) do
    inp = format_input(args)
    for x <- inp, y <- inp, z <- inp, x + y + z == 2020 do
                                              x * y * z
    end |> Enum.take(1)
  end
end
