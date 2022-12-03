mod day01;
mod day02;
mod day03;

macro_rules! print_day {
    ($day:expr, $fun:path) => {
        println!("Day{:0>2}: {:?}", $day, $fun(format!("input/day{:0>2}.txt", $day).as_str())?);
    };
}

fn main()  -> Result<(), Box<dyn std::error::Error>> {
    print_day!(1, day01::solve);
    print_day!(2, day02::solve);
    print_day!(3, day03::solve);

    Ok(())
}