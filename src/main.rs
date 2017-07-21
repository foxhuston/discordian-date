extern crate discordian_date;

use discordian_date::*;

fn main() {
    let ddate = DDate::now();
    println!("{}", ddate);

    if let Some(holiday) = ddate.get_holiday() {
        println!("Today is {:?}.", holiday);
    }

    if let Some(st_tibs) = ddate.get_st_tibs() {
        match st_tibs {
            StTibs::Tomorrow => println!("St. Tib's is tomorrow."),
            _ => println!("St. Tib's was yesterday.")
        }
    }
}
