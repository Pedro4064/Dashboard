#[repr(u8)]
#[derive(Debug)]
enum DayOfWeek {
    Monday = 0,
    Tuesday ,
    Wednesday,
    Thursday ,
    Friday ,
    Saturday ,
    Sunday ,
}

impl Into<u8> for DayOfWeek{
    fn into(self) -> u8{
        self as u8
    }

}

#[derive(Debug)]
enum Rule<T: Into<u8>> {
    All,                    // All is a simple *
    Range(T, T),          // A range is composed of 2 numerical values (e.g 0-15)
    Step(T),               // A step is composed of one number (e.g */3)
    Unit(T),               // A Unit is a simple numerical value
    RangedStep(T, T, u8), // A RangedStep is a concatenation of a Range rule and and step (e.g 0-10/2 i.e every two minutes from mimute 0 to minute 10)
}

#[derive(Debug)]
struct CronJob {
    minute: Vec<Rule<u8>>,
    hour: Vec<Rule<u8>>,
    day_of_month: Vec<Rule<u8>>,
    month: Vec<Rule<u8>>,
    day_of_week: Vec<Rule<DayOfWeek>>,
    command: String,
}

impl CronJob {
    fn new() -> Self {
        CronJob {
            minute: Vec::<Rule<u8>>::new(),
            hour: Vec::<Rule<u8>>::new(),
            day_of_week: Vec::<Rule<DayOfWeek>>::new(),
            month: Vec::<Rule<u8>>::new(),
            day_of_month: Vec::<Rule<u8>>::new(),
            command: String::new(),
        }
    }

    fn generate_command(&self) -> String {
        todo!()
    }

    fn generate_file(&self, file_name: &str) {}
}

trait RuleManipulation <T: Into<u8>> {
    fn add_rule(&mut self, new_rule: Rule<T>);
}

impl <T: Into<u8>> RuleManipulation<T> for Vec<Rule<T>> {
    fn add_rule(&mut self, new_rule: Rule<T>) {
        self.push(new_rule);
    }
}

fn main() {
    let mut anime = CronJob::new();
    anime.minute.add_rule(Rule::Range(0, 10)); // Add rule to run job for every minute from 0 to 10
    anime.hour.add_rule(Rule::RangedStep(0, 12, 2)); // Add rule to run job every two hours from hour 0 to hout 12
    anime.day_of_week.add_rule(Rule::RangedStep(DayOfWeek::Monday, DayOfWeek::Wednesday, 2)); // Add rule to run job every two hours from hour 0 to hout 12



    println!("{:#?}", anime);
}
