defmodule AdventOfCode.Day13 do

  def parse_input(input) do
    [t, ids] = String.trim(input) |> String.split("\n")
    %{time: String.to_integer(t),
      buses: String.split(ids, ",")
      |> Enum.with_index
      |> Enum.reject(fn {id, _} -> id == "x" end)
      |> Enum.map(fn {id, t} -> {String.to_integer(id), mod(t, String.to_integer(id))} end)}
  end
  
  def part1(args) do
    %{time: t, buses: b} = parse_input(args)
    {wait_t, {id, _}} = Stream.iterate(t, &(&1 + 1))
    |> Stream.map(&({&1, Enum.find(b, fn {id, _} -> rem(&1, id) == 0 end)}))
    |> Stream.reject(fn {_, res} -> is_nil(res) end)
    |> Enum.take(1)
    |> List.first
    (wait_t - t) * id
  end

  def chinese_remainder_theorem(x, this_n, [{n, a} | ns]) do
    Stream.iterate(x, &(&1 + this_n))
    |> Stream.filter(&(rem(&1, n) == a))
    |> Enum.take(1)
    |> List.first
    |> (fn new_x, new_n, new_ns -> chinese_remainder_theorem(new_x, new_n, new_ns) end).(n * this_n, ns)
  end
  def chinese_remainder_theorem(x, _, _), do: x

  def mod(a, b) when a >= 0, do: rem(a, b)
  def mod(a, b) when a < 0, do: rem(b + a, b)

  def part2(args) do
    %{time: t, buses: b} = parse_input(args)
    b = Enum.sort_by(b, fn {id, _} -> id end, &(&1 >= &2))
    |> Enum.map(fn {id, a} -> {id, rem(a, id)} end)
    chinese_remainder_theorem(0, 1, b)
    # chinese_remainder_theorem(0, 1, [{17, 0}, {13, 11}, {19, 16}])
  end
end
