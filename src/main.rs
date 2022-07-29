#[repr(u8)]
#[derive(Debug)]
enum DayOfWeek {
    Monday = 0,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

trait RuleValue{
    fn to_numerical_value(&self) -> u8;
}

impl RuleValue for DayOfWeek{
    fn to_numerical_value(&self) -> u8 {
        *self as u8
    }
}

impl RuleValue for u8{
    fn to_numerical_value(&self) -> u8{
        *self
    }
}


#[derive(Debug)]
enum Rule<T: RuleValue> {
    All,                  // All is a simple *
    Range(T, T),          // A range is composed of 2 numerical values (e.g 0-15)
    Step(T),              // A step is composed of one number (e.g */3)
    Unit(T),              // A Unit is a simple numerical value
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
        for rule in &self.minute {
            CronJob::parse_rule_entry(rule, |_| true);
        }
        for rule in &self.day_of_week {
            CronJob::parse_rule_entry(rule, |_| true);
        }
         "".to_string()
    }

    fn parse_rule_entry<T: RuleValue, F: Fn(Rule<T>) -> bool>(
        rule: &Rule<T>,
        validator: F,
    ) -> String {
        match rule {
            Rule::All => println!("All Rule"),
            Rule::Range(x, y) => {
                let x_value: u8 = x.to_numerical_value();
                let y_value: u8 = y.to_numerical_value();
                println!("Range Rule: {:?}-{:?}", x_value, y_value);
            }
            Rule::Step(step) => println!("Step Rule: */{:?}", step.to_numerical_value()),
            Rule::Unit(value) => println!("Unit Rule: {:?}", value.to_numerical_value()),
            Rule::RangedStep(l_limit, h_limit, step) => println!(
                "RangedStep Rule: {:?}-{:?}/{}",
                l_limit.to_numerical_value(),
                h_limit.to_numerical_value(),
                step
            ),
        }
        "".to_string()
    }

    fn generate_file(&self, file_name: &str) {}
}

trait RuleManipulation<T: RuleValue> {
    fn add_rule(&mut self, new_rule: Rule<T>);
}

impl<T: RuleValue> RuleManipulation<T> for Vec<Rule<T>> {
    fn add_rule(&mut self, new_rule: Rule<T>) {
        self.push(new_rule);
    }
}

fn main() {
    let mut anime = CronJob::new();
    anime.minute.add_rule(Rule::Range(0, 10)); // Add rule to run job for every minute from 0 to 10
    anime.minute.add_rule(Rule::RangedStep(0, 12, 2)); // Add rule to run job every two hours from hour 0 to hout 12
    anime
        .day_of_week
        .add_rule(Rule::RangedStep(DayOfWeek::Monday, DayOfWeek::Wednesday, 2)); // Add rule to run job every two hours from hour 0 to hout 12

    anime.generate_command();
    println!("{:#?}", anime);
}
