use super::basic_type::{Int16, UnsignedInt16, UnsignedInt32, UnsignedInt8};

const YEAR_OFFSET: UnsignedInt16 = 1900;
const YEAR_MAX: UnsignedInt32 = 9999 - YEAR_OFFSET as UnsignedInt32;
const MONTH_OFFSET: UnsignedInt8 = 1;
const MONTH_MAX: UnsignedInt32 = 12 - MONTH_OFFSET as UnsignedInt32;
const DAY_OFFSET: UnsignedInt8 = 1;
const DAY_IN_FOUR_CEDNTRY: UnsignedInt32 = 146_097;
const HOUR_MAX: UnsignedInt32 = 23;
const MINUTE_MAX: UnsignedInt32 = 59;
const SECOND_MAX: UnsignedInt32 = 59;
const NANOSECOND_MAX: UnsignedInt32 = 999_999_999;

pub fn get_max_day_count(year: UnsignedInt16, month: UnsignedInt8) -> UnsignedInt8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if year % 4 == 0 && (!(year % 100 == 0) || year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct EfiTime {
    year: UnsignedInt16,
    month: UnsignedInt8,
    day: UnsignedInt8,
    hour: UnsignedInt8,
    minute: UnsignedInt8,
    second: UnsignedInt8,
    pad1: UnsignedInt8,
    nanosecond: UnsignedInt32,
    time_zone: Int16,
    daylight: UnsignedInt8,
    pad2: UnsignedInt8,
}

impl EfiTime {
    pub const fn new(
        year: UnsignedInt16,
        month: UnsignedInt8,
        day: UnsignedInt8,
        hour: UnsignedInt8,
        minute: UnsignedInt8,
        second: UnsignedInt8,
        nanosecond: UnsignedInt32,
        time_zone: Int16,
        daylight: UnsignedInt8,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            pad1: 0,
            nanosecond,
            time_zone,
            daylight,
            pad2: 0,
        }
    }

    pub fn add_year(mut self, amount: UnsignedInt32) -> Self {
        if (YEAR_MAX - (self.year - YEAR_OFFSET) as UnsignedInt32) < amount {
            let rest = amount % (YEAR_MAX + 1) + (self.year - YEAR_OFFSET) as UnsignedInt32 % (YEAR_MAX + 1);
            self.year = (rest % (YEAR_MAX + 1)) as UnsignedInt16 + YEAR_OFFSET;
            self
        } else {
            self.year += amount as UnsignedInt16;
            self
        }
    }

    pub fn add_month(mut self, amount: UnsignedInt32) -> Self {
        if (MONTH_MAX - (self.month - MONTH_OFFSET) as UnsignedInt32) < amount {
            let mut extra_year = (self.month - MONTH_OFFSET) as UnsignedInt32 / (MONTH_MAX + 1);
            extra_year += amount / (MONTH_MAX + 1);
            let rest = amount % (MONTH_MAX + 1) + (self.month - MONTH_OFFSET) as UnsignedInt32 % (MONTH_MAX + 1);
            extra_year += rest / (MONTH_MAX + 1);
            self.month = (rest % (MONTH_MAX + 1)) as UnsignedInt8 + MONTH_OFFSET;
            self.add_year(extra_year)
        } else {
            self.month += amount as UnsignedInt8;
            self
        }
    }

    pub fn add_day(mut self, amount: UnsignedInt32) -> Self {
        self = self.add_year((amount / DAY_IN_FOUR_CEDNTRY) * 400);
        let mut amount = amount % DAY_IN_FOUR_CEDNTRY;
        'a: loop {
            let day_max = (get_max_day_count(self.year, self.month) - DAY_OFFSET) as UnsignedInt32;
            if day_max < amount {
                amount -= day_max + 1;
                self = self.add_month(1);
            }
            else {
                let rest = (self.day - DAY_OFFSET) as UnsignedInt32 + amount;
                if rest >= day_max + 1 {
                    self = self.add_month(1);
                }
                self.day = (rest % (day_max + 1)) as UnsignedInt8 + DAY_OFFSET;
                break 'a self;
            }
        }
    }

    pub fn add_hour(mut self, amount: UnsignedInt32) -> Self {
        if (HOUR_MAX - self.hour as UnsignedInt32) < amount {
            let mut extra_day = self.hour as UnsignedInt32 / (HOUR_MAX + 1);
            extra_day += amount / (HOUR_MAX + 1);
            let rest = amount % (HOUR_MAX + 1) + self.hour as UnsignedInt32 % (HOUR_MAX + 1);
            extra_day += rest / (HOUR_MAX + 1);
            self.hour = (rest % (HOUR_MAX + 1)) as UnsignedInt8;
            self.add_day(extra_day)
        } else {
            self.hour += amount as UnsignedInt8;
            self
        }
    }

    pub fn add_minute(mut self, amount: UnsignedInt32) -> Self {
        if (MINUTE_MAX - self.minute as UnsignedInt32) < amount {
            let mut extra_hour = self.minute as UnsignedInt32 / (MINUTE_MAX + 1);
            extra_hour += amount / (MINUTE_MAX + 1);
            let rest = amount % (MINUTE_MAX + 1) + self.minute as UnsignedInt32 % (MINUTE_MAX + 1);
            extra_hour += rest / (MINUTE_MAX + 1);
            self.minute = (rest % (MINUTE_MAX + 1)) as UnsignedInt8;
            self.add_hour(extra_hour)
        } else {
            self.minute += amount as UnsignedInt8;
            self
        }
    }

    pub fn add_second(mut self, amount: UnsignedInt32) -> Self {
        if (SECOND_MAX - self.second as UnsignedInt32) < amount {
            let mut extra_minute = self.second as UnsignedInt32 / (SECOND_MAX + 1);
            extra_minute += amount / (SECOND_MAX + 1);
            let rest = amount % (SECOND_MAX + 1) + self.second as UnsignedInt32 % (SECOND_MAX + 1);
            extra_minute += rest / (SECOND_MAX + 1);
            self.second = (rest % (SECOND_MAX + 1)) as UnsignedInt8;
            self.add_minute(extra_minute)
        } else {
            self.second += amount as UnsignedInt8;
            self
        }
    }

    pub fn add_nanosecond(mut self, amount: UnsignedInt32) -> Self {
        if NANOSECOND_MAX - self.nanosecond < amount {
            let mut extra_second = self.nanosecond / (NANOSECOND_MAX + 1);
            extra_second += amount / (NANOSECOND_MAX + 1);
            let rest = amount % (NANOSECOND_MAX + 1) + self.nanosecond % (NANOSECOND_MAX + 1);
            extra_second += rest / (NANOSECOND_MAX + 1);
            self.nanosecond = rest % (NANOSECOND_MAX + 1);
            self.add_second(extra_second)
        } else {
            self.nanosecond += amount;
            self
        }
    }

    pub fn sub_year(mut self, amount: UnsignedInt32) -> Self {
        if ((self.year - YEAR_OFFSET) as UnsignedInt32) < amount {
            let rest = amount - (self.year - YEAR_OFFSET) as UnsignedInt32;
            self.year = ((YEAR_MAX + 1 - rest % (YEAR_MAX + 1)) % (YEAR_MAX + 1)) as UnsignedInt16 + YEAR_OFFSET;
            self
        } else {
            self.year -= amount as UnsignedInt16;
            self
        }
    }

    pub fn sub_month(mut self, amount: UnsignedInt32) -> Self {
        if ((self.month - MONTH_OFFSET) as UnsignedInt32) < amount {
            let rest = amount - (self.month - MONTH_OFFSET) as UnsignedInt32;
            let extra_year = rest / (MONTH_MAX + 1) + 1;
            self.month = ((MONTH_MAX + 1 - rest % (MONTH_MAX + 1)) % (MONTH_MAX + 1)) as UnsignedInt8 + MONTH_OFFSET;
            self.sub_year(extra_year)
        } else {
            self.month -= amount as UnsignedInt8;
            self
        }
    }

    pub fn sub_day(mut self, amount: UnsignedInt32) -> Self {
        self = self.sub_year((amount / DAY_IN_FOUR_CEDNTRY) * 400);
        let mut amount = amount % DAY_IN_FOUR_CEDNTRY;
        'a: loop {
            if ((self.day - DAY_OFFSET) as UnsignedInt32) < amount {
                self = self.sub_month(1);
                let day_max = (get_max_day_count(self.year, self.month) - DAY_OFFSET) as UnsignedInt32;
                if day_max < amount {
                    amount -= day_max + 1;
                }
                else {
                    self.day = (day_max + 1 - (amount - (self.day - DAY_OFFSET) as UnsignedInt32)) as UnsignedInt8 + DAY_OFFSET;
                    break 'a self;
                }
            }
            else {
                self.day -= amount as UnsignedInt8;
                break 'a self;
            }
        }
    }

    pub fn sub_hour(mut self, amount: UnsignedInt32) -> Self {
        if (self.hour as UnsignedInt32) < amount {
            let rest = amount - self.hour as UnsignedInt32;
            let extra_day = rest / (HOUR_MAX + 1) + 1;
            self.hour = ((HOUR_MAX + 1 - rest % (HOUR_MAX + 1)) % (HOUR_MAX + 1)) as UnsignedInt8;
            self.sub_day(extra_day)
        } else {
            self.hour -= amount as UnsignedInt8;
            self
        }
    }

    pub fn sub_minute(mut self, amount: UnsignedInt32) -> Self {
        if (self.minute as UnsignedInt32) < amount {
            let rest = amount - self.minute as UnsignedInt32;
            let extra_hour = rest / (MINUTE_MAX + 1) + 1;
            self.minute = ((MINUTE_MAX + 1 - rest % (MINUTE_MAX + 1)) % (MINUTE_MAX + 1)) as UnsignedInt8;
            self.sub_hour(extra_hour)
        } else {
            self.minute -= amount as UnsignedInt8;
            self
        }
    }

    pub fn sub_second(mut self, amount: UnsignedInt32) -> Self {
        if (self.second as UnsignedInt32) < amount {
            let rest = amount - self.second as UnsignedInt32;
            let extra_minute = rest / (SECOND_MAX + 1) + 1;
            self.second = ((SECOND_MAX + 1 - rest % (SECOND_MAX + 1)) % (SECOND_MAX + 1)) as UnsignedInt8;
            self.sub_minute(extra_minute)
        } else {
            self.second -= amount as UnsignedInt8;
            self
        }
    }

    pub fn sub_nanosecond(mut self, amount: UnsignedInt32) -> Self {
        if self.nanosecond < amount {
            let rest = amount - self.nanosecond;
            let extra_second = rest / (NANOSECOND_MAX + 1) + 1;
            self.nanosecond = (NANOSECOND_MAX + 1 - rest % (NANOSECOND_MAX + 1)) % (NANOSECOND_MAX + 1);
            self.sub_second(extra_second)
        } else {
            self.nanosecond -= amount;
            self
        }
    }

    pub fn get_
}

impl Eq for EfiTime {}
impl PartialEq for EfiTime {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.hour == other.hour
            && self.minute == other.minute
            && self.second == other.second
            && self.nanosecond == other.nanosecond
    }
}
