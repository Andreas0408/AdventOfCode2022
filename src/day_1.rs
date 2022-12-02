mod data_import;
pub use data_import::lines_from_file;

use std::borrow::Borrow;

struct Elf
{
    backpack: Vec<i64>
}

impl Elf{
    fn add_to_back_pack(&mut self, new_value: i64)
    {
        self.backpack.push(new_value);
    }
    fn add_all_calories(& self) -> i64
    {
        let sum : i64 = self.backpack.iter().sum();
        return sum;
    }
}

pub fn solve() -> i64
{
    let data = lines_from_file("input_day1.txt");

    let mut elfes = Vec::new();
    elfes.push(Elf{
        backpack : Vec::new()
    });
    
    for line in data
    {
        if line.is_empty()
        {
            elfes.push(Elf{
                backpack : Vec::new()});
        }
        else {
            Elf::add_to_back_pack(elfes.last_mut().unwrap(), line.parse::<i64>().unwrap());
        }
    }

    let mut greatest_calories = Vec::new();
    for elf in elfes
    {
        let sum = Elf::add_all_calories(elf.borrow());
        greatest_calories.push(sum);
    }
    greatest_calories.sort();
    let top_three = greatest_calories[greatest_calories.len() -1] + greatest_calories[greatest_calories.len() -2] + greatest_calories[greatest_calories.len() -3];

    return top_three;
}