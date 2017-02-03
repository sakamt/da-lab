
use rusqlite::Connection;
use super::{util, timeseries, ensemble, ensemble_series};
use super::super::types::*;
use super::super::{io, settings};

pub struct SqliteStorage<'a> {
    conn: &'a Connection,
}

impl<'a> SqliteStorage<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        SqliteStorage { conn: conn }
    }
}

impl<'a> io::SeriesStorage for SqliteStorage<'a> {
    type Key = i64;
    fn save_truth(&self, setting: &settings::Truth, truth: &Truth) -> i64 {
        let table_name = util::generate_table_name("truth");
        timeseries::create_table(self.conn, &table_name);
        timeseries::save(setting.dt, truth, self.conn, &table_name);
        timeseries::register_truth(setting, self.conn, &table_name)
    }
    fn save_observation(&self, setting: &settings::Observation, tid: i64, obs: &Observation) -> i64 {
        let table_name = util::generate_table_name("obs");
        timeseries::create_table(self.conn, &table_name);
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

impl<'a> io::EnsembleStorage for SqliteStorage<'a> {
    type SeriesKey = String;
    type Key = String;
    fn save(&self, xs: &Ensemble) -> Self::Key {
        let table_name = util::generate_table_name("ensemble");
        ensemble::create_table(self.conn, &table_name);
        ensemble::insert(xs, self.conn, &table_name);
        table_name
    }
    fn commit(&self, series: &[(f64, Self::Key, Self::Key)]) -> Self::SeriesKey {
        let table_name = util::generate_table_name("ensemble_series");
        ensemble_series::create_table(self.conn, &table_name);
        for &(time, ref forecasted, ref analysized) in series.iter() {
            ensemble_series::insert(time, &forecasted, &analysized, self.conn, &table_name);
        }
        table_name
    }
    fn load(&self, _: Self::Key) -> Ensemble {
        // TODO
        Vec::new()
    }
    fn query(&self, _: Self::SeriesKey) -> Vec<(f64, Self::Key, Self::Key)> {
        // TODO
        Vec::new()
    }
}
