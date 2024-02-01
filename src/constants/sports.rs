pub struct Sports {
    pub name: &'static str,
    pub url: &'static str,
}

impl Sports {
    pub const fn new(name: &'static str, url: &'static str) -> Sports {
        Sports { name, url }
    }
}

// used github copilot to generate this
pub const SOCCER: Sports = Sports::new("Football", "https://reddit3.sportshub.stream/");
pub const AMERICAN_FOOTBALL: Sports = Sports::new("AmericanFootball", "https://football.sportshub.stream/");
pub const BASKETBALL: Sports = Sports::new("Basketball", "https://basketball2.sportshub.stream/");
pub const BASEBALL: Sports = Sports::new("Baseball", "https://baseball.sportshub.stream/");
pub const HOCKEY: Sports = Sports::new("Hockey", "https://hockey.sportshub.stream");
pub const TENNIS: Sports = Sports::new("Tennis", "https://tennis.sportshub.stream/");
pub const BOXING: Sports = Sports::new("Boxing", "https://boxing1.sportshub.stream/");
pub const FIGHTS: Sports = Sports::new("Fights", "https://mma.sportshub.stream/");
pub const MOTORSPORTS: Sports = Sports::new("Motorsports", "https://motorsport.sportshub.stream/");
pub const HORSE_RACING: Sports = Sports::new("HorseRacing", "https://sportshub.stream/horse-racing-streams/");
pub const RUGBY: Sports = Sports::new("Rugby", "https://rugby.sportshub.stream/");
pub const RUGBY_UNION: Sports = Sports::new("RugbyUnion", "https://sportshub.stream/rugby-union-streams/");
pub const RUGBY_SEVENS: Sports = Sports::new("RugbySevens", "https://sportshub.stream/rugby-sevens-streams/");
pub const CYCLING: Sports = Sports::new("Cycling", "https://cycling.sportshub.stream/");
pub const CRICKET: Sports = Sports::new("Cricket", "https://cricket.sportshub.stream/");
pub const GOLF: Sports = Sports::new("Golf", "https://golf.sportshub.stream/");
pub const AFL: Sports = Sports::new("AFL", "https://afl.sportshub.stream/");
pub const VOLLEYBALL: Sports = Sports::new("Volleyball", "https://volleyball.sportshub.stream/");
pub const SNOOKER: Sports = Sports::new("Snooker", "https://snooker.sportshub.stream/");
pub const DARTS: Sports = Sports::new("Darts", "https://darts1.sportshub.stream/");
pub const WATERSPORTS: Sports = Sports::new("Watersports", "https://sportshub.stream/water-sports-streams/");
pub const SUMMER_SPORTS: Sports = Sports::new("SummerSports", "https://sportshub.stream/summer-sports-streams/");
pub const BEACH_SPORTS: Sports = Sports::new("BeachSports", "https://sportshub.stream/beach-soccer-streams/");
pub const ESPORTS: Sports = Sports::new("Esports", "https://sportshub.stream/esports-streams/");
pub const HANDBALL: Sports = Sports::new("Handball", "https://handball.sportshub.stream/");
pub const ATHLETICS: Sports = Sports::new("Athletics", "https://sportshub.stream/athletics-streams/");
pub const TRIATHLON: Sports = Sports::new("Triathlon", "https://sportshub.stream/thriatlon-streams/");
pub const BEACH_VOLLEY: Sports = Sports::new("BeachVolley", "https://sportshub.stream/beach-volley-streams/");
pub const WATER_POLO: Sports = Sports::new("WaterPolo", "https://sportshub.stream/water-polo-streams/");
pub const BADMINTON: Sports = Sports::new("Badminton", "https://badminton.sportshub.stream/");
pub const FLOORBALL: Sports = Sports::new("Floorball", "https://sportshub.stream/floorball-streams/");
pub const FIELD_HOCKEY: Sports = Sports::new("FieldHockey", "https://sportshub.stream/field-hockey-streams/");
pub const TABLE_TENNIS: Sports = Sports::new("TableTennis", "https://sportshub.stream/table-tennis-streams/");
pub const ROWING: Sports = Sports::new("Rowing", "https://sportshub.stream/rowing-streams/");
pub const FUTSAL: Sports = Sports::new("Futsal", "https://sportshub.stream/futsal-streams/");
pub const NETBALL: Sports = Sports::new("Netball", "https://sportshub.stream/netball-streams/");
pub const WINTER_SPORTS: Sports = Sports::new("WinterSports", "https://sportshub.stream/winter-sports-streams/");
pub const CURLING: Sports = Sports::new("Curling", "https://sportshub.stream/curling-streams/");

pub const SPORTS: [Sports; 38] = [
    SOCCER,
    AMERICAN_FOOTBALL,
    BASKETBALL,
    BASEBALL,
    HOCKEY,
    TENNIS,
    BOXING,
    FIGHTS,
    MOTORSPORTS,
    HORSE_RACING,
    RUGBY,
    RUGBY_UNION,
    RUGBY_SEVENS,
    CYCLING,
    CRICKET,
    GOLF,
    AFL,
    VOLLEYBALL,
    SNOOKER,
    DARTS,
    WATERSPORTS,
    SUMMER_SPORTS,
    BEACH_SPORTS,
    ESPORTS,
    HANDBALL,
    ATHLETICS,
    TRIATHLON,
    BEACH_VOLLEY,
    WATER_POLO,
    BADMINTON,
    FLOORBALL,
    FIELD_HOCKEY,
    TABLE_TENNIS,
    ROWING,
    FUTSAL,
    NETBALL,
    WINTER_SPORTS,
    CURLING,
];
