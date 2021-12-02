use super::super::files;

#[derive(Debug)]
enum Instruction {
    Forward(i32),
    Up(i32),
    Down(i32),
    None,
}

#[derive(Debug)]
struct Position {
    x: i32,
    depth: i32,
}

impl Position {
    fn move_backwards(&mut self, value: &i32) {
        self.x -= value;
    }

    fn move_forward(&mut self, value: &i32) {
        self.x += value;
    }
    
    fn move_up(&mut self, value: &i32) {
        self.depth -= value;
    }

    fn move_down(&mut self, value: &i32) {
        self.depth += value;
    }
}

pub fn run_a() {
    // first we read the raw lines from the input file
    let lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_2/input.txt",
    ));

    // then we parse the lines into a list of Instructions
    let instructions: Vec<Instruction> = lines.iter().map(|line| {
        // here we are spliting a string using the separator ' ' (a char).
        // then we are "collecting" the Split object into a vector of &str (pointers to strings).
        let split_line = line.split(' ').collect::<Vec<&str>>();

        // this is a trick to destructure a vector. First, we create a slice using "[..]".
        // since this is a "refutable" pattern, we have to wrap it into a conditional.
        if let [raw_instruction, raw_param] = split_line[..] {
            // here we are parsing the second part into a number.
            let param: i32 = raw_param.parse().unwrap();

            // we use patter matching to map the strings to enums.
            let instruction = match raw_instruction {
                "forward" => Instruction::Forward(param),
                "down" => Instruction::Down(param),
                "up" => Instruction::Up(param),
                _ => Instruction::None
            };

            return instruction;
        }

        return Instruction::None;
    }).collect();

    // we do a quick check to see if the input had invalid values.
    let has_none = instructions.iter().any(|instruction| {
        match instruction {
            Instruction::None => true,
            _ => false,
        }
    });

    // if so, we finish early.
    if has_none {
        panic!("there was an invalid value");
    }

    // we initialize the position of the submarine using the Position struct.
    let initial_position = Position {
        x: 0,
        depth: 0,
    };

    // we fold (reduce) the instruction list. Pattern matching the instruction,
    // we call different methods of the Position struct.
    let final_position = instructions.iter().fold(initial_position, |mut position, instruction| {
        match instruction {
            Instruction::Forward(x) => position.move_forward(x),
            Instruction::Up(x) => position.move_up(x),
            Instruction::Down(x) => position.move_down(x),
            _ => (),
        };

        position
    });

    // finally we multiply the horizontal displacement times the depth, because
    // the exercise required it.
    let result = final_position.x * final_position.depth;
    println!("{}", result);
}