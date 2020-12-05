defmodule AdventOfCode.Day05 do
  def string_to_bin(s) do
    Regex.replace(~r/(F|B|L|R)/, s,
      fn (_, "F") -> "0"
         (_, "B") -> "1"
         (_, "L") -> "0"
         (_, "R") -> "1" end)
    |> String.to_integer(2)
  end

  def part1(args) do
    args |> String.split() |> Enum.map(&string_to_bin/1) |> Enum.sort(&>=/2) |> List.first
  end

  def seat_diff([h | t], [h2 | t2]), do: if (h - h2) == 2, do: h2 + 1, else: seat_diff(t, t2)

  def part2(args) do
    ids = args |> String.split() |> Enum.map(&string_to_bin/1) |> Enum.sort
    seat_diff(Enum.drop(ids, 1), Enum.take(ids, Enum.count(ids) - 1))
  end
end
