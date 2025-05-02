use intcode_rs::{
    machine,
    utils
};

fn main() {
    let code = utils::load_code_from_file("inputs/day5.txt");
    // part 1
    {
        let part1code = code.clone();
        let mut machine = machine::IntCodeMachine::new(part1code);
        machine.feed_input(1);
        machine.run();
        let result = machine
            .output()
            .last()
            .expect("output was empty");
        println!("part 1 result {result}");
    }
}
