mod day01;
mod day02;
mod day03;

fn main()  -> Result<(), Box<dyn std::error::Error>> {
    println!("Day01: {:#?}", day01::solve("input/day01.txt")?);
    println!("Day02: {:#?}", day02::solve("input/day02.txt")?);
    println!("Day03: {:#?}", day03::solve("input/day03.txt")?);

    Ok(())
}