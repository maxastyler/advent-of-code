defmodule AdventOfCode.Day20 do
  @doc "check if two edges are equal, with a flip"
  @spec check_flip(String.t(), String.t()) :: {boolean(), :noflip | :flip | nil}
  def check_flip(e1, e2) do
    cond do
      e1 == e2 -> {true, :noflip}
      String.reverse(e1) == e2 -> {true, :flip}
      true -> {false, nil}
    end
  end

  def parse_tile(tile) do
    [num, tile] = Regex.run(~r/Tile (\d+):\n(.*)/s, tile, capture: :all_but_first)
    split = String.split(tile, "\n", trim: true)
    {String.to_integer(num), tile_edges(split)}
  end

  def tile_edges(tile) do
    [
      List.first(tile),
      Enum.map(tile, &String.last/1) |> Enum.join(),
      List.last(tile),
      Enum.map(tile, &String.first/1) |> Enum.join()
    ]
  end

  def parse_tile_2(tile) do
    [num, tile] = Regex.run(~r/Tile (\d+):\n(.*)/s, tile, capture: :all_but_first)
    split = String.split(tile, "\n", trim: true)
    {String.to_integer(num), split}
  end

  @doc "rotates and flips the tile horizontally"
  def rotate_tile(tile) do
    Enum.zip(Enum.map(tile, &String.graphemes/1))
    |> Enum.map(&(Tuple.to_list(&1) |> List.to_string() |> String.reverse()))
  end

  @doc "flips the tile horizontally"
  def flip_tile(tile) do
    Enum.map(tile, &String.reverse/1)
  end

  def transformations(tile) do
    Stream.iterate(tile, &rotate_tile/1)
    |> Stream.take(4)
    |> Stream.flat_map(&[&1, flip_tile(&1)])
  end

  def complete_row([{_, c} | _] = row, tiles) do
    c_edge = Enum.map(c, &String.first/1) |> Enum.join()

    case find_matching_tile(
           c_edge,
           fn x -> Enum.map(x, &String.last/1) |> Enum.join() end,
           tiles
         ) do
      nil -> row
      {id, _} = new_tile -> complete_row([new_tile | row], List.keydelete(tiles, id, 0))
    end
  end

  def complete_column([{_, c} | _] = col, tiles) do
    c_edge = List.first(c)

    case find_matching_tile(c_edge, &List.last/1, tiles) do
      nil -> col
      {id, _} = new_tile -> complete_column([new_tile | col], List.keydelete(tiles, id, 0))
    end
  end

  def find_matching_tile(_, _, []), do: nil

  def find_matching_tile(edge, edge_fun, [{id, tile} | tiles]) do
    case Enum.reduce_while(transformations(tile), nil, fn t, _ ->
           case edge_fun.(t) == edge do
             true -> {:halt, t}
             _ -> {:cont, nil}
           end
         end) do
      nil -> find_matching_tile(edge, edge_fun, tiles)
      t -> {id, t}
    end
  end

  def part1(args) do
    tiles = String.split(args, "\n\n", trim: true) |> Enum.map(&parse_tile/1)
    edges = Enum.flat_map(tiles, &elem(&1, 1))

    for {id, es} <- tiles,
        (for e <- es,
             (for e2 <- edges, reduce: 0 do
                c -> c + with {true, _} <- check_flip(e, e2), do: 1, else: (_ -> 0)
              end) == 2 do
           1
         end)
        |> Enum.count() == 2,
        reduce: 1 do
      acc -> acc * id
    end
  end

  def find_bottom_right_tile(nil, _), do: nil

  def find_bottom_right_tile([{id, t} | tiles], edges) do
    case (for e <- tile_edges(t) do
            for e2 <- edges, reduce: 0 do
              c -> c + with {true, _} <- check_flip(e, e2), do: 1, else: (_ -> 0)
            end
          end) do
      [1, 1, 2, 2] -> {id, rotate_tile(t)}
      [2, 1, 1, 2] -> {id, t}
      [2, 2, 1, 1] -> {id, t |> rotate_tile() |> rotate_tile() |> rotate_tile()}
      [1, 2, 2, 1] -> {id, t |> rotate_tile() |> rotate_tile()}
      _ -> find_bottom_right_tile(tiles, edges)
    end
  end

  def trim_edges({_, tile}) do
    Enum.slice(tile, 1..-2) |> Enum.map(&String.slice(&1, 1..-2))
  end

  def connect_rows(tile_row) do
    Enum.map(tile_row, &trim_edges/1)
    |> Enum.zip()
    |> Enum.map(&(Tuple.to_list(&1) |> Enum.join()))
  end

  def monster_coords,
    do: [
      {0, 18},
      {1, 0},
      {1, 5},
      {1, 6},
      {1, 11},
      {1, 12},
      {1, 17},
      {1, 18},
      {1, 19},
      {2, 1},
      {2, 4},
      {2, 7},
      {2, 10},
      {2, 13},
      {2, 16}
    ]

  def get_char(puzzle, {x, y}), do: Enum.at(puzzle, x, "") |> String.at(y) == "#"

  def monster_at(puzzle, {x, y}) do
    Enum.all?(monster_coords(), fn {a, b} -> get_char(puzzle, {x + a, y + b}) end)
  end

  def part2(args) do
    tiles = String.split(args, "\n\n", trim: true) |> Enum.map(&parse_tile_2/1)
    starting_tile = find_bottom_right_tile(tiles, Enum.flat_map(tiles, &tile_edges(elem(&1, 1))))
    column = complete_column([starting_tile], List.keydelete(tiles, elem(starting_tile, 0), 0))

    completed =
      Enum.map(column, &complete_row([&1], List.keydelete(tiles, elem(&1, 0), 0)))
      |> Enum.flat_map(&connect_rows/1)
      |> flip_tile()

    for i <- 0..length(completed),
        j <- 0..(List.first(completed) |> String.length()),
        monster_at(completed, {i, j}) do
      {i, j}
    end
    |> Enum.flat_map(fn {i, j} ->
      monster_coords() |> Enum.map(fn {x, y} -> {x + i, y + j} end)
    end)
    |> Enum.reduce(completed, fn {i, j}, s ->
      List.update_at(s, i, fn x ->
        String.graphemes(x) |> List.replace_at(j, " ") |> List.to_string()
      end)
    end)
    |> Enum.flat_map(&String.graphemes/1)
    |> Enum.filter(&(&1 == "#"))
    |> Enum.count()
  end
end
