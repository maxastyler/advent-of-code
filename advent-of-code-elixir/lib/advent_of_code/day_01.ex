defmodule AdventOfCode.Day01 do
  def part1(args) do
    nums = String.graphemes(args) |> Enum.map(&String.to_integer(&1))
    nums = nums ++ Enum.slice(nums, 0..1)
    for {a, b} <- Enum.zip(nums, Enum.slice(nums, 1..-1)), a==b, reduce: 0 do
      acc -> acc + a
    end
  end

  def part2(args) do
    nums = String.graphemes(args) |> Enum.map(&String.to_integer(&1))
    skip = div(String.length(args), 2)
    for {a, b} <- Enum.zip(nums,
          Stream.cycle(nums)
          |> Stream.drop(skip)
          |> Stream.take(Enum.count(nums))), a==b, reduce: 0 do
      acc -> acc + a
    end
  end
end
