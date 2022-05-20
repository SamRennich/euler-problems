/*
Problem 19: Counting Sundays

You are given the following information, but you may prefer to do
some research for yourself.

1 Jan 1900 was a Monday.
Thirty days has September,
April, June and November.
All the rest have thirty-one,
Saving February alone,
Which has twenty-eight, rain or shine.
And on leap years, twenty-nine.
A leap year occurs on any year evenly divisible by 4,
but not on a century unless it is divisible by 400.

How many Sundays fell on the first of the month during the twentieth century
(1 Jan 1901 to 31 Dec 2000)?

Answer: 171
*/

const START_YEAR: i32 = 1901;
const END_YEAR: i32 = 2000;
const LEAP_INTERVAL: i32 = 4;
const DAYS_IN_WEEK: i32 = 7;
const MONTHS_IN_YEAR: usize = 12;
const MONTH_LENGTHS: [i32; MONTHS_IN_YEAR] =
	[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const LEAP_MONTH_LENGTHS: [i32; MONTHS_IN_YEAR] =
	[31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const START_DAY: i32 = 2;

fn main() {
	let mut count = 0;
	let mut day = START_DAY;

	for year in START_YEAR..=END_YEAR {
		for month in 0..MONTH_LENGTHS.len() {
			if day % DAYS_IN_WEEK == 0 {
				count += 1;
			}

			if year % LEAP_INTERVAL == 0 {
				day += LEAP_MONTH_LENGTHS[month];
			} else {
				day += MONTH_LENGTHS[month];
			}
		}
	}

	println!("{}", count);
}
