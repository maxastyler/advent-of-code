defmodule AdventOfCode.Day05 do
  def string_to_bin(s) do
    String.replace(s, ["F", "L"], "0")
    |> String.replace(["B", "R"], "1")
    |> String.to_integer(2)
  end

  def part1(args) do
    args |> String.split() |> Enum.map(&string_to_bin/1) |> Enum.sort(&>=/2) |> List.first
  end

  def part2(args) do
    ids = args |> String.split() |> Enum.map(&string_to_bin/1) |> Enum.sort
    Enum.reduce_while(ids, List.first(ids), &(if &1 - &2 > 1, do: {:halt, &2 + 1}, else: {:cont, &1}))
  end
end
