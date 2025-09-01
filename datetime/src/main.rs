/*
//THE NAIVE DATE STRUCT - has no notion of the actual time/ only the date
use chrono::NaiveDate; //it just model a date without considering any timezone

fn main() {
    let birthday = NaiveDate::from_ymd_opt(1991, 4, 12); //returns an option enum
    println!("{:#?}", birthday);

    let birthday = "1991-04-12";
    let birthday = birthday.parse::<NaiveDate>(); //returns a result enum
    println!("{:#?}", birthday);
}
 */
/*
/////////////////////////////////////////////////
////////////////////////////////////////////////
//THE TIMEDELTA STRUCT - models the idea of the duration
use chrono::TimeDelta;

fn main() {
    let five_seconds = TimeDelta::new(5, 0);
    println!("{:?}", five_seconds);

    let invalid = TimeDelta::new(5, 1_000_000_000); //b/c it i32 cannot contain this amount(1_000_000_000)
    println!("{:?}", invalid);

    let negative_five_seconds = TimeDelta::new(-5, 0);
    println!("{:?}", negative_five_seconds);

    let five_minutes = TimeDelta::minutes(-5); //convert the minutes to seconds
    println!("{}", five_minutes);

    let five_hours = TimeDelta::hours(5); //convert the hours to seconds
    println!("{}", five_hours);

    let five_days = TimeDelta::days(5); //convert the days to seconds
    println!("{}", five_days);

    let five_weeks = TimeDelta::weeks(5); //convert the weeks to seconds
    println!("{}", five_weeks);

    println!("{}", five_weeks.num_days());
    println!("{}", five_weeks.num_hours());
    println!("{}", five_weeks.num_minutes());

    let total_duration = five_weeks + five_days + five_hours + five_minutes;
    println!("{:?}", total_duration);
}
 */
/*
//////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////
//ADDING AND SUBTRACTING TIME
use chrono::{NaiveDate, TimeDelta};
use std::ops::{Add, Sub};

fn main(){
    let birthday = NaiveDate::from_ymd_opt(1991, 4, 12).unwrap();
    println!("{}", birthday.add(TimeDelta::days(5)));
    //OR, we can use the mathematical symbols
    println!("{}", birthday + TimeDelta::days(5));


    println!("{}", birthday.add(TimeDelta::weeks(2) + TimeDelta::days(5) ));

    println!("{}", birthday.sub(TimeDelta::weeks(3) ));

}
 */

use chrono::Local;

/*
/////////////////////////////////////
//////////////////////////////////////
//THE NAIVETIME AND NAIVEDATETIME STRUCTS
//   NAIVETIME STRUCT - has no notion of the actual date
//   NAIVEDATETIME - has notion for both date and time
use chrono::{NaiveDate, NaiveTime, NaiveDateTime, TimeDelta};

fn main() {
    let four_thirty_am = NaiveTime::from_hms_opt(4, 30, 0); //has no notion of the actual date
    println!("{:?}", four_thirty_am);

    let four_thirty_pm = NaiveTime::from_hms_opt(16, 30, 0); //has no notion of the actual date
    println!("{:?}", four_thirty_pm);

    let day = NaiveDate::from_ymd_opt(1969, 7,20).unwrap();
    let time = NaiveTime::from_hms_opt(20, 17,0).unwrap();
    let moon_landing = NaiveDateTime::new(day, time);
    println!("{}", moon_landing);

    println!("{}", moon_landing+ TimeDelta::days(1000)); //thousand days after the moon landing

}
 */
/*
//////////////////////////////////////////////////
//////////////////////////////////////////////////
//THE DATETIME STRUCT - combined date and time with time zone
//UTC - specifies the UTC time zone. it is most efficient (standardized timezone or a global standard or a reference time zone to which all the other times and time zones relate)
//Local - specifies the system local time zone (your computer)
use chrono::prelude::*;

fn main() {
    let system_time = Local::now();
    let utc_time = Utc::now();
    println!("{}", system_time.date_naive());
    println!("{}", utc_time.date_naive());

    println!("{}", system_time.time());
    println!("{}", utc_time.time());

    println!("{}", system_time.year());
    println!("{}", utc_time.year());

    println!("{}", system_time.month());
    println!("{}", utc_time.month());

    println!("{}", system_time.day());
    println!("{}", utc_time.day());

    println!("{}", system_time.hour());
    println!("{}", utc_time.hour());

    println!("{}", system_time.minute());
    println!("{}", utc_time.minute());

    println!("{}", system_time.second());
    println!("{}", utc_time.second());

    println!("{}", system_time.offset()); //relative to the UTC timezone
    println!("{}", utc_time.offset()); //itself
}
*/
/*
///////////////////////////////////////////////////////////
//////////////////////////////////////////////////////
//CONVERITNG TIMEZONES
use chrono::prelude::*;
use chrono_tz::Africa::Addis_Ababa;

fn main() {
    let local_time = Local::now();
    // let utc_time = local_time.with_timezone(&Utc);
    let addis_time = local_time.with_timezone(&Addis_Ababa);
    println!("{}", local_time);
    println!("{}", addis_time);
}
*/
/*
//////////////////////////////////////////////////////
//////////////////////////////////////////////////////
//THE PARSE_FROM_STR FUNCTION
use chrono::prelude::*;

fn main() {
    //-0600- means utc-6
    let someday = "31-Oct-1995 18:07:54 -0600";
    let dt = DateTime::parse_from_str(someday, "%d-%b-%Y %H:%M:%S %z");
    println!("{:?}", dt);
}
 */
/////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////
//THE FORMAT METHOD
use chrono::prelude::*;
fn main() {
    let utc_time = Utc::now();
    println!("{}", utc_time.format("%m-%d-%Y"));
    println!("{}", utc_time.format("%m/%d/%Y"));
    println!("{}", utc_time.format("%m-%d-%y"));
    println!("{}", utc_time.format("%b %d, %y %H:%M:%S"));
    println!("{}", utc_time.format("%A %I:%M %p"));
}
