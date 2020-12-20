defmodule AdventOfCode.Day18 do
  @doc "Find the opposite matching bracket, and return {bracket, after}"
  def match_bracket(string) do
    Enum.reduce_while(String.graphemes(string), {0, 0}, fn e, {s, level} ->
      case e do
        "(" ->
          {:cont, {s + 1, level + 1}}

        ")" when level == 1 ->
          {p1, p2} = String.split_at(string, s + 1)
          {:halt, {String.slice(p1, 1..(String.length(p1) - 2)), p2}}

        ")" ->
          {:cont, {s + 1, level - 1}}

        _ ->
          {:cont, {s + 1, level}}
      end
    end)
  end

  def p_str(<<?(, _rest::binary>> = str, tree) do
    {bracket, leftover} = match_bracket(str)
    p_str(leftover, [p_str(bracket, []) | tree])
  end

  def p_str(<<?+, rest::binary>>, tree) do
    p_str(rest, [:add | tree])
  end

  def p_str(<<?*, rest::binary>>, tree) do
    p_str(rest, [:mul | tree])
  end

  def p_str("", tree), do: Enum.reverse(tree)

  def p_str(s, tree) do
    [n, rest] = Regex.run(~r/(\d+)(.*)/, s, capture: :all_but_first)
    p_str(rest, [String.to_integer(n) | tree])
  end

  def p_1_reduce([a | rest]) do
    for [op, b] <- Enum.chunk_every(rest, 2), reduce: p_1_reduce(a) do
      acc ->
        case op do
          :add -> acc + p_1_reduce(b)
          :mul -> acc * p_1_reduce(b)
        end
    end
  end

  def p_1_reduce(x), do: x

  def do_adds(left, [a, :add, b | rest]) do
    do_adds(left, [p_2_reduce(a) + p_2_reduce(b) | rest])
  end

  def do_adds(left, [a, :mul | rest]) do
    do_adds(Enum.concat(left, [p_2_reduce(a), :mul]), rest)
  end

  def do_adds(left, [a]), do: Enum.concat(left, [p_2_reduce(a)])
  def do_adds(left, a), do: Enum.concat(left, [a])

  def p_2_reduce(xs) do
    Enum.take_every(do_adds([], xs), 2) |> Enum.reduce(1, &(&1 * &2))
  end

  def part1(args) do
    for expr <- String.replace(args, ~r/ /, "") |> String.split("\n", trim: true), reduce: 0 do
      a -> a + (p_str(expr, []) |> p_1_reduce())
    end
  end

  def part2(args) do
    for expr <- String.replace(args, ~r/ /, "") |> String.split("\n", trim: true), reduce: 0 do
      a -> a + (p_str(expr, []) |> p_2_reduce())
    end
  end
end
