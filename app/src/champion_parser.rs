use crate::{database::Database, models::{champion_model::Champion, recommended_champion_model::RecommendedChampion}};
use std::{thread, time};
use std::collections::HashMap;

fn get_highest_str(map: HashMap<String, i64>) -> String {
    
    let mut result = String::new();
    let mut tmp: i64 = -1;

    for (entry, value) in map {
        if tmp == -1 {
            result = entry;
            tmp = value;
            continue;
        }
        if value > tmp {
            result = entry;
            tmp = value;
        }
    }

    result
}

fn get_highest(map: HashMap<i64, i64>) -> i64 {
    
    let mut result: i64 = 0;
    let mut tmp: i64 = -1;

    for (entry, value) in map {
        if tmp == -1 {
            result = entry;
            tmp = value;
            continue;
        }
        if value > tmp {
            result = entry;
            tmp = value;
        }
    }

    result
}

fn read_matches(champion_name: String, matches: &Vec<Champion>, role: String) -> RecommendedChampion {

    let mut wins = 0;
    let mut loses = 0;
    let mut item0: HashMap<i64, i64> = HashMap::new();
    let mut item1: HashMap<i64, i64> = HashMap::new();
    let mut item3: HashMap<i64, i64> = HashMap::new();
    let mut item2: HashMap<i64, i64> = HashMap::new();
    let mut item4: HashMap<i64, i64> = HashMap::new();
    let mut item5: HashMap<i64, i64> = HashMap::new();
    let mut item6: HashMap<i64, i64> = HashMap::new();
    let mut rune: HashMap<String, i64> = HashMap::new();
    let mut rune1: HashMap<String, i64> = HashMap::new();
    let mut rune2: HashMap<String, i64> = HashMap::new();
    let mut summoner1: HashMap<String, i64> = HashMap::new();
    let mut summoner2: HashMap<String, i64> = HashMap::new();
    let mut spellmax1: HashMap<String, i64> = HashMap::new();
    let mut spellmax2: HashMap<String, i64> = HashMap::new();
    let mut spellmax3: HashMap<String, i64> = HashMap::new();
    let mut spellmax4: HashMap<String, i64> = HashMap::new();

    for element in matches {
        if element.role != role {
            continue;
        }

        match element.win {
            true => wins += 1,
            false => loses += 1,
        };

        item0.insert(element.item0, *(item0.get(&element.item0).get_or_insert(&0)) + 1);
        item1.insert(element.item1, *(item1.get(&element.item1).get_or_insert(&0)) + 1);
        item2.insert(element.item2, *(item2.get(&element.item2).get_or_insert(&0)) + 1);
        item3.insert(element.item3, *(item3.get(&element.item3).get_or_insert(&0)) + 1);
        item4.insert(element.item4, *(item4.get(&element.item4).get_or_insert(&0)) + 1);
        item5.insert(element.item5, *(item5.get(&element.item5).get_or_insert(&0)) + 1);
        item6.insert(element.item6, *(item6.get(&element.item6).get_or_insert(&0)) + 1);
        rune.insert(element.rune.clone(), *(rune.get(&element.rune.clone()).get_or_insert(&0)) + 1);
        rune1.insert(element.rune1.clone(), *(rune1.get(&element.rune1.clone()).get_or_insert(&0)) + 1);
        rune2.insert(element.rune2.clone(), *(rune2.get(&element.rune2.clone()).get_or_insert(&0)) + 1);
        summoner1.insert(element.summoner1.clone(), *(summoner1.get(&element.summoner1.clone()).get_or_insert(&0)) + 1);
        summoner2.insert(element.summoner2.clone(), *(summoner2.get(&element.summoner2.clone()).get_or_insert(&0)) + 1);
        spellmax1.insert(element.spellmax1.clone(), *(spellmax1.get(&element.spellmax1.clone()).get_or_insert(&0)) + 1);
        spellmax2.insert(element.spellmax2.clone(), *(spellmax2.get(&element.spellmax2.clone()).get_or_insert(&0)) + 1);
        spellmax3.insert(element.spellmax3.clone(), *(spellmax3.get(&element.spellmax3.clone()).get_or_insert(&0)) + 1);
        spellmax4.insert(element.spellmax4.clone(), *(spellmax4.get(&element.spellmax4.clone()).get_or_insert(&0)) + 1);
    }

    RecommendedChampion {
        id: None,
        wins,
        loses,
        name: champion_name,
        title: "Default title".into(),
        item0: get_highest(item0),
        item1: get_highest(item1),
        item2: get_highest(item2),
        item3: get_highest(item3),
        item4: get_highest(item4),
        item5: get_highest(item5),
        item6: get_highest(item6),
        role,
        rune: get_highest_str(rune),
        rune1: get_highest_str(rune1),
        rune2: get_highest_str(rune2),
        summoner1: get_highest_str(summoner1),
        summoner2: get_highest_str(summoner2),
        spellmax1: get_highest_str(spellmax1),
        spellmax2: get_highest_str(spellmax2),
        spellmax3: get_highest_str(spellmax3),
        spellmax4: get_highest_str(spellmax4),
    }

}

fn update_champion(db: &Database, champion_name: String, matches: Vec<Champion>) {

    let _ = db.update_recommended_champion(read_matches(champion_name.clone(), &matches, "TOP".into()));
    let _ = db.update_recommended_champion(read_matches(champion_name.clone(), &matches, "JUNGLE".into()));
    let _ = db.update_recommended_champion(read_matches(champion_name.clone(), &matches, "MIDDLE".into()));
    let _ = db.update_recommended_champion(read_matches(champion_name.clone(), &matches, "BOT".into()));
    let _ = db.update_recommended_champion(read_matches(champion_name.clone(), &matches, "UTILITY".into()));

}

pub fn initialize_champions(db: Database) {
    loop {

        let data = db.get_champions();

        if data.is_err() {
            continue;
        }

        let data = data.unwrap();

        for (champion_name, matches) in data {
            update_champion(&db, champion_name, matches);
        }

        thread::sleep(time::Duration::from_secs(300));
    }
}
