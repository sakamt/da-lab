
use rusqlite::Connection;
use super::{util, timeseries};
use super::super::types::*;
use super::super::{io, settings};

pub struct SqliteStorage<'a> {
    conn: &'a Connection,
}

impl<'a> io::SeriesStorage for SqliteStorage<'a> {
    type Key = i64;
    fn save_truth(&self, setting: &settings::Truth, truth: &Truth) -> i64 {
        let table_name = util::generate_table_name("truth");
        timeseries::save(setting.dt, truth, self.conn, &table_name);
        timeseries::register_truth(setting, self.conn, &table_name)
    }
    fn save_observation(&self, setting: &settings::Observation, tid: i64, obs: &Observation) -> i64 {
        let table_name = util::generate_table_name("obs");
        timeseries::save(setting.dt, obs, self.conn, &table_name);
        timeseries::register_observation(setting, tid, self.conn, &table_name)
    }
    fn load_truth(&self, id: i64) -> (settings::Truth, Truth) {
        let (setting, tbname) = timeseries::lookup_truth(id, self.conn);
        let v: Truth = timeseries::load(&tbname, self.conn);
        (setting, v)
    }
    fn load_observation(&self, id: i64) -> (settings::Observation, i64, Observation) {
        let (setting, tid, tbname) = timeseries::lookup_observation(id, self.conn);
        let v: Observation = timeseries::load(&tbname, self.conn);
        (setting, tid, v)
    }
}
