use std::fs;
use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

const STATS_PATH: &str = "statistics.json";

static FILE_MUTEX: Option<Mutex<Stats>> = None;

/// Global statistics
/// All stats are only counted for games that were finished
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Stats {
    #[serde(default = "zero")]          pub total_wins: u64,
    #[serde(default = "zero")]          pub total_draws: u64,
    #[serde(default = "empty_map")]     pub language: HashMap<crate::Language, u64>,
    #[serde(default = "empty_map")]     pub timer: HashMap<u64, u64>,
    #[serde(default = "empty_map")]     pub win_turn: HashMap<u64, u64>, // At which turn did the game end?
    #[serde(default = "empty_map")]     pub game_count_for_one_room: HashMap<u64, u64>, // How many games did people play in one room (accumulated, if played 3 games it counts for 1, 2 and 3 games)
    #[serde(default = "zero")]          pub max_room_active_at_same_time: u64,
}

pub type StatsHandle = Arc<Mutex<Option<Stats>>>;

pub fn update_stats(stats: &Arc<Mutex<Option<Stats>>>, update_fn: &dyn Fn(&mut Stats) -> ()) {
    match stats.lock() {
        Ok(mut stats) => {
            match &mut *stats {
                Some(st) => {
                    update_fn(st);
                    let _ = save(st);
                },
                None => (),
            }
        },
        Err(_) => log::error!("Couldn't acquire stats mutex!")
    }    
}

pub fn load() -> Option<Stats> {
    if fs::exists(STATS_PATH).is_ok_and(|exists| exists) {
        let statistics_json = fs::read(STATS_PATH)
            .map_err(|err| log::error!("Couldn't read stats file: {}. Stats will not be modified.", err)).ok()?;
        let stats = serde_json::from_slice::<Stats>(&statistics_json)
            .map_err(|err| log::error!("Couldn't deserialize stats file: {}. Stats will not be modified.", err)).ok()?;

        return Some(stats);
    }
    else {
        log::info!("Stats file doesn't exist, creating new stats.");

        return Some(Stats {
            total_draws: 0,
            total_wins: 0,
            language: HashMap::new(),
            timer: HashMap::new(),
            win_turn: HashMap::new(),
            game_count_for_one_room: HashMap::new(),
            max_room_active_at_same_time: 0,
        });
    }
}

pub fn save(stats: &Stats) -> Result<(), ()> {
    let serialized_stats = serde_json::to_string_pretty(stats)
        .map_err(|err| log::error!("Couldn't serialize stats file: {}", err))?;

    fs::write(STATS_PATH, serialized_stats)
        .map_err(|err| log::error!("Couldn't write stats file: {}", err))?;

    Ok(())
}

pub fn increment_stat_map_counter<T>(map: &mut HashMap<T, u64>, key: T) where T: Eq + std::hash::Hash + Clone {
    match map.get(&key) {
        Some(v) => map.insert(key, v + 1),
        None => map.insert(key, 1),
    };
}

fn empty_map<T>() -> HashMap<T, u64> {
    HashMap::new()
}

fn zero() -> u64 {
    0
}
