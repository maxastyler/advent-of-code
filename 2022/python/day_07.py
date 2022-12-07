import re
from functools import reduce
from typing import Self, Callable

from attr import frozen, field, evolve
from pyrsistent import v, pvector
from pyrsistent.typing import PVector

ls_reg = re.compile("(\d+|dir) ([^\n]*)")


@frozen
class File:
    name: str = field()
    size: int = field()


@frozen
class Directory:
    name: str = field()
    children: PVector[Self | File] = field()

    def find_child_index(self, filter: Callable[[Self | File], bool]) -> int | None:
        for i, child in enumerate(self.children):
            if filter(child):
                return i
        return None

    def size(self) -> int:
        return sum(
            child.size if isinstance(child, File) else child.size()
            for child in self.children
        )

    def size_filtered(self, max: int) -> int:
        self_size = self.size()
        return sum(
            child.size_filtered(max)
            for child in self.children
            if isinstance(child, Directory)
        ) + (self_size if self_size <= max else 0)


@frozen
class DirZipper:
    parent: Self | None = field()
    left: PVector[Directory | File] = field()
    right: PVector[Directory | File] = field()
    name: str = field()
    focus: Directory | File = field()

    def up(self) -> Self | None:
        if self.parent:
            return evolve(
                self.parent,
                focus=Directory(
                    name=self.name,
                    children=self.left.append(self.focus).extend(self.right),
                ),
            )
        else:
            return None

    def down(self, position: int) -> Self | None:
        if isinstance(self.focus, File) or position >= len(self.focus.children):
            return None
        else:
            left = self.focus.children[:position]
            right = self.focus.children[position + 1 :]
            return DirZipper(
                parent=self,
                left=left,
                right=right,
                name=self.focus.name,
                focus=self.focus.children[position],
            )

    def top(self) -> Self:
        new_zipper = self
        while (up := new_zipper.up()) is not None:
            new_zipper = up
        return new_zipper

    def apply(self, func: Callable[[Directory | File], Directory | File]) -> Self:
        return evolve(self, focus=func(self.focus))


def parse_ls_result(ls_result: tuple[str, str]) -> Directory | File:
    if ls_result[0] == "dir":
        return Directory(ls_result[1], pvector())
    else:
        return File(ls_result[1], int(ls_result[0]))


def apply_line(zipper: DirZipper, line: tuple[str, str]) -> DirZipper | None:
    match line:
        case ("cd", "/"):
            return zipper.top()
        case ("cd", ".."):
            return zipper.up()
        case ("cd", folder):
            i = zipper.focus.find_child_index(lambda x: x.name == folder)
            return zipper.down(i)
        case ("ls", results):
            return zipper.apply(
                lambda dir: evolve(
                    dir,
                    children=pvector(
                        [parse_ls_result(r) for r in ls_reg.findall(results)]
                    ),
                )
            )
        case _:
            raise ValueError("Should not be here!")


def parse_input(input: str) -> DirZipper:
    lines = [
        (command, rest.strip())
        for (command, rest) in re.compile("\$ (cd|ls)([^$]*)").findall(input)
    ]
    state = DirZipper(
        parent=None,
        left=pvector(),
        right=pvector(),
        name="",
        focus=Directory("/", children=pvector()),
    )
    return reduce(apply_line, lines, state)


def find_smallest(dir: Directory, to_delete: int) -> int | None:
    my_size = dir.size()
    if my_size >= to_delete:
        try:
            return min(
                s
                for child in dir.children
                if isinstance(child, Directory)
                and (s := (find_smallest(child, to_delete))) is not None
            )
        except ValueError:
            return my_size
    else:
        return None


if __name__ == "__main__":
    with open("../inputs/day_07") as f:
        filesystem = parse_input(f.read()).top().focus
        print(f"Part 1: {filesystem.size_filtered(100000)}")
        total_space = 70000000
        needed_space = 30000000
        to_delete = needed_space - (total_space - filesystem.size())
        print(f"Part 2: {find_smallest(filesystem, to_delete)}")
