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

type RuleValueType = u8;
trait RuleValue {
    fn to_numerical_value(&self) -> RuleValueType;
}

impl RuleValue for DayOfWeek {
    fn to_numerical_value(&self) -> RuleValueType {
        *self as RuleValueType
    }
}

impl RuleValue for u8 {
    fn to_numerical_value(&self) -> RuleValueType {
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
    minute: Vec<Rule<RuleValueType>>,
    hour: Vec<Rule<RuleValueType>>,
    day_of_month: Vec<Rule<RuleValueType>>,
    month: Vec<Rule<RuleValueType>>,
    day_of_week: Vec<Rule<DayOfWeek>>,
    command: String,
}

impl CronJob {
    fn new() -> Self {
        CronJob {
            minute: Vec::<Rule<RuleValueType>>::new(),
            hour: Vec::<Rule<RuleValueType>>::new(),
            day_of_week: Vec::<Rule<DayOfWeek>>::new(),
            month: Vec::<Rule<RuleValueType>>::new(),
            day_of_month: Vec::<Rule<RuleValueType>>::new(),
            command: String::new(),
        }
    }

    fn generate_command(&self) -> String {
        // The formatted crontab description command
        let mut command = String::new();

        // Specify the validation rule for each field
        let minute_validator = |value: RuleValueType| value > 0 && value < 60;
        let hour_validator = |value: RuleValueType| value > 0 && value < 24;
        let day_of_week_validator = |value: RuleValueType| value > 0 && value < 7;
        let month_validator = |value: RuleValueType| value >= 1 && value <= 12;
        let day_of_month_validator = |value: RuleValueType| value >= 1 && value <= 31;

        for rule in &self.minute {
            command.push_str(&CronJob::parse_rule_entry(rule, minute_validator));
            command.push(',');
        }
        for rule in &self.day_of_week {
            CronJob::parse_rule_entry(rule, |_| true);
        }
        command
    }

    fn parse_rule_entry<T: RuleValue, F: Fn(RuleValueType) -> bool>(
        rule: &Rule<T>,
        validator: F,
    ) -> String {
        match rule {
            Rule::All => String::from("*"),
            Rule::Range(l_bound, h_bound) => format!(
                "{:?}-{:?}",
                l_bound.to_numerical_value(),
                h_bound.to_numerical_value()
            )
            ,
            Rule::Step(step) => format!("*/{}", step.to_numerical_value()),
            Rule::Unit(value) => format!("{}", value.to_numerical_value()),
            Rule::RangedStep(l_limit, h_limit, step) => format!(
                "{}-{}/{}",
                l_limit.to_numerical_value(),
                h_limit.to_numerical_value(),
                step
            ),
        } 
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
