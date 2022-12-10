use crate::utils::file;
use log::{debug, info};

fn simulate_cpu(commands: &Vec<(file::CPUCommand, i32)>) -> Vec<i32> {
    let mut register = 1;
    commands.iter().fold(Vec::new(), |mut cycles, itm| {
        let (command, val) = itm;
        match command {
            file::CPUCommand::Noop => cycles.push(register),
            file::CPUCommand::Addx => {
                cycles.push(register);
                cycles.push(register);
                register += val;
            },
        }
        cycles
    })
}

fn to_crt(cycles: &Vec<i32>) -> Vec<String> {
    let mut cycle = 1;
    let mut cursor = 0;
    cycles.iter().fold(Vec::new(), |mut crt, register| {
        if cycle % 40 == 1 {
            crt.push(String::from(""))
        }
        let mut row = String::from(crt.pop().unwrap().as_str());
        let sprite = vec![*register-1, *register, *register+1];
        if sprite[0] == cursor || sprite[1] == cursor || sprite[2] == cursor {
            row.push('#');
        } else {
            row.push('.');
        }
        crt.push(row);
        cycle += 1;
        cursor += 1;
        if cursor == 40 {
            cursor = 0;
        }
        crt
    })
}

pub fn task1(path: &str) {
    if let Ok(input) = file::read_lines(path) {
        let commands = file::to_commands::<i32>(input);
        debug!("Found {} commands", commands.len());
        let cycles = simulate_cpu(&commands);
        debug!("Simulated {} cycles", cycles.len());
        let cycle20 = cycles[19] * 20;
        let cycle60 = cycles[59] * 60;
        let cycle100 = cycles[99] * 100;
        let cycle140 = cycles[139] * 140;
        let cycle180 = cycles[179] * 180;
        let cycle220 = cycles[219] * 220;
        info!("Sum of cycles {}", cycle20 + cycle60 + cycle100 + cycle140 + cycle180 + cycle220);
    }
}

pub fn task2(path: &str) {
    if let Ok(input) = file::read_lines(path) {
        let commands = file::to_commands::<i32>(input);
        debug!("Found {} commands", commands.len());
        let cycles = simulate_cpu(&commands);
        debug!("Simulated {} cycles", cycles.len());
        let crt = to_crt(&cycles);
        info!("CRT Output:");
        for line in crt.iter() {
            info!("{}", line);
        }
    } 
}
