mod macros;
mod day01;
mod day02;
mod day03;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_day!(1, day01);
    print_day!(2, day02);
    print_day!(3, day03);

    Ok(())
}
