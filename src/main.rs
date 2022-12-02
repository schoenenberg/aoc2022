mod day01;
mod day02;

fn main()  -> Result<(), Box<dyn std::error::Error>> {
    println!("Day01: {:#?}", day01::solve("input/day01.txt")?);
    println!("Day02: {:#?}", day02::solve("input/day02.txt")?);

    Ok(())
}