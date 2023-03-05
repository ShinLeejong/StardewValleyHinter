mod infos;

use infos::{npc::npc::NPC, event::event::Event, season::season::Season, Infos};

fn main() {
    init();
}

fn init() {
    let events: Vec<Event> = init_events();
    let npcs: Vec<NPC> = init_npcs();
    start(&events, &npcs);
}

fn start<'life>(events: &'life Vec<Event>, npcs: &'life Vec<NPC>) {
    println!("");
    let guide_msg: &str = "원하는 번호를 선택하세요.\n1) 계절\n2) NPC\n3) 축제\n\n또는, '계절'을 입력하여 모든 계절의 달력을 확인하거나, 'NPC'를 입력하여 모든 NPC의 생일을 확인하거나, '축제'를 입력하여 모든 축제의 날짜를 확인하세요.";
    println!("{}", guide_msg);
    
    let input: String = get_input();

    match input.as_str() {
        "1" => process_season(events, npcs),
        "2" => process_npc(events, npcs),
        "3" => process_event(events, npcs),
        "계절" => { print_season_all(events, npcs); start(events, npcs); },
        "NPC" => print_npc_all(events, npcs),
        "축제" => print_event_all(events, npcs),
        _ => print_not_found(events, npcs)
    }
}

fn restart<'life>(_err_msg: &str, events: &Vec<Event>, npcs: &Vec<NPC>) {
    println!("{}", _err_msg);
    println!("");
    start(events, npcs);
}

// process detail

fn process_season<'life>(_events: &'life Vec<Event>, _npcs: &'life Vec<NPC>) {
    let guide_msg: &str = "데이터를 표시할 계절을 입력하세요. ex) 봄";
    println!("{}", guide_msg);

    let input: String = get_input();

    print_season(input, _events, _npcs, false);
}

fn process_npc<'life>(_events: &'life Vec<Event>, _npcs: &'life Vec<NPC>) {
    let guide_msg: &str = "데이터를 표시할 NPC의 이름을 입력하세요. ex) 거스";
    println!("{}", guide_msg);

    let input: String = get_input();

    print_npc(input, _events, _npcs, false);
}

fn process_event<'life>(_events: &'life Vec<Event>, _npcs: &'life Vec<NPC>) {
    let guide_msg: &str = "데이터를 표시할 계절을 입력하세요. ex) 달걀 축제";
    println!("{}", guide_msg);

    let input: String = get_input();

    print_event(input, _events, _npcs, false);
}

// print logics

fn print_season<'life> (_input: String, _events: &'life Vec<Event>, _npcs: &'life Vec<NPC>, is_from_call_all: bool) {
    let season: Season = Season {
        name: _input
    };

    season.print();

    if !is_from_call_all {
        start(_events, _npcs);
    }
}

fn print_npc<'life> (_input: String, _events: &'life Vec<Event>, _npcs: &'life Vec<NPC>, is_from_call_all: bool) {

    let npc = get_npc_by_input(_input, &_npcs);

    if !is_from_call_all {
        print_line();
    }

    match npc {
        Some(e) => e.print(),
        None => { 
            print_line();
            restart("존재하지 않는 NPC 이름입니다.", _events, _npcs);
            return;
        }
    }

    if !is_from_call_all {
        print_line();
        start(_events, _npcs);
    }
}

fn print_event<'life>(_input: String, _events: &'life Vec<Event>, _npcs: &'life Vec<NPC>, is_from_call_all: bool) {

    let event = get_event_by_input(_input, &_events);

    if !is_from_call_all {
        print_line();
    }

    match event {
        Some(e) => e.print(),
        None => {
            print_line();
            restart("존재하지 않는 축제 이름입니다.", _events, _npcs);
            return;
        }
    }

    if !is_from_call_all {
        print_line();
        start(_events, _npcs);
    }
}

fn print_season_all<'life> (_events: &'life Vec<Event>, _npcs: &'life Vec<NPC>) {
    print_line();
    print_season(String::from("봄"), _events, _npcs, true);
    print_season(String::from("여름"), _events, _npcs, true);
    print_season(String::from("가을"), _events, _npcs, true);
    print_season(String::from("겨울"), _events, _npcs, true);
    print_line();
    start(_events, _npcs);
}

fn print_npc_all<'life> (_events: &'life Vec<Event>, _npcs: &'life Vec<NPC>) {
    print_line();
    for npc in _npcs {
        print_npc(String::from(npc.get_name()), &_events, &_npcs, true);
    }
    print_line();
    start(_events, _npcs);
}

fn print_event_all<'life> (_events: &'life Vec<Event>, _npcs: &'life Vec<NPC>) {
    print_line();
    for event in _events {
        print_event(String::from(event.get_name()), &_events, &_npcs, true);
    }
    print_line();
    start(_events, _npcs);
}

fn print_not_found<'life> (events: &'life Vec<Event>, npcs: &'life Vec<NPC>) {
    restart("올바르지 않은 입력값입니다.", events, npcs)
}

fn print_line() {
    println!("==================================================================");
}

// private functions

// initializations

fn init_events() -> Vec<Event> {
    let mut events: Vec<Event> = vec![];

    let event_names: [&str; 9] = ["달걀 축제", "봄꽃 무도회", "루아우", "달빛 해파리들의 춤", "스타듀 밸리 품평회", "영령의 전야제", "얼음 축제", "야시장", "겨울 별의 만찬"];
    let event_dates: [u8; 9] = [12, 23, 28 + 10, 28 + 27, 56 + 15, 56 + 26, 84 + 7, 84 + 14, 84 + 24];

    for (idx, _) in event_names.iter().enumerate() {
        let event: Event = Event {
            name: String::from(event_names[idx]),
            season: get_season(event_dates[idx]),
            days: get_days(event_dates[idx]),
            day: get_day(event_dates[idx])
        };

        events.push(event);
    }

    return events;
}

fn init_npcs() -> Vec<NPC> {
    let mut npcs: Vec<NPC> = vec![];

    let npc_names: [&str; 22] = ["루이스", "피에르", "캐롤라인", "조지", "에블린", "마니", "재스", "로빈", "드미트리우스", "거스", "팸", "켄트", "조디", "빈센트", "클린트", "윌리", "마법사", "라이너스", "드워프", "크로버스", "샌디", "레오"];
    let npc_dates: [u8; 22] = [6, 25, 28 * 3 + 6, 28 * 2 + 23, 28 * 3 + 19, 28 * 2 + 17, 28 * 1 + 3, 28 * 2 + 20, 28 * 1 + 18, 28 * 1 + 7, 17, 3, 28 * 2 + 10, 9, 28 * 3 + 25, 28 + 24, 28 * 3 + 16, 28 * 3 + 2, 28 + 22, 28 * 3 + 0, 28 * 2 + 14, 28 + 25];
    let npc_favorites: [[&str; 3]; 22] = [["매운 고추", "맥주", "블루베리"], ["토끼발", "계란류", "우유류"], ["토끼발", "토파즈", "커피"], ["토끼발", "맥주", "커피"], ["다이아몬드", "맥주", "커피"], ["다이아몬드", "커피", "계란류"], ["피자", "수선화", "코코넛"], ["복숭아", "우유류", "과일류"], ["아이스크림", "계란류", "맥주"], ["다이아몬드", "커피", "맥주"], ["맥주", "선인장 열매", "과일류"], ["계란류", "마요네즈", "과일류"], ["다이아몬드", "달걀류", "우유류"], ["코코넛", "피자", "아이스크림"], ["다이아 제외 보석", "금 주괴", "구리 주괴"], ["다이아몬드", "맥주", "석영"], ["태양 정수","공허 정수","석영"], ["선인장 열매", "코코넛", "과일류"], ["다이아 제외 보석", "석영", "빵"], ["다이아몬드", "공허 달걀", "석영"], ["토끼발", "과일류", "석영"], ["망고", "토끼발", "석영"]];

    for (idx, _) in npc_names.iter().enumerate() {
        let npc: NPC = NPC {
            name: String::from(npc_names[idx]),
            season: get_season(npc_dates[idx]),
            days: get_days(npc_dates[idx]),
            day: get_day(npc_dates[idx]),
            favorites: [String::from(npc_favorites[idx][0]), String::from(npc_favorites[idx][1]), String::from(npc_favorites[idx][2])]
        };

        npcs.push(npc);
    }

    return npcs;
}

fn get_season(dates: u8) -> String {
    match dates / 28 {
        0 => String::from("봄"),
        1 => String::from("여름"),
        2 => String::from("가을"),
        3 => String::from("겨울"),
        _ => String::from("")
    }
}

fn get_days(dates: u8) -> u8 {
    dates % 28
}

fn get_day(dates: u8) -> String {
    match dates % 7 {
        0 => String::from("일"),
        1 => String::from("월"),
        2 => String::from("화"),
        3 => String::from("수"),
        4 => String::from("목"),
        5 => String::from("금"),
        6 => String::from("토"),
        _ => String::from("")
    }
}

// find data

fn get_event_by_input(_input: String, _events: &Vec<Event>) -> Option<&Event> {
    for event in _events {
        if _input == event.get_name() {
            return Some(event)
        }
    }
    None
}

fn get_npc_by_input(_input: String, _npcs: &Vec<NPC>) -> Option<&NPC> {
    for npc in _npcs {
        if _input == npc.get_name() {
            return Some(npc)
        }
    }
    None
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("invalid input");

    input.pop();
    input.pop();

    input
}