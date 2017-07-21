extern crate time;
extern crate ordinal;

use ordinal::Ordinal;
use std::fmt;

#[derive(Debug)]
pub enum StTibs {
    Tomorrow,
    Yesterday
}

#[derive(Debug, Clone, Copy)]
pub enum Holidays {
    Mungay,
    Mojoday,
    Syaday,
    Zaraday,
    Maladay,
    Chaoflux,
    Discoflux,
    Confuflux,
    Bureflux,
    Afflux
}

pub struct DDate {
    pub year: u32,
    pub season: u8,
    pub day: u8,
    pub weekday: u8
}

impl DDate {
    pub fn now() -> Self {
        let t = time::now();

        Self::from_day_year(t.tm_yday as u16, (t.tm_year + 1900) as u32)
    }

    pub fn from_day_year(day: u16, year: u32) -> Self {
        let year = (year - 1970 + 3136) as u32;
        let weekday = (day % 5) as u8;

        let season = (day / 73) as u8;
        let day = (day % 73 + 1) as u8;

        DDate {
            year,
            season,
            day,
            weekday
        }
    }

    pub fn get_holiday(&self) -> Option<Holidays> {
        let apostle_holidays = [Holidays::Mungay, Holidays::Mojoday, Holidays::Syaday, Holidays::Zaraday, Holidays::Maladay];
        let seasonal_holidays = [Holidays::Chaoflux, Holidays::Discoflux, Holidays::Confuflux, Holidays::Bureflux, Holidays::Afflux];

        if self.day == 5 {
            Some(apostle_holidays[self.season as usize])
        } else if self.day == 50 {
            Some(seasonal_holidays[self.season as usize])
        } else {
            None
        }
    }

    pub fn get_st_tibs(&self) -> Option<StTibs> {
        // First St. Tib's in 3138.
        if (self.year - 2) % 4 == 0 && self.season == 0 {
            if self.day == 59 {
                return Some(StTibs::Tomorrow);
            }

            if self.day == 60 {
                return Some(StTibs::Yesterday);
            }
        }
        None
    }
}

impl fmt::Display for DDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let seasons = ["Chaos", "Discord", "Confusion", "Bureaucracy", "The Aftermath"];
        let weekdays = ["Sweetmorn", "Boomtime", "Pungenday", "Prickle-Prickle", "Setting Orange"];

        write!(f, "{}, the {} day of {}, year {}.",
               weekdays[self.weekday as usize],
               Ordinal::from(self.day),
               seasons[self.season as usize],
               self.year)
    }
}
