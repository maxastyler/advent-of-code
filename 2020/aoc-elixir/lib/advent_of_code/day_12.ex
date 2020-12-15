defmodule AdventOfCode.Day12 do
  def mod(x, y) when x >= 0, do: rem(x, y)
  def mod(x, y) when x < 0 do
    case rem(x, y) do
      0 -> rem(x, y)
      z -> y + z
    end
  end

  def rot(p, 0), do: p
  def rot({x, y}, r) when r >= 0, do: rot({-y, x}, r - 90)
  def rot({x, y}, r) when r < 0, do: rot({y, -x}, r + 90)
 
  def parse_instruction_1({r, x, y}, "N" <> num), do: {r, x, y + String.to_integer(num)}
  def parse_instruction_1({r, x, y}, "S" <> num), do: {r, x, y - String.to_integer(num)}
  def parse_instruction_1({r, x, y}, "E" <> num), do: {r, x + String.to_integer(num), y}
  def parse_instruction_1({r, x, y}, "W" <> num), do: {r, x - String.to_integer(num), y}
  def parse_instruction_1({r, x, y}, "L" <> num), do: {r + String.to_integer(num), x, y}
  def parse_instruction_1({r, x, y}, "R" <> num), do: {r - String.to_integer(num), x, y}
  def parse_instruction_1({r, _, _} = s, "F" <> num) do
    case mod(r, 360) do
      0 -> parse_instruction_1(s, "E" <> num)
      90 -> parse_instruction_1(s, "N" <> num)
      180 -> parse_instruction_1(s, "W" <> num)
      270 -> parse_instruction_1(s, "S" <> num)
    end
  end

  def parse_instruction_2({{x, y}, p}, "N" <> num), do: {{x, y + String.to_integer(num)}, p}
  def parse_instruction_2({{x, y}, p}, "S" <> num), do: {{x, y - String.to_integer(num)}, p}
  def parse_instruction_2({{x, y}, p}, "E" <> num), do: {{x + String.to_integer(num), y}, p}
  def parse_instruction_2({{x, y}, p}, "W" <> num), do: {{x - String.to_integer(num), y}, p}
  def parse_instruction_2({{x, y}, p}, "L" <> num), do: {rot({x, y}, String.to_integer(num)), p}
  def parse_instruction_2({{x, y}, p}, "R" <> num), do: {rot({x, y}, -String.to_integer(num)), p}
  def parse_instruction_2({{x, y}, {px, py}}, "F" <> num) do
    d = String.to_integer(num)
    {{x, y}, {px + d * x, py + d * y}}
  end

  def part1(args) do
    {_, x, y} = for line <- String.trim(args) |> String.split, reduce: {0, 0, 0} do
      acc -> parse_instruction_1(acc, line)
    end
    abs(x) + abs(y)
  end

  def part2(args) do
    {_, {x, y}} = for line <- String.trim(args) |> String.split, reduce: {{10, 1}, {0, 0}} do
      acc -> parse_instruction_2(acc, line)
    end
    abs(x) + abs(y)
  end
end
