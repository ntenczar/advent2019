use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const INPUT_FILENAME: &'static str = "input";

fn main() -> Result<()> {
    let file = File::open(INPUT_FILENAME)?;
    let mut input: Vec<i64> = Vec::new();
    for line in BufReader::new(file).lines() {
        let l = line.unwrap();
        let parsed = l.parse::<i64>().unwrap();
        input.push(parsed);
    }
    part_one(&input);
    part_two(&input);
    Ok(())
}

fn part_one(input: &Vec<i64>) {
    let mut fuels: Vec<i64> = Vec::new();
    let mut fuel_sum: i64 = 0;
    for mass in input {
        let fuel = calc_fuel_for_mass(*mass);
        fuels.push(fuel);
        fuel_sum += fuel;
    }
    println!("Sum of fuel: {}", fuel_sum);
}

fn part_two(input: &Vec<i64>) {
    let mut fuels: Vec<i64> = Vec::new();
    let mut fuel_sum: i64 = 0;
    for mass in input {
        let fuel = calc_fuel_for_mass_and_its_fuel(*mass);
        fuels.push(fuel);
        fuel_sum += fuel;
    }
    println!("Sum of fuel: {}", fuel_sum);
}

fn calc_fuel_for_mass(mass: i64) -> i64 {
    (mass as f64 / 3.0).floor() as i64 - 2
}

fn calc_fuel_for_mass_and_its_fuel(mass: i64) -> i64 {
    let fuel = calc_fuel_for_mass(mass);
    if fuel > 0 {
        fuel + calc_fuel_for_mass_and_its_fuel(fuel)
    } else {
        0
    }
}

#[test]
fn test_calc_fuel_for_mass_and_its_fuel() {
    assert_eq!(calc_fuel_for_mass_and_its_fuel(14), 2);
    assert_eq!(calc_fuel_for_mass_and_its_fuel(1969), 966);
    assert_eq!(calc_fuel_for_mass_and_its_fuel(100756), 50346);
}
