use core::fmt;

use chrono::NaiveDate;
use headless_chrome::Browser;

#[derive(Debug)]
struct TrumpGolfTrack {
    days_in_office: u32,
    days_spent_golfing: u32,
    time_spent_golfing: f32,
    since: NaiveDate,
    days: Vec<NaiveDate>,
}

impl TrumpGolfTrack {
    fn fetch() -> Result<Self, Box<dyn std::error::Error>> {
        let browser = Browser::default()?;

        let tab = browser.new_tab()?;
        tab.navigate_to("https://trumpgolftrack.com")?;

        let days_in_office = tab
            .wait_for_element("main .grid div:nth-child(1) > p:nth-child(2)")?
            .get_inner_text()?
            .parse()?;

        let days_spent_golfing = tab
            .find_element("main .grid div:nth-child(2) > p:nth-child(2)")?
            .get_inner_text()?
            .parse()?;

        let time_spent_golfing = tab
            .find_element("main .grid div:nth-child(3) > p:nth-child(2)")?
            .get_inner_text()?
            .trim_end_matches("%")
            .parse()?;

        let since = tab
            .find_element("main .grid div:nth-child(1) > p:nth-child(3)")?
            .get_inner_text()?;

        let since = NaiveDate::parse_from_str(since.trim_start_matches("Since "), "%B %d, %Y")?;

        let days = {
            let mut days = tab
                .find_elements("main .container:nth-child(2) ul li p:nth-child(1)")?
                .into_iter()
                .map(|element| element.get_inner_text())
                .flatten()
                .map(|date| NaiveDate::parse_from_str(&date, "%m/%d/%Y"))
                .collect::<Result<Vec<_>, _>>()?;

            days.sort();
            days
        };

        Ok(TrumpGolfTrack {
            days_in_office,
            days_spent_golfing,
            time_spent_golfing,
            since,
            days,
        })
    }
}

impl fmt::Display for TrumpGolfTrack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            writeln!(f, "Trump has been in office since {} ({} days).", self.since, self.days_in_office)?;
            writeln!(f, "He has spent {} days golfing.", self.days_spent_golfing)?;
            writeln!(f, "He has spent {}% of his presidency golfing.", self.time_spent_golfing)?;
            writeln!(f, "He has golfed on the following days:")?;
            for day in &self.days {
                writeln!(f, "\t{}", day)?;
            }

            return Ok(());
        } else {
            write!(
                f,
                "Trump has been in office for {} days since {} and spent {} days golfing, which is {}% of his presidency.",
                self.days_in_office,
                self.since,
                self.days_spent_golfing,
                self.time_spent_golfing,
            )
        }
    }
}

fn main() {
    let trump_golf_track = TrumpGolfTrack::fetch().unwrap();
    println!("{:#}", trump_golf_track);
}
