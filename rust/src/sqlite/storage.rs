
use rusqlite::Connection;
use super::{util, timeseries, ensemble, ensemble_series};
use super::super::types::*;
use super::super::{io, settings};

impl io::SeriesStorage for Connection {
    type Key = i64;
    fn save_truth(&self, setting: &settings::Truth, truth: &Truth) -> i64 {
        let table_name = util::generate_table_name("truth");
        timeseries::create_table(self, &table_name);
        timeseries::save(setting.dt, truth, self, &table_name);
        timeseries::register_truth(setting, self, &table_name)
    }
    fn save_observation(&self, setting: &settings::Observation, tid: i64, obs: &Observation) -> i64 {
        let table_name = util::generate_table_name("obs");
        timeseries::create_table(self, &table_name);
        timeseries::save(setting.dt, obs, self, &table_name);
        timeseries::register_observation(setting, tid, self, &table_name)
    }
    fn load_truth(&self, id: i64) -> (settings::Truth, Truth) {
        let (setting, tbname) = timeseries::lookup_truth(id, self);
        let v: Truth = timeseries::load(&tbname, self);
        (setting, v)
    }
    fn load_observation(&self, id: i64) -> (settings::Observation, i64, Observation) {
        let (setting, tid, tbname) = timeseries::lookup_observation(id, self);
        let v: Observation = timeseries::load(&tbname, self);
        (setting, tid, v)
    }
}

impl io::EnsembleStorage for Connection {
    type SeriesKey = String;
    type Key = String;
    fn save_ensemble(&self, xs: &Ensemble) -> Self::Key {
        let table_name = util::generate_table_name("ensemble");
        ensemble::create_table(self, &table_name);
        ensemble::insert(xs, self, &table_name);
        table_name
    }
    fn load_ensemble(&self, table_name: Self::Key) -> Ensemble {
        ensemble::load(&table_name, self)
    }
    fn commit_ensemble_series(&self, series: &[(f64, Self::Key, Self::Key)]) -> Self::SeriesKey {
        let table_name = util::generate_table_name("ensemble_series");
        ensemble_series::create_table(self, &table_name);
        for &(time, ref forecasted, ref analysized) in series.iter() {
            ensemble_series::insert(time, &forecasted, &analysized, self, &table_name);
        }
        table_name
    }
    fn query_ensemble_series(&self, table_name: Self::SeriesKey) -> Vec<(f64, Self::Key, Self::Key)> {
        ensemble_series::load(&table_name, self)
    }
}
