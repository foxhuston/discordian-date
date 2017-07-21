extern crate time;
extern crate ordinal;

use ordinal::Ordinal;


fn main() {
    let seasons = ["Chaos", "Discord", "Confusion", "Bureaucracy", "The Aftermath"];
    let days = ["Sweetmorn", "Boomtime", "Pungenday", "Prickle-Prickle", "Setting Orange"];
    let apostle_holidays = ["Mungay", "Mojoday", "Syaday", "Zaraday", "Maladay"];
    let seasonal_holidays = ["Chaoflux", "Discoflux", "Confuflux", "Bureflux", "Afflux"];

    let t = time::now();
    let day = t.tm_yday;
    let year = t.tm_year - 70 + 3136;
    let weekday = days[(day % 5) as usize];

    let season = day / 73;
    let dday = day % 73 + 1;

    println!("{}, the {} day of {}, year {}.",
            weekday,
            Ordinal::from(dday),
            seasons[season as usize],
            year);

    if dday == 5 {
       println!("Today is {}", apostle_holidays[season as usize]);
    } else if dday == 50 {
       println!("Today is {}", seasonal_holidays[season as usize]);
    }
}
