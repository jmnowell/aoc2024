mod commons;

use crate::solutions::commons::*;

// Returns the number of "safe" reports
// from a given set of reports
pub fn part1(path: &str) -> usize {
    let report_sequence = read_lines(path);
    let mut safe_report_count: usize = 0;

    for data  in report_sequence {
        let report = parse_report(&data);

        let mut inc_dec_report: bool = true;

        // determine if we're increasing or decreasing
        // based on the first two digits of the sequence
        assert!(report.len() > 2);
        let ascending = report[1] > report[0];

        let mut last_digit = report[0];

        for idx in 1..report.len() {
            let current_digit = report[idx];

            if ascending {
                if last_digit < current_digit {
                    inc_dec_report = check_diff(last_digit, current_digit);

                    if !inc_dec_report {
                        break;
                    }
                } else {
                    inc_dec_report = false;
                    break;
                }
            } else {
                if last_digit > current_digit {
                    inc_dec_report = check_diff(last_digit, current_digit);

                    if !inc_dec_report {
                        break;
                    }
                } else {
                    inc_dec_report = false;
                    break;
                }
            }

            last_digit = current_digit;
        }

        if inc_dec_report {
            safe_report_count +=1;
        }
    }

    return safe_report_count;
}

pub fn part2(path: &str) -> usize {
    let report_sequence = read_lines(path);
    let mut safe_report_count: usize = 0;

    for data  in report_sequence {
        let report = parse_report(&data);
        let mut is_safe_report = true;
        let mut damper_fired = false;

        // determine if we're increasing or decreasing
        // based on the first two digits of the sequence
        assert!(report.len() > 2);
        let mut last_digit = report[0];
        let mut is_ascending = false;
        let mut first_run = true;

        for idx in 1..report.len() {
            // get the current number
            let current_digit = report[idx];

            if first_run {
                // on the first run, we only need to check
                // if the difference is the within bounds
                // we may be burning the dampener here
                if !check_diff(last_digit, current_digit) {
                    if damper_fired {
                        is_safe_report = false;
                        break;
                    }
                    
                    damper_fired = true;
                    //first_run = false;
                } else {
                    // difference was ok, save current to last
                    first_run = false;
                    is_ascending = current_digit > last_digit;
                    last_digit = current_digit;
                }
            } else {
                // outside of the first run
                // we need to see if we're ascending
                let ascending = current_digit > last_digit;

                if ascending == is_ascending {
                    // we're still headed in the same direction
                    if !check_diff(last_digit, current_digit) {
                        if damper_fired {
                            is_safe_report = false;
                            break;
                        }

                        damper_fired = true;
                    } else {
                        last_digit = current_digit;
                    }
                } else {
                    // we've changed direction, so we need to 
                    // check the damper
                    if damper_fired {
                        is_safe_report = false;
                        break;
                    }

                    damper_fired = true;
                }
            }
        }

        if is_safe_report {
            safe_report_count += 1;
        }
    }

    return safe_report_count;
}