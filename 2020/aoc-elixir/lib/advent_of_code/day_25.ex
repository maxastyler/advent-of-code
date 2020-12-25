defmodule AdventOfCode.Day25 do
  def transform(subject, loop_size, mod \\ 20_201_227, value \\ 1) do
    cond do
      loop_size <= 0 -> value
      :else -> transform(subject, loop_size - 1, mod, rem(value * subject, mod))
    end
  end

  def crack_key({cur_a, cur_b}, {a, b}, loop_size \\ 0) do
    cond do
      cur_a == a ->
        {:a, loop_size}

      cur_b == b ->
        {:b, loop_size}

      :else ->
        crack_key({rem(cur_a * 7, 20_201_227), rem(cur_b * 7, 20_201_227)}, {a, b}, loop_size + 1)
    end
  end

  def parse_input(input) do
    String.split(input, "\n", trim: true) |> Enum.map(&String.to_integer/1)
  end

  def part1(args) do
    [a, b] = parse_input(args)

    case crack_key({1, 1}, {a, b}) do
      {:a, loop} -> transform(b, loop)
      {:b, loop} -> transform(a, loop)
    end
  end

  def part2(args) do
  end
end
