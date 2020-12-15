defmodule AdventOfCode.Day14 do
  use Bitwise

  def zero({zeros, ones}), do: {zeros <<< 1, ones <<< 1}
  def one({zeros, ones}), do: {(zeros <<< 1) ||| 1, (ones <<< 1) ||| 1}
  def no_change({zeros, ones}), do: {(zeros <<< 1) ||| 1, ones <<< 1}

  def parse_mask_1(mask) do
    for x <- String.graphemes(mask), reduce: {~~~0, 0} do
      acc -> case x do
               "X" -> no_change(acc)
               "0" -> zero(acc)
               "1" -> one(acc)
             end
    end
  end

  def parse_mask_2(mask) do
    for x <- String.graphemes(mask), reduce: [{~~~0, 0}] do
      floats -> case x do
                  "X" -> Enum.map(floats, &[zero(&1), one(&1)]) |> Enum.concat
                  "0" -> Enum.map(floats, &no_change/1)
                  "1" -> Enum.map(floats, &one/1)
                end
    end
  end

  def input_to_instructions(input, mask_parse_func) do
    for l <- String.trim(input) |> String.split("\n") do
      case l do
        "mask = " <> rest -> {:mask, mask_parse_func.(rest)}
        rest -> with [[_, loc], [_, val]] <- Regex.scan(~r/(\d+)/, rest) do
                  {:set, {String.to_integer(loc), String.to_integer(val)}}
                end
      end
    end
  end

  def part1(args) do
    instructions = input_to_instructions(args, &parse_mask_1/1)
    {_, _, memory} = for {inst, {x1, x2}} <- instructions, reduce: {0, 0, %{}} do
      {zeros, ones, memory} -> case inst do
                                 :mask -> {x1, x2, memory}
                                 :set -> {zeros, ones, Map.put(memory, x1, (x2 &&& zeros) ||| ones)}
                               end
    end
    Map.values(memory) |> Enum.sum
  end

  def part2(args) do
    instructions = input_to_instructions(args, &parse_mask_2/1)
    {_, memory} = for {inst, v} <- instructions, reduce: {[], %{}} do
      {masks, memory} -> case inst do
                           :mask -> {v, memory}
                           :set -> with {loc, val} <- v do
                                     {masks, Enum.reduce(masks, memory,
                                         fn {z, o}, m -> Map.put(m, (loc &&& z) ||| o, val) end)}
                                   end
                         end
    end
    Map.values(memory) |> Enum.sum
  end
end
