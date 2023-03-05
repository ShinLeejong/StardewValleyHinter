use std::collections::HashMap;

pub struct Season {
    pub name: String
}

impl crate::infos::Infos for Season {
    fn print(&self) -> () {
        println!("==================================================================");
        match self.name.as_str() {
            "봄" => print_season(0),
            "여름" => print_season(1),
            "가을" => print_season(2),
            "겨울" => print_season(3),
            _ => print_wrong_season()
        }
        println!("==================================================================");
    }

    fn get_name(&self) -> &str {
        ""
    }

    fn get_season(&self) -> &str {
        ""
    }

    fn get_days(&self) -> u8 {
        0
    }

    fn get_day(&self) -> &str {
        ""
    }
}

fn print_season(season_num: u8) {
    let mut cal_info: HashMap<u8, &str> = HashMap::new();

    cal_info.insert(6, "루이스");
    cal_info.insert(25, "피에르");
    cal_info.insert(28 * 3 + 6, "캐롤라인");
    cal_info.insert(28 * 2 + 23, "조지");
    cal_info.insert(28 * 3 + 19, "에블린");
    cal_info.insert(28 * 2 + 17, "마니");
    cal_info.insert(28 + 3, "재스");
    cal_info.insert(28 * 2 + 20, "로빈");
    cal_info.insert(28 + 18, "드미트리우스");
    cal_info.insert(28 + 7, "거스");
    cal_info.insert(17, "팸");
    cal_info.insert(3, "켄트");
    cal_info.insert(28 * 2 + 10, "조디");
    cal_info.insert(9, "빈센트");
    cal_info.insert(28 * 3 + 25, "클린트");
    cal_info.insert(28 + 23, "윌리");
    cal_info.insert(28 * 3 + 16, "마법사");
    cal_info.insert(28 * 3 + 2, "라이너스");
    cal_info.insert(28 + 21, "드워프");
    cal_info.insert(28 * 3 + 0, "크로버스");
    cal_info.insert(28 * 2 + 14, "샌디");
    cal_info.insert(28 + 25, "레오");
    cal_info.insert(12, "달걀 축제");
    cal_info.insert(23, "봄꽃 무도회");
    cal_info.insert(28 + 10, "루아우");
    cal_info.insert(28 + 27, "달빛 해파리들의 춤");
    cal_info.insert(28 * 2 + 15, "스타듀 밸리 품평회");
    cal_info.insert(28 * 2 + 26, "열령의 전야제");
    cal_info.insert(28 * 3 + 7, "얼음 축제");
    cal_info.insert(28 * 3 + 14, "야시장");
    cal_info.insert(28 * 3 + 15, "야시장");
    cal_info.insert(28 * 3 + 16, "야시장");
    cal_info.insert(28 * 3 + 24, "겨울 별의 만찬");

    let mut idx: u8 = 0;

    // 개선 필요
    while idx < 28 {
        print!("| {}\t", idx);

        let item_option: Option<&&str> = cal_info.get(&(idx + (season_num * 28)));
        if item_option != None {
            print!("{}", item_option.unwrap());
        }
        else {
            print!(" ");
        }

        print!(" |");

        if (idx + 1) % 7 == 0 {
            println!("");
        }

        idx += 1;
    }
}

fn print_wrong_season() {
    println!("올바르지 않은 계절입니다.");
}