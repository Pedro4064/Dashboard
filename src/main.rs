enum DayOfWeek {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6,
}

struct CronJob {
    minute: Option<u8>,
    hour: Option<u8>,
    day_of_month: Option<u8>,
    month: Option<u8>,
    day_of_week: Option<DayOfWeek>,
    command: Option<String>,
}

impl CronJob {
    fn new() -> Self {
        CronJob {
            minute: None,
            hour: None,
            day_of_week: None,
            month: None,
            day_of_month: None,
            command: None,
        }
    }

    fn generate_command(&self) -> String {
        // The format for a cron job is: minute hour day_of_month month day_of_week command_to_run
        // An imprtant thing to keep in mind is that if not specified it should be placed a *
        
        // Parse the input from u8 to String representation, * if None
        let parse_input =
            |input: Option<u8>| input.map_or_else(|| "*".to_string(), |value| value.to_string());

        // Now we can popualte the command string with the structs fileds
       format!("{} {} {} {} {} {}", parse_input(self.minute), parse_input(self.hour), parse_input(self.day_of_month), parse_input(self.month), "" , self.command.as_ref().unwrap() )
    }

    fn generate_file(&self, fiel_name: &str) {}
}

fn main() {
    let mut job = CronJob::new();
    job.command = Some("python3 potato.py".to_string());

    println!("{}", job.generate_command());
}
