fn parse_report(report: &str) -> Vec<i32> {
    // split by space
    report.split_whitespace()
        // parse each element to i32
        .map(|x| x.parse().unwrap())
        // collect to Vec<i32>
        .collect()
}

enum Bool3 {
    True,
    False,
    Unknown,
}

fn report_is_safe(report: &Vec<i32>) -> bool {
    let mut prev = report[0];
    let mut increasing = Bool3::Unknown;

    for i in 1..report.len() {
        let current = report[i];

        if (current-prev).abs() > 3 || current == prev {
            return false;
        }

        match increasing {
            Bool3::Unknown => {
                if current > prev {
                    increasing = Bool3::True;
                } else {
                    increasing = Bool3::False;
                }
            },
            Bool3::True => {
                if current < prev {
                    return false;
                }
            },
            Bool3::False => {
                if current > prev {
                    return false;
                }
            },
        }

        prev = current;
    }

    return true;
}

fn report_is_safe_by_removing(report: &Vec<i32>) -> bool {
    if report_is_safe(&report) {
        return true;
    }

    for i in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(i);

        if report_is_safe(&new_report) {
            return true;
        }
    }

    return false;
}

fn main() {
    // read from input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    // split by new line
    let lines = input.lines();

    // collect reports
    let reports: Vec<Vec<i32>> = lines.map(|line| parse_report(line)).collect();

    let mut safe_reports = 0;
    // iterate over reports
    for report in reports {
        if report_is_safe_by_removing(&report) {
            safe_reports += 1;
        }
    }

    println!("{}", safe_reports);
}
