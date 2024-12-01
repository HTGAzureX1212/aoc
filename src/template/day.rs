use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[cfg(feature = "today")]
use chrono::{Datelike, FixedOffset, Utc};

#[cfg(feature = "today")]
const SERVER_UTC_OFFSET: i32 = -5;

/// A valid day number of advent (i.e. an integer in range 1 to 25).
///
/// # Display
/// This value displays as a two digit number.
///
/// ```
/// # use advent_of_code::Day;
/// let day = Day::new(8).unwrap();
/// assert_eq!(day.to_string(), "08")
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Day(u16, u8);

impl Day {
    /// Creates a [`Day`] from the provided value if it's in the valid range,
    /// returns [`None`] otherwise.
    pub fn new(year: u16, day: u8) -> Option<Self> {
        if year < 2015 || year > 2024 {
            return None;
        }
        if day == 0 || day > 25 {
            return None;
        }
        Some(Self(year, day))
    }

    // Not part of the public API
    #[doc(hidden)]
    pub const fn __new_unchecked(year: u16, day: u8) -> Self {
        Self(year, day)
    }

    /// Converts the [`Day`] into an [`u8`].
    pub fn into_inner(self) -> (u16, u8) {
        (self.0, self.1)
    }
}

#[cfg(feature = "today")]
impl Day {
    /// Returns the current day if it's between the 1st and the 25th of december, `None` otherwise.
    pub fn today() -> Option<Self> {
        let offset = FixedOffset::east_opt(SERVER_UTC_OFFSET * 3600)?;
        let today = Utc::now().with_timezone(&offset);
        if today.month() == 12 && today.day() <= 25 {
            Self::new(
                u16::try_from(today.year()).ok()?,
                u8::try_from(today.day()).ok()?,
            )
        } else {
            None
        }
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{:02}", self.0, self.1)
    }
}

impl PartialEq<(u16, u8)> for Day {
    fn eq(&self, other: &(u16, u8)) -> bool {
        self.0.eq(&other.0) && self.1.eq(&other.1)
    }
}

impl PartialOrd<(u16, u8)> for Day {
    fn partial_cmp(&self, other: &(u16, u8)) -> Option<std::cmp::Ordering> {
        let year = self.0.partial_cmp(&other.0);
        let day = self.1.partial_cmp(&other.1);

        if let Some(ordering1) = year
            && let Some(ordering2) = day
        {
            Some(ordering1.then(ordering2))
        } else {
            None
        }
    }
}

/* -------------------------------------------------------------------------- */

impl FromStr for Day {
    type Err = DayFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('-').collect::<Vec<_>>();
        let year = parts[0].parse().map_err(|_| DayFromStrError)?;
        let day = parts[1].parse().map_err(|_| DayFromStrError)?;
        Self::new(year, day).ok_or(DayFromStrError)
    }
}

/// An error which can be returned when parsing a [`Day`].
#[derive(Debug)]
pub struct DayFromStrError;

impl Error for DayFromStrError {}

impl Display for DayFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "expecting a year number from 2015 to 2024 and \
            a day number between 1 and 25",
        )
    }
}

/* -------------------------------------------------------------------------- */

/// An iterator that yields every day of advent from the 1st to the 25th.
pub fn all_days() -> AllDays {
    AllDays::new()
}

/// An iterator that yields every day of advent from the 1st to the 25th.
pub struct AllDays {
    current_year: u16,
    current_day: u8,
}

impl AllDays {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            current_year: 2015,
            current_day: 1,
        }
    }
}

impl Iterator for AllDays {
    type Item = Day;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_year > 2024 {
            return None;
        }
        // NOTE: the iterator starts at 1 and we have verified that the value is not above 25.
        let day = Day(self.current_year, self.current_day);
        self.current_day += 1;
        if self.current_day > 25 && self.current_year <= 2024 {
            self.current_year += 1;
            self.current_day = 1;
        }

        Some(day)
    }
}

/* -------------------------------------------------------------------------- */

/// Creates a [`Day`] value in a const context.
#[macro_export]
macro_rules! day {
    ($year:literal, $day:literal) => {{
        const _ASSERT: () = assert!(
            $day != 0 && $day <= 25 && $year >= 2015 && $year <= 2024,
            concat!(
                "invalid day number `",
                $day,
                "`, expecting a value between 1 and 25"
            ),
        );
        $crate::template::Day::__new_unchecked($year, $day)
    }};
}

/* -------------------------------------------------------------------------- */

#[cfg(feature = "test_lib")]
mod tests {
    use super::{all_days, Day};

    #[test]
    fn all_days_iterator() {
        let mut iter = all_days();

        assert_eq!(iter.next(), Some(Day(2015, 1)));
        assert_eq!(iter.next(), Some(Day(2015, 2)));
        assert_eq!(iter.next(), Some(Day(2015, 3)));
        assert_eq!(iter.next(), Some(Day(2015, 4)));
        assert_eq!(iter.next(), Some(Day(2015, 5)));
        assert_eq!(iter.next(), Some(Day(2015, 6)));
        assert_eq!(iter.next(), Some(Day(2015, 7)));
        assert_eq!(iter.next(), Some(Day(2015, 8)));
        assert_eq!(iter.next(), Some(Day(2015, 9)));
        assert_eq!(iter.next(), Some(Day(2015, 10)));
        assert_eq!(iter.next(), Some(Day(2015, 11)));
        assert_eq!(iter.next(), Some(Day(2015, 12)));
        assert_eq!(iter.next(), Some(Day(2015, 13)));
        assert_eq!(iter.next(), Some(Day(2015, 14)));
        assert_eq!(iter.next(), Some(Day(2015, 15)));
        assert_eq!(iter.next(), Some(Day(2015, 16)));
        assert_eq!(iter.next(), Some(Day(2015, 17)));
        assert_eq!(iter.next(), Some(Day(2015, 18)));
        assert_eq!(iter.next(), Some(Day(2015, 19)));
        assert_eq!(iter.next(), Some(Day(2015, 20)));
        assert_eq!(iter.next(), Some(Day(2015, 21)));
        assert_eq!(iter.next(), Some(Day(2015, 22)));
        assert_eq!(iter.next(), Some(Day(2015, 23)));
        assert_eq!(iter.next(), Some(Day(2015, 24)));
        assert_eq!(iter.next(), Some(Day(2015, 25)));
        assert_eq!(iter.next(), Some(Day(2015, 1)));
        assert_eq!(iter.next(), Some(Day(2016, 2)));
        assert_eq!(iter.next(), Some(Day(2016, 3)));
        assert_eq!(iter.next(), Some(Day(2016, 4)));
        assert_eq!(iter.next(), Some(Day(2016, 5)));
        assert_eq!(iter.next(), Some(Day(2016, 6)));
        assert_eq!(iter.next(), Some(Day(2016, 7)));
        assert_eq!(iter.next(), Some(Day(2016, 8)));
        assert_eq!(iter.next(), Some(Day(2016, 9)));
        assert_eq!(iter.next(), Some(Day(2016, 10)));
        assert_eq!(iter.next(), Some(Day(2016, 11)));
        assert_eq!(iter.next(), Some(Day(2016, 12)));
        assert_eq!(iter.next(), Some(Day(2016, 13)));
        assert_eq!(iter.next(), Some(Day(2016, 14)));
        assert_eq!(iter.next(), Some(Day(2016, 15)));
        assert_eq!(iter.next(), Some(Day(2016, 16)));
        assert_eq!(iter.next(), Some(Day(2016, 17)));
        assert_eq!(iter.next(), Some(Day(2016, 18)));
        assert_eq!(iter.next(), Some(Day(2016, 19)));
        assert_eq!(iter.next(), Some(Day(2016, 20)));
        assert_eq!(iter.next(), Some(Day(2016, 21)));
        assert_eq!(iter.next(), Some(Day(2016, 22)));
        assert_eq!(iter.next(), Some(Day(2016, 23)));
        assert_eq!(iter.next(), Some(Day(2016, 24)));
        assert_eq!(iter.next(), Some(Day(2016, 25)));
        assert_eq!(iter.next(), Some(Day(2017, 1)));
        assert_eq!(iter.next(), Some(Day(2017, 2)));
        assert_eq!(iter.next(), Some(Day(2017, 3)));
        assert_eq!(iter.next(), Some(Day(2017, 4)));
        assert_eq!(iter.next(), Some(Day(2017, 5)));
        assert_eq!(iter.next(), Some(Day(2017, 6)));
        assert_eq!(iter.next(), Some(Day(2017, 7)));
        assert_eq!(iter.next(), Some(Day(2017, 8)));
        assert_eq!(iter.next(), Some(Day(2017, 9)));
        assert_eq!(iter.next(), Some(Day(2017, 10)));
        assert_eq!(iter.next(), Some(Day(2017, 11)));
        assert_eq!(iter.next(), Some(Day(2017, 12)));
        assert_eq!(iter.next(), Some(Day(2017, 13)));
        assert_eq!(iter.next(), Some(Day(2017, 14)));
        assert_eq!(iter.next(), Some(Day(2017, 15)));
        assert_eq!(iter.next(), Some(Day(2017, 16)));
        assert_eq!(iter.next(), Some(Day(2017, 17)));
        assert_eq!(iter.next(), Some(Day(2017, 18)));
        assert_eq!(iter.next(), Some(Day(2017, 19)));
        assert_eq!(iter.next(), Some(Day(2017, 20)));
        assert_eq!(iter.next(), Some(Day(2017, 21)));
        assert_eq!(iter.next(), Some(Day(2017, 22)));
        assert_eq!(iter.next(), Some(Day(2017, 23)));
        assert_eq!(iter.next(), Some(Day(2017, 24)));
        assert_eq!(iter.next(), Some(Day(2017, 25)));
        assert_eq!(iter.next(), Some(Day(2018, 1)));
        assert_eq!(iter.next(), Some(Day(2018, 2)));
        assert_eq!(iter.next(), Some(Day(2018, 3)));
        assert_eq!(iter.next(), Some(Day(2018, 4)));
        assert_eq!(iter.next(), Some(Day(2018, 5)));
        assert_eq!(iter.next(), Some(Day(2018, 6)));
        assert_eq!(iter.next(), Some(Day(2018, 7)));
        assert_eq!(iter.next(), Some(Day(2018, 8)));
        assert_eq!(iter.next(), Some(Day(2018, 9)));
        assert_eq!(iter.next(), Some(Day(2018, 10)));
        assert_eq!(iter.next(), Some(Day(2018, 11)));
        assert_eq!(iter.next(), Some(Day(2018, 12)));
        assert_eq!(iter.next(), Some(Day(2018, 13)));
        assert_eq!(iter.next(), Some(Day(2018, 14)));
        assert_eq!(iter.next(), Some(Day(2018, 15)));
        assert_eq!(iter.next(), Some(Day(2018, 16)));
        assert_eq!(iter.next(), Some(Day(2018, 17)));
        assert_eq!(iter.next(), Some(Day(2018, 18)));
        assert_eq!(iter.next(), Some(Day(2018, 19)));
        assert_eq!(iter.next(), Some(Day(2018, 20)));
        assert_eq!(iter.next(), Some(Day(2018, 21)));
        assert_eq!(iter.next(), Some(Day(2018, 22)));
        assert_eq!(iter.next(), Some(Day(2018, 23)));
        assert_eq!(iter.next(), Some(Day(2018, 24)));
        assert_eq!(iter.next(), Some(Day(2018, 25)));
        assert_eq!(iter.next(), Some(Day(2019, 1)));
        assert_eq!(iter.next(), Some(Day(2019, 2)));
        assert_eq!(iter.next(), Some(Day(2019, 3)));
        assert_eq!(iter.next(), Some(Day(2019, 4)));
        assert_eq!(iter.next(), Some(Day(2019, 5)));
        assert_eq!(iter.next(), Some(Day(2019, 6)));
        assert_eq!(iter.next(), Some(Day(2019, 7)));
        assert_eq!(iter.next(), Some(Day(2019, 8)));
        assert_eq!(iter.next(), Some(Day(2019, 9)));
        assert_eq!(iter.next(), Some(Day(2019, 10)));
        assert_eq!(iter.next(), Some(Day(2019, 11)));
        assert_eq!(iter.next(), Some(Day(2019, 12)));
        assert_eq!(iter.next(), Some(Day(2019, 13)));
        assert_eq!(iter.next(), Some(Day(2019, 14)));
        assert_eq!(iter.next(), Some(Day(2019, 15)));
        assert_eq!(iter.next(), Some(Day(2019, 16)));
        assert_eq!(iter.next(), Some(Day(2019, 17)));
        assert_eq!(iter.next(), Some(Day(2019, 18)));
        assert_eq!(iter.next(), Some(Day(2019, 19)));
        assert_eq!(iter.next(), Some(Day(2019, 20)));
        assert_eq!(iter.next(), Some(Day(2019, 21)));
        assert_eq!(iter.next(), Some(Day(2019, 22)));
        assert_eq!(iter.next(), Some(Day(2019, 23)));
        assert_eq!(iter.next(), Some(Day(2019, 24)));
        assert_eq!(iter.next(), Some(Day(2019, 25)));
        assert_eq!(iter.next(), Some(Day(2020, 1)));
        assert_eq!(iter.next(), Some(Day(2020, 2)));
        assert_eq!(iter.next(), Some(Day(2020, 3)));
        assert_eq!(iter.next(), Some(Day(2020, 4)));
        assert_eq!(iter.next(), Some(Day(2020, 5)));
        assert_eq!(iter.next(), Some(Day(2020, 6)));
        assert_eq!(iter.next(), Some(Day(2020, 7)));
        assert_eq!(iter.next(), Some(Day(2020, 8)));
        assert_eq!(iter.next(), Some(Day(2020, 9)));
        assert_eq!(iter.next(), Some(Day(2020, 10)));
        assert_eq!(iter.next(), Some(Day(2020, 11)));
        assert_eq!(iter.next(), Some(Day(2020, 12)));
        assert_eq!(iter.next(), Some(Day(2020, 13)));
        assert_eq!(iter.next(), Some(Day(2020, 14)));
        assert_eq!(iter.next(), Some(Day(2020, 15)));
        assert_eq!(iter.next(), Some(Day(2020, 16)));
        assert_eq!(iter.next(), Some(Day(2020, 17)));
        assert_eq!(iter.next(), Some(Day(2020, 18)));
        assert_eq!(iter.next(), Some(Day(2020, 19)));
        assert_eq!(iter.next(), Some(Day(2020, 20)));
        assert_eq!(iter.next(), Some(Day(2020, 21)));
        assert_eq!(iter.next(), Some(Day(2020, 22)));
        assert_eq!(iter.next(), Some(Day(2020, 23)));
        assert_eq!(iter.next(), Some(Day(2020, 24)));
        assert_eq!(iter.next(), Some(Day(2020, 25)));
        assert_eq!(iter.next(), Some(Day(2021, 1)));
        assert_eq!(iter.next(), Some(Day(2021, 2)));
        assert_eq!(iter.next(), Some(Day(2021, 3)));
        assert_eq!(iter.next(), Some(Day(2021, 4)));
        assert_eq!(iter.next(), Some(Day(2021, 5)));
        assert_eq!(iter.next(), Some(Day(2021, 6)));
        assert_eq!(iter.next(), Some(Day(2021, 7)));
        assert_eq!(iter.next(), Some(Day(2021, 8)));
        assert_eq!(iter.next(), Some(Day(2021, 9)));
        assert_eq!(iter.next(), Some(Day(2021, 10)));
        assert_eq!(iter.next(), Some(Day(2021, 11)));
        assert_eq!(iter.next(), Some(Day(2021, 12)));
        assert_eq!(iter.next(), Some(Day(2021, 13)));
        assert_eq!(iter.next(), Some(Day(2021, 14)));
        assert_eq!(iter.next(), Some(Day(2021, 15)));
        assert_eq!(iter.next(), Some(Day(2021, 16)));
        assert_eq!(iter.next(), Some(Day(2021, 17)));
        assert_eq!(iter.next(), Some(Day(2021, 18)));
        assert_eq!(iter.next(), Some(Day(2021, 19)));
        assert_eq!(iter.next(), Some(Day(2021, 20)));
        assert_eq!(iter.next(), Some(Day(2021, 21)));
        assert_eq!(iter.next(), Some(Day(2021, 22)));
        assert_eq!(iter.next(), Some(Day(2021, 23)));
        assert_eq!(iter.next(), Some(Day(2021, 24)));
        assert_eq!(iter.next(), Some(Day(2021, 25)));
        assert_eq!(iter.next(), Some(Day(2022, 1)));
        assert_eq!(iter.next(), Some(Day(2022, 2)));
        assert_eq!(iter.next(), Some(Day(2022, 3)));
        assert_eq!(iter.next(), Some(Day(2022, 4)));
        assert_eq!(iter.next(), Some(Day(2022, 5)));
        assert_eq!(iter.next(), Some(Day(2022, 6)));
        assert_eq!(iter.next(), Some(Day(2022, 7)));
        assert_eq!(iter.next(), Some(Day(2022, 8)));
        assert_eq!(iter.next(), Some(Day(2022, 9)));
        assert_eq!(iter.next(), Some(Day(2022, 10)));
        assert_eq!(iter.next(), Some(Day(2022, 11)));
        assert_eq!(iter.next(), Some(Day(2022, 12)));
        assert_eq!(iter.next(), Some(Day(2022, 13)));
        assert_eq!(iter.next(), Some(Day(2022, 14)));
        assert_eq!(iter.next(), Some(Day(2022, 15)));
        assert_eq!(iter.next(), Some(Day(2022, 16)));
        assert_eq!(iter.next(), Some(Day(2022, 17)));
        assert_eq!(iter.next(), Some(Day(2022, 18)));
        assert_eq!(iter.next(), Some(Day(2022, 19)));
        assert_eq!(iter.next(), Some(Day(2022, 20)));
        assert_eq!(iter.next(), Some(Day(2022, 21)));
        assert_eq!(iter.next(), Some(Day(2022, 22)));
        assert_eq!(iter.next(), Some(Day(2022, 23)));
        assert_eq!(iter.next(), Some(Day(2022, 24)));
        assert_eq!(iter.next(), Some(Day(2022, 25)));
        assert_eq!(iter.next(), Some(Day(2023, 1)));
        assert_eq!(iter.next(), Some(Day(2023, 2)));
        assert_eq!(iter.next(), Some(Day(2023, 3)));
        assert_eq!(iter.next(), Some(Day(2023, 4)));
        assert_eq!(iter.next(), Some(Day(2023, 5)));
        assert_eq!(iter.next(), Some(Day(2023, 6)));
        assert_eq!(iter.next(), Some(Day(2023, 7)));
        assert_eq!(iter.next(), Some(Day(2023, 8)));
        assert_eq!(iter.next(), Some(Day(2023, 9)));
        assert_eq!(iter.next(), Some(Day(2023, 10)));
        assert_eq!(iter.next(), Some(Day(2023, 11)));
        assert_eq!(iter.next(), Some(Day(2023, 12)));
        assert_eq!(iter.next(), Some(Day(2023, 13)));
        assert_eq!(iter.next(), Some(Day(2023, 14)));
        assert_eq!(iter.next(), Some(Day(2023, 15)));
        assert_eq!(iter.next(), Some(Day(2023, 16)));
        assert_eq!(iter.next(), Some(Day(2023, 17)));
        assert_eq!(iter.next(), Some(Day(2023, 18)));
        assert_eq!(iter.next(), Some(Day(2023, 19)));
        assert_eq!(iter.next(), Some(Day(2023, 20)));
        assert_eq!(iter.next(), Some(Day(2023, 21)));
        assert_eq!(iter.next(), Some(Day(2023, 22)));
        assert_eq!(iter.next(), Some(Day(2023, 23)));
        assert_eq!(iter.next(), Some(Day(2023, 24)));
        assert_eq!(iter.next(), Some(Day(2023, 25)));
        assert_eq!(iter.next(), Some(Day(2024, 1)));
        assert_eq!(iter.next(), Some(Day(2024, 2)));
        assert_eq!(iter.next(), Some(Day(2024, 3)));
        assert_eq!(iter.next(), Some(Day(2024, 4)));
        assert_eq!(iter.next(), Some(Day(2024, 5)));
        assert_eq!(iter.next(), Some(Day(2024, 6)));
        assert_eq!(iter.next(), Some(Day(2024, 7)));
        assert_eq!(iter.next(), Some(Day(2024, 8)));
        assert_eq!(iter.next(), Some(Day(2024, 9)));
        assert_eq!(iter.next(), Some(Day(2024, 10)));
        assert_eq!(iter.next(), Some(Day(2024, 11)));
        assert_eq!(iter.next(), Some(Day(2024, 12)));
        assert_eq!(iter.next(), Some(Day(2024, 13)));
        assert_eq!(iter.next(), Some(Day(2024, 14)));
        assert_eq!(iter.next(), Some(Day(2024, 15)));
        assert_eq!(iter.next(), Some(Day(2024, 16)));
        assert_eq!(iter.next(), Some(Day(2024, 17)));
        assert_eq!(iter.next(), Some(Day(2024, 18)));
        assert_eq!(iter.next(), Some(Day(2024, 19)));
        assert_eq!(iter.next(), Some(Day(2024, 20)));
        assert_eq!(iter.next(), Some(Day(2024, 21)));
        assert_eq!(iter.next(), Some(Day(2024, 22)));
        assert_eq!(iter.next(), Some(Day(2024, 23)));
        assert_eq!(iter.next(), Some(Day(2024, 24)));
        assert_eq!(iter.next(), Some(Day(2024, 25)));
        assert_eq!(iter.next(), None);
    }
}

/* -------------------------------------------------------------------------- */
