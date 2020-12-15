defmodule AdventOfCode.Day09 do
  def parse_input(input) do
    String.trim(input)
    |> String.split("\n")
    |> Enum.map(&String.to_integer/1)
  end

  def find_bad_encoding(nums) do
    Stream.chunk_every(nums, 26, 1)
    |> Stream.reject(
      &(List.last(&1) in for(
          {x, i} <- Enum.with_index(Enum.take(&1, 25)),
          y <- Enum.drop(&1, i) |> Enum.take(25),
          do: x + y
        ))
    )
    |> Enum.take(1)
    |> List.first()
    |> List.last()
  end

  def part1(args) do
    parse_input(args)
    |> find_bad_encoding
  end

  def part2(args) do
    input = parse_input(args)
    bad_num = find_bad_encoding(input)

    [bad_encoding | _] =
      Stream.map(2..Enum.count(input), &Stream.chunk_every(input, &1, 1))
      |> Stream.concat()
      |> Stream.filter(&(Enum.sum(&1) == bad_num))
      |> Enum.take(1)

    Enum.min(bad_encoding) + Enum.max(bad_encoding)
  end
end
