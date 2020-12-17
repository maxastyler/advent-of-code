defmodule AdventOfCode.Day17 do
  def parse_input(input, dim) do
    for {l, i} <- String.split(input) |> Enum.with_index() do
      for {c, j} <- String.graphemes(l) |> Enum.with_index(), c == "#" do
        {[i, j | List.duplicate(0, dim - 2)], 1}
      end
    end
    |> Enum.concat()
    |> Map.new()
  end

  def get_coord_bounds(coordinates) do
    coord_zip = Enum.zip(coordinates)
    mins = Enum.map(coord_zip, &((Tuple.to_list(&1) |> Enum.min()) - 1))
    maxs = Enum.map(coord_zip, &((Tuple.to_list(&1) |> Enum.max()) + 1))
    Enum.zip(mins, maxs) |> Enum.map(fn {x, y} -> x..y end)
  end

  def range_product([x | []]), do: Enum.map(x, &[&1])

  def range_product([x | xs]) do
    Enum.flat_map(x, fn i -> Enum.map(range_product(xs), &[i | &1]) end)
  end

  def neighbour_iter(dim) do
    List.duplicate(-1..1, dim)
    |> range_product()
    |> Enum.reject(fn x -> Enum.all?(x, &(&1 == 0)) end)
  end

  def next_cell(coord, cells, neighbours) do
    val = Map.get(cells, coord, 0)

    case (for n <- neighbours, reduce: 0 do
            a ->
              a + Map.get(cells, Enum.zip(coord, n) |> Enum.map(fn {x, y} -> x + y end), 0)
          end) do
      3 -> 1
      2 when val == 1 -> 1
      _ -> 0
    end
  end

  def step(cells) do
    neighbours = Map.keys(cells) |> List.first() |> Enum.count() |> neighbour_iter()

    for c <- range_product(get_coord_bounds(Map.keys(cells))),
        next_cell(c, cells, neighbours) == 1,
        into: %{} do
      {c, 1}
    end
  end

  def part1(args) do
    cells = parse_input(args, 3)
    Stream.iterate(cells, &step/1) |> Enum.at(6) |> Enum.count()
  end

  def part2(args) do
    cells = parse_input(args, 4)
    Stream.iterate(cells, &step/1) |> Enum.at(6) |> Enum.count()
  end
end
