/*
Problem 17: Number Letter Counts

If the numbers 1 to 5 are written out in words: one, two, three, four, five,
then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
If all the numbers from 1 to 1000 (one thousand) inclusive were
written out in words, how many letters would be used?
NOTE: Do not count spaces or hyphens.
For example, 342 (three hundred and forty-two) contains 23 letters and 115
(one hundred and fifteen) contains 20 letters.
The use of "and" when writing out numbers is in compliance with British usage.

Answer: 21124
*/

const RANGE: i32 = 1000;

fn main() {
	let mut sum = 0;

	for num in 1..=RANGE {
		let ones = num % 10;
		let tens = (num / 10) % 10;
		let hundreds = (num / 100) % 10;
		let thousands = (num / 1000) % 10;

		if tens != 1 {
			if ones == 1 {
				sum += "one".chars().count();
			} else if ones == 2 {
				sum += "two".chars().count();
			} else if ones == 3 {
				sum += "three".chars().count();
			} else if ones == 4 {
				sum += "four".chars().count();
			} else if ones == 5 {
				sum += "five".chars().count();
			} else if ones == 6 {
				sum += "six".chars().count();
			} else if ones == 7 {
				sum += "seven".chars().count();
			} else if ones == 8 {
				sum += "eight".chars().count();
			} else if ones == 9 {
				sum += "nine".chars().count();
			}

			if tens == 2 {
				sum += "twenty".chars().count();
			} else if tens == 3 {
				sum += "thirty".chars().count();
			} else if tens == 4 {
				sum += "forty".chars().count();
			} else if tens == 5 {
				sum += "fifty".chars().count();
			} else if tens == 6 {
				sum += "sixty".chars().count();
			} else if tens == 7 {
				sum += "seventy".chars().count();
			} else if tens == 8 {
				sum += "eighty".chars().count();
			} else if tens == 9 {
				sum += "ninety".chars().count();
			}
		} else {
			if ones == 0 {
				sum += "ten".chars().count();
			} else if ones == 1 {
				sum += "eleven".chars().count();
			} else if ones == 2 {
				sum += "twelve".chars().count();
			} else if ones == 3 {
				sum += "thirteen".chars().count();
			} else if ones == 4 {
				sum += "fourteen".chars().count();
			} else if ones == 5 {
				sum += "fifteen".chars().count();
			} else if ones == 6 {
				sum += "sixteen".chars().count();
			} else if ones == 7 {
				sum += "seventeen".chars().count();
			} else if ones == 8 {
				sum += "eighteen".chars().count();
			} else if ones == 9 {
				sum += "nineteen".chars().count();
			}
		}

		if hundreds == 1 {
			sum += "onehundred".chars().count();
		} else if hundreds == 2 {
			sum += "twohundred".chars().count();
		} else if hundreds == 3 {
			sum += "threehundred".chars().count();
		} else if hundreds == 4 {
			sum += "fourhundred".chars().count();
		} else if hundreds == 5 {
			sum += "fivehundred".chars().count();
		} else if hundreds == 6 {
			sum += "sixhundred".chars().count();
		} else if hundreds == 7 {
			sum += "sevenhundred".chars().count();
		} else if hundreds == 8 {
			sum += "eighthundred".chars().count();
		} else if hundreds == 9 {
			sum += "ninehundred".chars().count();
		}

		if thousands == 1 {
			sum += "onethousand".chars().count();
		} else if thousands == 2 {
			sum += "twothousand".chars().count();
		} else if thousands == 3 {
			sum += "threethousand".chars().count();
		} else if thousands == 4 {
			sum += "fourthousand".chars().count();
		} else if thousands == 5 {
			sum += "fivethousand".chars().count();
		} else if thousands == 6 {
			sum += "sixthousand".chars().count();
		} else if thousands == 7 {
			sum += "seventhousand".chars().count();
		} else if thousands == 8 {
			sum += "eightthousand".chars().count();
		} else if thousands == 9 {
			sum += "ninethousand".chars().count();
		}

		if (ones > 0 || tens > 0) && (hundreds > 0 || thousands > 0) {
			sum += "and".chars().count();
		}
	}

	println!("{}", sum);
}
