defmodule AdventOfCode.Day11 do

  def neighbour_p1({x, y}, {dx, dy}, cells), do: Map.get(cells, {x + dx, y + dy})
  def neighbour_p2({x, y}, {dx, dy}, cells) do
    new_x = x + dx
    new_y = y + dy
    case Map.get(cells, {new_x, new_y}) do
      :noseat -> neighbour_p2({new_x, new_y}, {dx, dy}, cells)
      other -> other
    end
  end

  def count_neighbours(pos, cells, neighbour_func) do
    for dx <- [-1, 0, 1], dy <- [-1, 0, 1], {dx, dy} != {0, 0}, reduce: 0 do
      acc -> case neighbour_func.(pos, {dx, dy}, cells) do
               :full -> acc + 1
               _ -> acc
             end
    end    
  end

  def step(cells, neighbour_func, death_count) do
    for {k, v} <- cells, into: %{} do
      n = count_neighbours(k, cells, neighbour_func)
      {k, cond do
        v == :noseat -> :noseat
        n >= death_count -> :empty
        n == 0 -> :full
        true -> v
      end}
    end
  end

  def input_to_map(input) do
    for {line, i} <- String.trim(input) |> String.split |> Enum.with_index do
      for {c, j} <- String.graphemes(line) |> Enum.with_index do
        {{i, j}, (if c == "L", do: :empty, else: :noseat)}
      end
    end |> Enum.concat |> Map.new
  end

  def parse_and_solve(input, neighbour_func, death_count) do
    init = input_to_map(input)
    Stream.iterate(init, fn x -> step(x, neighbour_func, death_count) end)
    |> Stream.drop(1)
    |> Enum.reduce_while(init, fn e, a -> if e == a, do: {:halt, e}, else: {:cont, e} end)
    |> Map.values
    |> Enum.frequencies
    |> Map.get(:full)
  end
  
  def part1(args), do: parse_and_solve(args, &neighbour_p1/3, 4)
  def part2(args), do: parse_and_solve(args, &neighbour_p2/3, 5)
end
