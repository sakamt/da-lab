use serde::*;
use std::fs::*;
use std::io::{BufReader, BufWriter};
use std::path::*;

pub fn save_msg<T: Serialize>(val: &T, filename: &str) {
    let f = File::create(filename).ok().unwrap();
    let mut buf = BufWriter::new(f);
    let mut enc = ::rmp_serde::Serializer::new(&mut buf);
    val.serialize(&mut enc).unwrap();
}

pub fn load_msg<T: Deserialize>(filename: &str) -> T {
    let f = File::open(filename).unwrap();
    let mut buf = BufReader::new(f);
    let mut dec = ::rmp_serde::Deserializer::new(&mut buf);
    Deserialize::deserialize(&mut dec).unwrap()
}

pub fn read_json<Contents: Deserialize>(filename: &str) -> Contents {
    let f = File::open(filename).unwrap();
    let mut buf = BufReader::new(f);
    ::serde_json::from_reader(&mut buf).unwrap()
}

pub struct MsgpackSaver {
    pub path: PathBuf,
}

impl MsgpackSaver {
    pub fn new(kind: &str) -> Self {
        let root = match ::std::env::var("DATA_DIR") {
            Ok(data_dir) => {
                info!("Use $DATA_DIR({}) as root", data_dir);
                PathBuf::from(data_dir)
            }
            Err(_) => {
                let cur = ::std::env::current_dir().expect("Cannot get current_dir");
                info!("Use current directory ({:?}) as root", cur);
                cur
            }
        };
        let job_id = ::uuid::Uuid::new_v4();
        let path = root.join(kind).join(job_id.simple().to_string());
        ::std::fs::create_dir_all(&path).expect("Cannot create data directory");
        info!("data directory = {:?}", &path);
        Self { path }
    }

    pub fn save<T: Serialize>(&self, name: &str, data: &T) {
        save_msg(data, self.path.join(name).to_str().unwrap());
    }
}
