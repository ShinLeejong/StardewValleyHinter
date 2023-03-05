pub struct Event {
    pub name: String,   // 이름
    pub season: String, // 계절
    pub days: u8,       // 일
    pub day: String,    // 요일
}

impl crate::infos::Infos for Event {
    fn print(&self) -> () {
        let mut tab = "";
        match self.name.len() {
            0..=13 => tab = "\t\t\t",
            14..=20 => tab = "\t\t",
            21..=256 => tab = "\t",
            _ => ()
        }
        println!("| 축제 이름 : {}{}{} {}일({}요일)", self.name, tab, self.season, self.days, self.day);
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_season(&self) -> &str {
        &self.season
    }

    fn get_days(&self) -> u8 {
        self.days
    }

    fn get_day(&self) -> &str {
        &self.day
    }
}