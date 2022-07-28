enum DayOfWeek {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6,
}

//Basically every numerical entry on a cronjob can have *(for any) ,(for discrete separation) -(for
//range) /(for step value)
#[derive(Debug)]
enum Rule {
    All,           // All is a simple *
    Range(u8, u8), // A range is composed of 2 numerical values (e.g 0-15)
    Step(u8),      // A step is composed of one number (e.g */3)
    Unit(u8),      // A Unit is a simple numerical value
}

#[derive(Debug)]
struct CronJob {
    minute: Vec<Rule>,
    hour: Vec<Rule>,
    day_of_month: Vec<Rule>,
    month: Vec<Rule>,
    day_of_week: Vec<Rule>,
    command: String,
}

impl CronJob {
    fn new() -> Self {
        CronJob {
            minute: Vec::<Rule>::new(),
            hour: Vec::<Rule>::new(),
            day_of_week: Vec::<Rule>::new(),
            month: Vec::<Rule>::new(),
            day_of_month: Vec::<Rule>::new(),
            command: String::new(),
        }
    }

    fn generate_command(&self) -> String {
        todo!()
    }

    fn generate_file(&self, file_name: &str) {}
}

trait RuleManipulation {
    fn add_rule(&mut self, new_rule: Rule);
}

impl RuleManipulation for Vec<Rule> {
    fn add_rule(&mut self, new_rule: Rule){
        self.push(new_rule);
    }
}

fn main() {
    let mut anime = CronJob::new();
    anime.minute.add_rule(Rule::Range(0, 10));

    println!("{:#?}", anime);
}
