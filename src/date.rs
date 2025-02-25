use std::cmp;
use std::fmt::{self, Display, Formatter, Pointer};
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::{DateTime, get_digit_unchecked};

use crate::error::Error as Error;

/// Log timestamp type.
///
/// Parse using `FromStr` impl.
/// Format using the `Display` trait.
/// Convert timestamp into/from `SytemTime` to use.
/// Supports comparsion and sorting.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Date {
    /// 1...31
    pub day: u8,
    /// 1...12
    pub mon: u8,
    /// 1970...9999
    pub year: u16,
}

impl Date{
    /// Parse a date from bytes, no check is performed for extract characters at the end of the string
    pub(crate) fn parse_bytes_partial(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() < 10 {
            return Err(Error::E("TooShort".to_string()));
        }
        let year: u16;
        let month: u8;
        let day: u8;
        unsafe {
            let y1 = get_digit_unchecked!(bytes, 0, "InvalidCharYear") as u16;
            let y2 = get_digit_unchecked!(bytes, 1, "InvalidCharYear") as u16;
            let y3 = get_digit_unchecked!(bytes, 2, "InvalidCharYear") as u16;
            let y4 = get_digit_unchecked!(bytes, 3, "InvalidCharYear") as u16;
            year = y1 * 1000 + y2 * 100 + y3 * 10 + y4;

            match bytes.get_unchecked(4) {
                b'-' => (),
                _ => return Err(Error::E("InvalidCharDateSep".to_string())),
            }

            let m1 = get_digit_unchecked!(bytes, 5, "InvalidCharMonth");
            let m2 = get_digit_unchecked!(bytes, 6, "InvalidCharMonth");
            month = m1 * 10 + m2;

            match bytes.get_unchecked(7) {
                b'-' => (),
                _ => return Err(Error::E("InvalidCharDateSep".to_string())),
            }

            let d1 = get_digit_unchecked!(bytes, 8, "InvalidCharDay");
            let d2 = get_digit_unchecked!(bytes, 9, "InvalidCharDay");
            day = d1 * 10 + d2;
        }

        // calculate the maximum number of days in the month, accounting for leap years in the
        // gregorian calendar
        let max_days = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => return Err(Error::E("OutOfRangeMonth".to_string())),
        };

        if day < 1 || day > max_days {
            return Err(Error::E("OutOfRangeDay".to_string()));
        }

        Ok(Self {
            day,
            mon: month,
            year
        })
    }
}

impl From<DateTime> for Date{
    fn from(arg: DateTime) -> Self {
        Date{
            day: arg.day,
            mon: arg.mon,
            year: arg.year
        }
    }
}

impl FromStr for Date {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //"0000-00-00";
        let d=Date::parse_bytes_partial(s.as_bytes())?;
        Ok(d)
    }
}

impl Display for Date{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buf: [u8; 10] = *b"0000-00-00";

        buf[0] = b'0' + (self.year / 1000) as u8;
        buf[1] = b'0' + (self.year / 100 % 10) as u8;
        buf[2] = b'0' + (self.year / 10 % 10) as u8;
        buf[3] = b'0' + (self.year % 10) as u8;

        buf[5] = b'0' + (self.mon / 10) as u8;
        buf[6] = b'0' + (self.mon % 10) as u8;

        buf[8] = b'0' + (self.day / 10) as u8;
        buf[9] = b'0' + (self.day % 10) as u8;
        f.write_str(std::str::from_utf8(&buf[..]).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::Date;

    #[test]
    fn test_date() {
        let d = Date::from_str("1234-12-13 11:12:13.123456").unwrap();
        println!("{}", d);
        assert_eq!("1234-12-13".to_string(), d.to_string());
    }
}
