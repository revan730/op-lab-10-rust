use std::{io, mem};

enum StackNode {
    Empty,
    Value(char, Box<StackNode>)
}

struct Stack {
    pub stack_top: StackNode,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack_top: StackNode::Empty,
        }
    }

    pub fn push(&mut self, c: char) {
        let tmp_node: StackNode = mem::replace(&mut self.stack_top, StackNode::Empty);
        self.stack_top = StackNode::Value(c, Box::new(tmp_node));
    }

    pub fn pop(&mut self) -> Option<char> {
        match mem::replace(&mut self.stack_top, StackNode::Empty) {
            StackNode::Empty => None,
            StackNode::Value(c, next) => {
                self.stack_top = *next;

                Some(c)
            },
        }
    }
}


fn input_text(stack: &mut Stack) {
    let stdin = io::stdin();
    let mut stop = false;

    while !stop {
        let mut input_str: String = String::new();
        println!("Input text or press Ctrl-D to stop:");
        let read: Result<usize, io::Error> = stdin.read_line(&mut input_str);
        match read {
            Ok(count) => {
                if count == 0 {
                    stop = true;
                }
            },
            Err(_) => {
                println!("Try again");
                continue;
            },
        }
        
        // Push onto stack
        input_str.chars().for_each(|c: char| stack.push(c));
    }
    println!("Finished input");
}

fn output_text(stack: &mut Stack) {
    loop {
        let next = stack.pop();
        match  next {
            Some(c) => print!("{}", c),
            None => break,
        }
    }
}

fn main() {
    let mut stack_top = Stack::new();
    input_text(&mut stack_top);
    output_text(&mut stack_top);
}
