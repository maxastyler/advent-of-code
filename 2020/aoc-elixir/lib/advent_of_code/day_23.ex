defmodule AdventOfCode.Day23 do
  defmodule GameState do
    @type cups :: %{integer() => integer()}
    @type t :: %__MODULE__{current: integer(), cups: cups(), max: integer(), min: integer()}
    defstruct [:current, :cups, :max, :min]

    @spec from_str(String.t(), nil | integer()) :: t()
    def from_str(str, up_to \\ nil) do
      first = String.graphemes(str) |> Enum.map(&String.to_integer/1)

      ints =
        [h | t] =
        case up_to do
          nil -> first
          _ -> Enum.concat(first, (Enum.max(first) + 1)..up_to)
        end

      %GameState{
        current: h,
        cups: Enum.zip(ints, t ++ [h]) |> Map.new(),
        max: Enum.max(ints),
        min: Enum.min(ints)
      }
    end

    defp next_id_inner(id, taken, smallest, largest) do
      next? = id - 1

      cond do
        next? < smallest -> next_id_inner(largest + 1, taken, smallest, largest)
        next? in taken -> next_id_inner(next?, taken, smallest, largest)
        :else -> next?
      end
    end

    @doc "get the next id"
    def next_id(%GameState{current: current, max: max, min: min}, taken) do
      next_id_inner(current, taken, min, max)
    end

    @doc "get the next n from the given id and cups"
    @spec get_next(integer(), cups(), integer()) :: [integer()]
    def get_next(_id, _cups, 0), do: []

    def get_next(id, cups, n) do
      [id | get_next(cups[id], cups, n - 1)]
    end

    def take_turn(%GameState{current: current, cups: cups} = state, n \\ 1) do
      cond do
        n <= 0 ->
          state

        :else ->
          taken = [a, _, c] = get_next(cups[current], cups, 3)
          current_new_neighbour = cups[c]
          dest = next_id(state, taken)
          c_new_neighbour = cups[dest]

          take_turn(
            %GameState{
              state
              | cups:
                  Map.put(cups, current, current_new_neighbour)
                  |> Map.put(dest, a)
                  |> Map.put(c, c_new_neighbour),
                current: current_new_neighbour
            },
            n - 1
          )
      end
    end

    def cups_list({x, m}) when m == %{}, do: []

    def cups_list({x, m}) do
      [x | cups_list(Map.pop(m, x))]
    end

    def cups_list(%GameState{current: current, cups: cups}) do
      cups_list({current, cups})
    end
  end

  def part1(args) do
    cups =
      args
      |> GameState.from_str()
      |> GameState.take_turn(100)

    Map.put(cups, :current, 1) |> GameState.cups_list() |> Enum.filter(&(&1 != 1)) |> Enum.join()
  end

  def part2(args) do
    cups =
      args
      |> GameState.from_str(1_000_000)
      |> GameState.take_turn(10_000_000)

    [_, a, b] = GameState.get_next(1, cups.cups, 3)
    a * b
  end
end
