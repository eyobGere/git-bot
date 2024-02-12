use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::str::FromStr;

fn main() {
    let daily_commits = daily_commit_amount_randomizer();
    let random_increments = commit_date_randomizer();
    let (day, month, file_path) = user_inputs();
    function_that_does_the_job(daily_commits, random_increments, day, month, &file_path);
}

fn daily_commit_amount_randomizer() -> i32 {
    let daily_commits = rand::thread_rng().gen_range(1..=4);
    println!("{}", daily_commits);
    daily_commits
}

fn commit_date_randomizer() -> i32 {
    let random_increments = rand::thread_rng().gen_range(1..=6);
    random_increments
}

fn function_that_does_the_job(daily_commits: i32, _random_increments: i32, day: String, month: String, file_path: &str) {
    for _ in 0..daily_commits {
        // Read file lines
        let lines = match read_file_lines(file_path) {
            Ok(lines) => lines,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return;
            }
        };

        // Select random segment
        if let Some(segment) = select_random_segment(&lines, 10, 80) {
            // Create a new file with the selected segment
            let new_file_path = "generated_file.txt";
            if let Err(e) = write_segment_to_file(new_file_path, &segment) {
                eprintln!("Error writing to new file: {}", e);
                return;
            }

            // Stage the changes
            if !run_command("git", &["add", new_file_path]) {
                eprintln!("Failed to stage the file");
                return;
            }

            // Commit the changes with a random message
            let commit_message: String = thread_rng().sample_iter(&Alphanumeric).take(9).map(char::from).collect();
            if !run_command("git", &["commit", "-m", &commit_message]) {
                eprintln!("Failed to commit the changes");
                return;
            }

            // Generate the amended date
            let random_hour = rand::thread_rng().gen_range(0..=23);
            let random_minute = rand::thread_rng().gen_range(0..=59);
            let random_second = rand::thread_rng().gen_range(0..=59);
            let random_day = rand::thread_rng().gen_range(1..=31);
            let commit_date = format!(
                "{} {} {} {:02}:{:02}:{:02} 2024",
                day,
                month,
                random_day,
                random_hour,
                random_minute,
                random_second
            );

            // Amend the commit with the new date
            if !run_command("git", &["commit", "--amend", "--no-edit", "--date", &commit_date]) {
                eprintln!("Failed to amend the commit");
                return;
            }

            // Push the changes
            if !run_command("git", &["push","origin", "main", "--force"]) {
                eprintln!("Failed to push the changes");
                return;
            }
        } else {
            eprintln!("Not enough lines to select a valid segment.");
            return;
        }
    }
}

fn read_file_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn select_random_segment<'a>(lines: &'a [String], min_gap: usize, max_gap: usize) -> Option<Vec<&'a String>> {
    let total_lines = lines.len();

    if total_lines < max_gap {
        return None;
    }

    let mut rng = rand::thread_rng();
    let start_index = rng.gen_range(0..total_lines - min_gap);
    let end_index = rng.gen_range((start_index + min_gap).min(total_lines - 1)..=(start_index + max_gap).min(total_lines - 1));

    let segment = lines[start_index..=end_index].iter().collect();
    Some(segment)
}

fn write_segment_to_file(path: &str, segment: &[&String]) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
    for line in segment {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

fn run_command(command: &str, args: &[&str]) -> bool {
    let status = Command::new(command)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute command");

    status.success()
}

fn user_inputs() -> (String, String, String) {
    #[derive(Debug)]
    enum Date {
        Mon,
        Tue,
        Wed,
        Thu,
        Fri,
        Sat,
        Sun,
    }

    #[derive(Debug)]
    enum Month {
        Jan,
        Feb,
        Mar,
        Apr,
        May,
        Jun,
        Jul,
        Aug,
        Sep,
        Oct,
        Nov,
        Dec,
    }

    impl FromStr for Date {
        type Err = ();

        fn from_str(input: &str) -> Result<Date, Self::Err> {
            match input {
                "Mon" => Ok(Date::Mon),
                "Tue" => Ok(Date::Tue),
                "Wed" => Ok(Date::Wed),
                "Thu" => Ok(Date::Thu),
                "Fri" => Ok(Date::Fri),
                "Sat" => Ok(Date::Sat),
                "Sun" => Ok(Date::Sun),
                _ => Err(()),
            }
        }
    }

    impl FromStr for Month {
        type Err = ();

        fn from_str(input: &str) -> Result<Month, Self::Err> {
            match input {
                "Jan" => Ok(Month::Jan),
                "Feb" => Ok(Month::Feb),
                "Mar" => Ok(Month::Mar),
                "Apr" => Ok(Month::Apr),
                "May" => Ok(Month::May),
                "Jun" => Ok(Month::Jun),
                "Jul" => Ok(Month::Jul),
                "Aug" => Ok(Month::Aug),
                "Sep" => Ok(Month::Sep),
                "Oct" => Ok(Month::Oct),
                "Nov" => Ok(Month::Nov),
                "Dec" => Ok(Month::Dec),
                _ => Err(()),
            }
        }
    }

    println!("Enter a day where you want the commit to start from [Mon, Tue, Wed...etc] in this exact format");
    let mut day = String::new();
    io::stdin()
        .read_line(&mut day)
        .expect("Failed to read line for day");
    let day = day.trim().to_string();

    println!("Enter a month where you want the commit to start from [Jan, Feb, Mar...etc] in this exact format");
    let mut month = String::new();
    io::stdin()
        .read_line(&mut month)
        .expect("Failed to read line for month");
    let month = month.trim().to_string();

    println!("Enter the file path you want to read from:");
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read line for file path");
    let file_path = file_path.trim().to_string();

    (day, month, file_path)
}

