use rand::*;

const MONTHS: [&str; 12] = [
    "December",
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
];

pub struct Turn {
    pub turn: i32,
    pub label: String,
}

impl Turn {
    fn set_date_label(&mut self) {
        let year = self.turn / 12 + 1;
        let month = self.turn % 12;
        let mut max = 31;
        if month == 4 || month == 6 || month == 9 || month == 11 {
            max = 30;
        }
        if month == 2 {
            if year % 4 == 0 && year % 100 != 0 && year % 400 == 0 {
                max = 29;
            } else {
                max = 28;
            }
        }
        let random_day = rand::thread_rng().gen_range(1..max);
        self.label = format!("{}  {}  {}", random_day, MONTHS[month as usize], year);
    }

    pub fn init_turn() -> Turn {
        let mut turn = Turn {
            turn: 1,
            label: "".to_string(),
        };
        turn.set_date_label();
        turn
    }
    pub fn next_turn(&mut self) {
        self.turn += 1;
        self.set_date_label();
    }
}
