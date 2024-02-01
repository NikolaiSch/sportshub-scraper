use anyhow::Error;
use chrono::{Datelike, TimeZone, Utc};

/// Parses a date string into a parsable format
/// an example input is "1 February at 0:00"
///
/// # Arguments
/// * `date` - The date string to parse
///
/// # Example
/// ```
/// use scraper::date_parser::date_parser;
///
/// let date = "1 February at 0:00";
/// let parsed_date = date_parser(date);
///
/// assert_eq!(parsed_date, "1-2-0-00".to_string());
/// ```
pub fn date_parser(date: &str) -> String {
    let date = date.trim();
    let date = date.replace(" at ", " ");
    let date = date.replace("January", "1");
    let date = date.replace("February", "2");
    let date = date.replace("March", "3");
    let date = date.replace("April", "4");
    let date = date.replace("May", "5");
    let date = date.replace("June", "6");
    let date = date.replace("July", "7");
    let date = date.replace("August", "8");
    let date = date.replace("September", "9");
    let date = date.replace("October", "10");
    let date = date.replace("November", "11");
    let date = date.replace("December", "12");
    let date = date.replace(":", " ");
    let date = date.replace(" ", "-");
    let date = date.replace("th", "");
    let date = date.replace("st", "");
    let date = date.replace("nd", "");
    let date = date.replace("rd", "");

    date
}

/// Parses a date string into a i64 timestamp
/// an example input is "1-2-0-00"
///
/// # Arguments
/// * `date` - The date string to parse
/// * `year` - The year to use
///
/// # Example
/// ```
/// use std::time;
///
/// use scraper::date_parser::date_parser_to_instant;
///
/// let date = "1-2-0-00";
/// let year = 2024;
/// let parsed_date = date_parser_to_instant(date, year).unwrap();
///
/// assert_eq!(parsed_date, 1_706_745_600);
/// ```
pub fn date_parser_to_instant(date: &str, year: i32) -> Result<i64, Error> {
    let date = date.split("-").collect::<Vec<&str>>();
    let day = date[0].parse::<u32>().unwrap();
    let month = date[1].parse::<u32>().unwrap();
    let hour = date[2].parse::<u32>().unwrap();
    let minute = date[3].parse::<u32>().unwrap();

    let mut year = year;
    if day == 1 && month == 1 && hour == 1 && minute == 1 {
        year = 1990;
    }

    let t = Utc.with_ymd_and_hms(year, month, day, hour, minute, 0);

    if let chrono::LocalResult::Single(time) = t {
        return Ok(time.timestamp());
    }

    Err(anyhow::anyhow!("Could not parse date"))
}

/// Checks closest year to the current timestamp
/// and returns the timestamp of the date
/// an example input is "1-2-0-00"
///
/// # Arguments
/// * `date` - The date string to parse
///
/// # Example
/// ```
/// use scraper::date_parser::date_parser_with_closest_year;
///
/// let date = "1-2-0-00";
/// let parsed_date = date_parser_with_closest_year(date).unwrap();
///
/// assert_eq!(parsed_date, 1_706_745_600);
/// // This will be different depending on the current year
///
/// let date = "12-12-0-00";
/// let parsed_date = date_parser_with_closest_year(date).unwrap();
///
/// assert_eq!(parsed_date, 1_702_339_200);
/// ```
pub fn date_parser_with_closest_year(date: &str) -> anyhow::Result<i64> {
    let now = Utc::now();
    let year = now.year();
    let date_this_year = date_parser_to_instant(date, year)?;
    let date_next_year = date_parser_to_instant(date, year + 1)?;
    let date_last_year = date_parser_to_instant(date, year - 1)?;

    let now = now.timestamp();

    let diff = (now - date_this_year).abs();
    let diff_next_year = (now - date_next_year).abs();
    let diff_last_year = (now - date_last_year).abs();

    if diff < diff_next_year && diff < diff_last_year {
        Ok(date_this_year)
    } else if diff_next_year < diff && diff_next_year < diff_last_year {
        Ok(date_next_year)
    } else {
        Ok(date_last_year)
    }
}


/// Parses a date string into a i32 seconds since epoch
/// an example input is "1 February at 0:00"
/// This function is used to parse the date string from the website
/// into a timestamp that can be used to compare with the current time
/// and check if the stream is live or not
/// # Arguments
/// * `date` - The date string to parse
///
/// # Example
/// ```
/// use scraper::date_parser::date_string_to_timestamp;
///
/// let date = "1 February at 0:00";
/// let parsed_date = date_string_to_timestamp(date).unwrap();
///
/// assert_eq!(parsed_date, 1_706_745_600);
/// ```
pub fn date_string_to_timestamp(date: &str) -> anyhow::Result<i64> {
    let date = date_parser(date);
    let date = date_parser_with_closest_year(&date)?;

    Ok(date)
}


#[cfg(test)]
mod test {
    mod test_date_parser {
        use super::super::date_parser;

        #[test]
        fn test_1st_jan() {
            let date = "1st January at 0:00";
            let parsed_date = date_parser(date);

            assert_eq!(parsed_date, "1-1-0-00".to_string());
        }

        #[test]
        fn test_2nd_feb() {
            let date = "2nd February at 0:00";
            let parsed_date = date_parser(date);

            assert_eq!(parsed_date, "2-2-0-00".to_string());
        }

        #[test]
        fn test_3rd_mar() {
            let date = "3rd March at 0:00";
            let parsed_date = date_parser(date);

            assert_eq!(parsed_date, "3-3-0-00".to_string());
        }

        #[test]
        fn test_4th_apr() {
            let date = "4th April at 0:00";
            let parsed_date = date_parser(date);

            assert_eq!(parsed_date, "4-4-0-00".to_string());
        }

        #[test]
        fn test_random_date_1() {
            let date = "1st April at 3:45";
            let parsed_date = date_parser(date);

            assert_eq!(parsed_date, "1-4-3-45".to_string());
        }

        #[test]
        fn test_random_date_2() {
            let date = "2nd August at 8:15";
            let parsed_date = date_parser(date);

            assert_eq!(parsed_date, "2-8-8-15".to_string());
        }

        #[test]
        fn test_random_date_3() {
            let date = "3rd December at 12:00";
            let parsed_date = date_parser(date);

            assert_eq!(parsed_date, "3-12-12-00".to_string());
        }

        #[test]
        fn test_random_date_4() {
            let date = "4th May at 23:59";
            let parsed_date = date_parser(date);

            assert_eq!(parsed_date, "4-5-23-59".to_string());
        }

        #[test]
        fn test_random_date_5() {
            let date = "5th June at 23:59";
            let parsed_date = date_parser(date);

            assert_eq!(parsed_date, "5-6-23-59".to_string());
        }
    }

    mod test_timestamp {
        use super::super::date_parser_to_instant;

        #[test]
        fn test_1st_jan() {
            let date = "1-1-0-00";
            let year = 2024;
            let parsed_date = date_parser_to_instant(date, year).unwrap();

            assert_eq!(parsed_date, 1704067200);
        }

        #[test]
        fn test_2nd_feb() {
            let date = "2-2-0-00";
            let year = 2024;
            let parsed_date = date_parser_to_instant(date, year).unwrap();

            assert_eq!(parsed_date, 1706832000);
        }

        #[test]
        fn test_3rd_mar() {
            let date = "3-3-0-00";
            let year = 2024;
            let parsed_date = date_parser_to_instant(date, year).unwrap();

            assert_eq!(parsed_date, 1709424000);
        }

        #[test]
        fn test_4th_apr() {
            let date = "4-4-0-00";
            let year = 2024;
            let parsed_date = date_parser_to_instant(date, year).unwrap();

            assert_eq!(parsed_date, 1712188800);
        }

        #[test]
        fn test_random_date_1() {
            let date = "1-4-3-45";
            let year = 2024;
            let parsed_date = date_parser_to_instant(date, year).unwrap();

            assert_eq!(parsed_date, 1711943100);
        }

        #[test]
        fn test_random_date_2() {
            let date = "2-8-8-15";
            let year = 2024;
            let parsed_date = date_parser_to_instant(date, year).unwrap();

            assert_eq!(parsed_date, 1722586500);
        }

        #[test]
        fn test_random_date_3() {
            let date = "3-12-12-00";
            let year = 2024;
            let parsed_date = date_parser_to_instant(date, year).unwrap();

            assert_eq!(parsed_date, 1733227200);
        }

        #[test]
        fn test_random_date_4() {
            let date = "4-5-23-59";
            let year = 2024;
            let parsed_date = date_parser_to_instant(date, year).unwrap();

            assert_eq!(parsed_date, 1714867140);
        }

        #[test]
        fn test_random_date_5() {
            let date = "5-6-23-59";
            let year = 2024;
            let parsed_date = date_parser_to_instant(date, year).unwrap();

            assert_eq!(parsed_date, 1717631940);
        }
    }

    mod test_closest_year {
        use super::super::date_parser_with_closest_year;

        #[test]
        fn test_1st_jan() {
            let date = "1-1-0-00";
            let parsed_date = date_parser_with_closest_year(date).unwrap();

            assert_eq!(parsed_date, 1704067200);
        }

        #[test]
        fn test_2nd_feb() {
            let date = "2-2-0-00";
            let parsed_date = date_parser_with_closest_year(date).unwrap();

            assert_eq!(parsed_date, 1706832000);
        }

        #[test]
        fn test_3rd_mar() {
            let date = "3-3-0-00";
            let parsed_date = date_parser_with_closest_year(date).unwrap();

            assert_eq!(parsed_date, 1709424000);
        }

        #[test]
        fn test_4th_apr() {
            let date = "4-4-0-00";
            let parsed_date = date_parser_with_closest_year(date).unwrap();

            assert_eq!(parsed_date, 1712188800);
        }

        #[test]
        fn test_random_date_1() {
            let date = "1-4-3-45";
            let parsed_date = date_parser_with_closest_year(date).unwrap();

            assert_eq!(parsed_date, 1711943100);
        }

        #[test]
        fn test_random_date_2() {
            let date = "2-8-8-15";
            let parsed_date = date_parser_with_closest_year(date).unwrap();

            assert_eq!(parsed_date, 1722586500);
        }

        #[test]
        fn test_random_date_3() {
            let date = "3-12-12-00";
            let parsed_date = date_parser_with_closest_year(date).unwrap();

            assert_eq!(parsed_date, 1701604800);
        }

        #[test]
        fn test_random_date_4() {
            let date = "4-5-23-59";
            let parsed_date = date_parser_with_closest_year(date).unwrap();

            assert_eq!(parsed_date, 1714867140);
        }

        #[test]
        fn test_random_date_5() {
            let date = "5-6-23-59";
            let parsed_date = date_parser_with_closest_year(date).unwrap();

            assert_eq!(parsed_date, 1717631940);
        }
    }

    mod test_combined_parsing {
        #[test]
        fn test_1st_jan() {
            let date = "1st January at 0:00";
            let parsed_date = super::super::date_string_to_timestamp(date).unwrap();

            assert_eq!(parsed_date, 1704067200);
        }

        #[test]
        fn test_2nd_feb() {
            let date = "2nd February at 0:00";
            let parsed_date = super::super::date_string_to_timestamp(date).unwrap();

            assert_eq!(parsed_date, 1706832000);
        }

        #[test]
        fn test_3rd_mar() {
            let date = "3rd March at 0:00";
            let parsed_date = super::super::date_string_to_timestamp(date).unwrap();

            assert_eq!(parsed_date, 1709424000);
        }

        #[test]
        fn test_4th_apr() {
            let date = "4th April at 0:00";
            let parsed_date = super::super::date_string_to_timestamp(date).unwrap();

            assert_eq!(parsed_date, 1712188800);
        }

        #[test]
        fn test_random_date_1() {
            let date = "1st April at 3:45";
            let parsed_date = super::super::date_string_to_timestamp(date).unwrap();

            assert_eq!(parsed_date, 1711943100);
        }

        #[test]
        fn test_random_date_2() {
            let date = "2nd August at 8:15";
            let parsed_date = super::super::date_string_to_timestamp(date).unwrap();

            assert_eq!(parsed_date, 1722586500);
        }

        #[test]
        fn test_random_date_3() {
            let date = "3rd December at 12:00";
            let parsed_date = super::super::date_string_to_timestamp(date).unwrap();

            assert_eq!(parsed_date, 1701604800);
        }

        #[test]
        fn test_random_date_4() {
            let date = "4th May at 23:59";
            let parsed_date = super::super::date_string_to_timestamp(date).unwrap();

            assert_eq!(parsed_date, 1714867140);
        }

        #[test]
        fn test_random_date_5() {
            let date = "5th June at 23:59";
            let parsed_date = super::super::date_string_to_timestamp(date).unwrap();

            assert_eq!(parsed_date, 1717631940);
        }
    }
}
