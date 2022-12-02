mod day_1;
mod day_2;

pub use day_1::solve as solve_day1;
pub use day_2::solve as solve_day2;

fn main()
{
    let result;
    let day = 2;
    let task = 2;
    match day
    {
        1 => result = solve_day1(),
        2 => result = solve_day2(task),
        _ => result = 0i64,

    }
    println!("{}", result);

}
