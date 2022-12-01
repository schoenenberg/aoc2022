mod day01;

pub fn solve_all() -> Result<(), Box<dyn std::error::Error>> {
    println!("Day01: {:#?}", day01::solve("input/day01.txt")?);

    Ok(())
}
