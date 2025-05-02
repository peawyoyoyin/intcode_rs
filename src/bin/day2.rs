use intcode_rs::{machine, utils};

fn main() {
    let code = utils::load_code_from_file("inputs/day2.txt");
    // part 1
    {
        let mut part1code = code.clone();
        part1code[1] = 12;
        part1code[2] = 2;

        let mut machine = machine::IntCodeMachine::new(part1code);
        machine.run();
        assert!(machine.halted(), "machine did not halt after run");
        let result = machine.memory(0).expect("cannot find memory at address 0");
        println!("part 1 result {result}");
    }

    // part 2
    {
        let target: isize = 19690720;
        let mut ans = 99999999;
        for noun in 0..100 {
            for verb in 0..100 {
                let mut new_code = code.clone();
                new_code[1] = noun;
                new_code[2] = verb;
                let mut machine = machine::IntCodeMachine::new(new_code);
                machine.run();
                assert!(machine.halted(), "machine did not halt after run, noun={noun}, verb={verb}");
                let result = machine.memory(0).expect(&format!(
                    "cannot find memory at address 0 while running with noun={noun}, verb={verb}"
                ));
                if result == &target {
                    ans = 100 * noun + verb;
                }
            }
        }
        println!("part 2 result {ans}");
    }
}
