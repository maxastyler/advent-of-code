use std::collections::VecDeque;

enum CompState {
    ok,
    waiting,
    finished,
}

#[derive(Default, Debug)]
struct Comp {
    p: usize,
    tape: Vec<i32>,
    input: VecDeque<i32>,
    output: VecDeque<i32>,
}

impl Comp {
    fn v(&self, index: usize, mode: i32) -> i32 {
        match mode {
            0 => self.tape[self.tape[index] as usize],
            _ => self.tape[index],
        }
    }

    fn step(&mut self) -> CompState {
        let ins = self.tape[self.p];
        let op = ins % 100;
        let m1 = (ins / 100) % 10;
        let m2 = (ins / 1000) % 10;
        match op {
            1 => {
                self.p += 4;
                let i = self.v(self.p + 3, 0) as usize;
                self.tape[i] = self.v(self.p + 1, m1) + self.v(self.p + 2, m2);
                CompState::ok
            }
            2 => {
                self.p += 4;
                let i = self.v(self.p + 3, 0) as usize;
                self.tape[i] = self.v(self.p + 1, m1) * self.v(self.p + 2, m2);
                CompState::ok
            }
            3 => {
                if let i = self.input.pop_front() {
                    self.p += 2;
                    let i = self.v(self.p + 1, 0) as usize;
                    self.tape[i] = 
                }
            }
            4 => {}
            5 => {}
            6 => {}
            7 => {}
            8 => {}
            99 => {}
            _ => panic!("Bad input"),
        }
    }
}

fn main() {
    let mut c = Comp {
        tape: vec![1, 0, 0, 0, 0],

        ..Default::default()
    };
    c.step();
    println!("{:?}", c);
}
