defmodule AdventOfCode.Day24 do
  def get_coord("e" <> rest, {a, b}), do: get_coord(rest, {a, b + 1})
  def get_coord("se" <> rest, {a, b}), do: get_coord(rest, {a - 1, b + 1})
  def get_coord("sw" <> rest, {a, b}), do: get_coord(rest, {a - 1, b})
  def get_coord("w" <> rest, {a, b}), do: get_coord(rest, {a, b - 1})
  def get_coord("nw" <> rest, {a, b}), do: get_coord(rest, {a + 1, b - 1})
  def get_coord("ne" <> rest, {a, b}), do: get_coord(rest, {a + 1, b})
  def get_coord("", s), do: s

  def neighbours({a, b}),
    do: [{a, b + 1}, {a - 1, b + 1}, {a - 1, b}, {a, b - 1}, {a + 1, b - 1}, {a + 1, b}]

  def possible_coords(current) do
    Stream.flat_map(current, &[&1 | neighbours(&1)]) |> Stream.uniq()
  end

  def take_turn(black, n \\ 1) do
    cond do
      n <= 0 ->
        black

      :else ->
        for c <- possible_coords(black),
            num_surrounding =
              (for n <- neighbours(c), reduce: 0 do
                 a -> if n in black, do: a + 1, else: a
               end),
            num_surrounding == 2 or
              (c in black and (num_surrounding > 0 and num_surrounding < 3)),
            into: MapSet.new() do
          c
        end
        |> take_turn(n - 1)
    end
  end

  def str_to_state(string) do
    String.split(string, "\n", trim: true)
    |> Enum.map(&get_coord(&1, {0, 0}))
    |> Enum.frequencies()
    |> Enum.filter(fn {_, x} -> rem(x, 2) == 1 end)
    |> Enum.map(fn {x, _} -> x end)
    |> Enum.into(MapSet.new())
  end

  def part1(args) do
    str_to_state(args)
    |> Enum.count()
  end

  def part2(args) do
    str_to_state(args) |> take_turn(100) |> Enum.count()
  end
end
