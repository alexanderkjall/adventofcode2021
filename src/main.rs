use clap::Parser;
use nom::Err;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

#[derive(Debug)]
pub enum Error {
    Generic(&'static str),
    GenericDyn(String),
    IO(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<nom::Err<nom::error::Error<&str>>> for Error {
    fn from(err: Err<nom::error::Error<&str>>) -> Self {
        Error::GenericDyn(format!("{:?}", err))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::GenericDyn(format!("{:?}", err))
    }
}

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Alexander Kjäll <alexander.kjall@gmail.com>"
)]
struct Arguments {
    #[clap(short, long)]
    day: u8,
}

fn main() {
    let args: Arguments = Arguments::parse();

    let res = match args.day {
        1 => day1::calculate(),
        2 => day2::calculate(),
        3 => day3::calculate(),
        4 => day4::calculate(),
        5 => day5::calculate(),
        6 => day6::calculate(),
        7 => day7::calculate(),
        8 => day8::calculate(),
        9 => day9::calculate(),
        10 => day10::calculate(),
        11 => day11::calculate(),
        12 => day12::calculate(),
        13 => day13::calculate(),
        14 => day14::calculate(),
        15 => day15::calculate(),
        16 => day16::calculate(),
        _ => Err(Error::Generic("illegal day")),
    };

    match res {
        Ok((part1, part2)) => println!("day {}\npart 1: {}\npart 2: {}", args.day, part1, part2),
        Err(err) => println!("{:?}", err),
    }
}
