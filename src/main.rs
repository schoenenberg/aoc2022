mod day01;
mod day02;
mod day03;
mod day04;
mod macros;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_day!(1, day01);
    print_day!(2, day02);
    print_day!(3, day03);
    print_day!(4, day04);

    Ok(())
}
