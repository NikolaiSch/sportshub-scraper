use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Sport {
    pub name: &'static str,
    pub url: &'static str,
}
impl Sport {
    pub const fn new(name: &'static str, url: &'static str) -> Sport {
        Sport { name, url }
    }
}

// used github copilot to generate this
pub const SOCCER: Sport = Sport::new("Football", "https://reddit3.sportshub.stream/");
pub const AMERICAN_FOOTBALL: Sport = Sport::new("AmericanFootball", "https://football.sportshub.stream/");
pub const BASKETBALL: Sport = Sport::new("Basketball", "https://basketball2.sportshub.stream/");
pub const BASEBALL: Sport = Sport::new("Baseball", "https://baseball.sportshub.stream/");
pub const HOCKEY: Sport = Sport::new("Hockey", "https://hockey.sportshub.stream");
pub const TENNIS: Sport = Sport::new("Tennis", "https://tennis.sportshub.stream/");
pub const BOXING: Sport = Sport::new("Boxing", "https://boxing1.sportshub.stream/");
pub const FIGHTS: Sport = Sport::new("Fights", "https://mma.sportshub.stream/");
pub const MOTORSPORTS: Sport = Sport::new("Motorsports", "https://motorsport.sportshub.stream/");
pub const HORSE_RACING: Sport = Sport::new("HorseRacing", "https://sportshub.stream/horse-racing-streams/");
pub const RUGBY: Sport = Sport::new("Rugby", "https://rugby.sportshub.stream/");
pub const RUGBY_UNION: Sport = Sport::new("RugbyUnion", "https://sportshub.stream/rugby-union-streams/");
pub const RUGBY_SEVENS: Sport = Sport::new("RugbySevens", "https://sportshub.stream/rugby-sevens-streams/");
pub const CYCLING: Sport = Sport::new("Cycling", "https://cycling.sportshub.stream/");
pub const CRICKET: Sport = Sport::new("Cricket", "https://cricket.sportshub.stream/");
pub const GOLF: Sport = Sport::new("Golf", "https://golf.sportshub.stream/");
pub const AFL: Sport = Sport::new("AFL", "https://afl.sportshub.stream/");
pub const VOLLEYBALL: Sport = Sport::new("Volleyball", "https://volleyball.sportshub.stream/");
pub const SNOOKER: Sport = Sport::new("Snooker", "https://snooker.sportshub.stream/");
pub const DARTS: Sport = Sport::new("Darts", "https://darts1.sportshub.stream/");
pub const WATERSPORTS: Sport = Sport::new("Watersports", "https://sportshub.stream/water-sports-streams/");
pub const SUMMER_SPORTS: Sport = Sport::new("SummerSports", "https://sportshub.stream/summer-sports-streams/");
pub const BEACH_SPORTS: Sport = Sport::new("BeachSports", "https://sportshub.stream/beach-soccer-streams/");
pub const ESPORTS: Sport = Sport::new("Esports", "https://sportshub.stream/esports-streams/");
pub const HANDBALL: Sport = Sport::new("Handball", "https://handball.sportshub.stream/");
pub const ATHLETICS: Sport = Sport::new("Athletics", "https://sportshub.stream/athletics-streams/");
pub const TRIATHLON: Sport = Sport::new("Triathlon", "https://sportshub.stream/thriatlon-streams/");
pub const BEACH_VOLLEY: Sport = Sport::new("BeachVolley", "https://sportshub.stream/beach-volley-streams/");
pub const WATER_POLO: Sport = Sport::new("WaterPolo", "https://sportshub.stream/water-polo-streams/");
pub const BADMINTON: Sport = Sport::new("Badminton", "https://badminton.sportshub.stream/");
pub const FLOORBALL: Sport = Sport::new("Floorball", "https://sportshub.stream/floorball-streams/");
pub const FIELD_HOCKEY: Sport = Sport::new("FieldHockey", "https://sportshub.stream/field-hockey-streams/");
pub const TABLE_TENNIS: Sport = Sport::new("TableTennis", "https://sportshub.stream/table-tennis-streams/");
pub const ROWING: Sport = Sport::new("Rowing", "https://sportshub.stream/rowing-streams/");
pub const FUTSAL: Sport = Sport::new("Futsal", "https://sportshub.stream/futsal-streams/");
pub const NETBALL: Sport = Sport::new("Netball", "https://sportshub.stream/netball-streams/");
pub const WINTER_SPORTS: Sport = Sport::new("WinterSports", "https://sportshub.stream/winter-sports-streams/");
pub const CURLING: Sport = Sport::new("Curling", "https://sportshub.stream/curling-streams/");

pub const SPORTS: [Sport; 38] = [
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
