defmodule AdventOfCode.Day08 do
  defmodule Computer do
    defstruct [input: %{}, acc: 0, pointer: 0, visited: MapSet.new(), exited: nil]

    def step(%{exited: exited} = state) when exited != nil, do: state
    def step(%{input: input, pointer: pointer} = state) when map_size(input) == pointer do
      %{state | exited: :graceful}
    end
    def step(%{input: input, pointer: pointer} = state) when map_size(input) < pointer do
      %{state | exited: :crashed}
    end
    def step(%{input: input, acc: acc, pointer: pointer, visited: visited} = state) do
      if pointer in visited do
        %{state | exited: :looping}
      else
        new_visited = MapSet.put(visited, pointer)
        case input[pointer] do
          [:nop, _] -> %{state | pointer: pointer + 1, visited: new_visited}
          [:acc, n] -> %{state | pointer: pointer + 1, acc: acc + n, visited: new_visited}
          [:jmp, n] -> %{state | pointer: pointer + n, visited: new_visited}
        end
      end
    end

    def run_until_exit(%{exited: exited} = state) when exited != nil, do: state
    def run_until_exit(state), do: run_until_exit(step(state))
  end

  def parse_input(input) do
    Regex.scan(~r/(acc|jmp|nop) ([+-]\d+)/, input, capture: :all_but_first)
    |> Enum.map(fn [instr, n] -> [String.to_atom(instr), String.to_integer(n)] end)
    |> Enum.with_index
    |> Enum.reduce(%{}, fn({v,k}, acc)-> Map.put(acc, k, v) end)
  end

  def part1(args) do
    %{acc: acc} = Computer.run_until_exit(%Computer{input: parse_input(args)})
    acc
  end

  def part2(args) do
    input = parse_input(args)
    %{acc: acc} = 0..map_size(input)
    |> Stream.filter(&(with [op, _] <- input[&1], do: op in [:nop, :jmp], else: (nil -> nil)))
    |> Stream.map(&(Map.update(input, &1, nil, fn [:nop, n] -> [:jmp, n]
                                                  [:jmp, n] -> [:nop, n] end)))
    |> Stream.map(&Computer.run_until_exit(%Computer{input: &1}))
    |> Stream.filter(fn %{exited: :graceful} -> true
                        _ -> false end)
    |> Stream.take(1)
    |> Enum.to_list()
    |> List.first()
    acc
  end
end
