pub struct NPC {
    pub name: String,   // 이름
    pub season: String, // 계절
    pub days: u8,       // 일
    pub day: String,    // 요일
    pub favorites: [String; 3]
}

impl crate::infos::Infos for NPC {
    fn print(&self) -> () {
        let mut first_tab = "";
        match self.name.len() {
            0..=8 => first_tab = "\t\t",
            9..=20 => first_tab = "\t",
            _ => ()
        }

        let mut second_tab = "";
        match self.season.len() / 3 + self.days.to_string().len() {
            0..=3 => second_tab = "\t\t",
            _ => second_tab = "\t",
        }

        let mut favorites: String = String::new();
        for favorite in &self.favorites {
            favorites.push_str(&favorite);
            favorites.push_str(", ");
        }
        favorites.pop();
        favorites.pop();

        println!("| 이름 : {}{}생일: {} {}일({}요일){} 좋아하는 것: {}", self.name, first_tab, self.season, self.days, self.day, second_tab, favorites)
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