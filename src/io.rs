use serde::*;
use std::fs::*;
use std::io::{BufReader, BufWriter};
use std::path::*;

use rmp::Marker;
use rmp::encode::{ValueWriteError, write_map_len, write_str};
use rmp_serde::encode::VariantWriter;
use serde::Serialize;
use std::io::Write;

/// This struct is proposed in rmp-serde
/// https://github.com/3Hren/msgpack-rust/pull/133
struct StructMapWriter;
impl VariantWriter for StructMapWriter {
    fn write_struct_len<W>(&self, wr: &mut W, len: u32) -> Result<Marker, ValueWriteError>
    where
        W: Write,
    {
        write_map_len(wr, len) // write_array_len -> write_map_len
    }

    fn write_field_name<W>(&self, wr: &mut W, key: &str) -> Result<(), ValueWriteError>
    where
        W: Write,
    {
        write_str(wr, key) // OK(()) -> write_str
    }
}

pub fn save_msg<T: Serialize>(val: &T, filename: &str) {
    let f = File::create(filename).ok().unwrap();
    let mut buf = BufWriter::new(f);
    let mut enc = ::rmp_serde::Serializer::new(&mut buf);
    val.serialize(&mut enc).unwrap();
}

pub fn save_msg_as_map<T: Serialize>(val: &T, filename: &str) {
    let f = File::create(filename).ok().unwrap();
    let mut buf = BufWriter::new(f);
    let mut enc = ::rmp_serde::Serializer::with(&mut buf, StructMapWriter);
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

    fn name2path(&self, name: &str) -> PathBuf {
        let name = if name.ends_with(".msg") {
            name.to_string()
        } else {
            format!("{}.msg", name)
        };
        self.path.join(name)
    }

    pub fn save<T: Serialize>(&self, name: &str, data: &T) {
        let path = self.name2path(name);
        let mut buf = BufWriter::new(File::create(path).ok().unwrap());
        let mut enc = ::rmp_serde::Serializer::new(&mut buf);
        data.serialize(&mut enc).unwrap();
    }

    pub fn save_as_map<T: Serialize>(&self, name: &str, data: &T) {
        let path = self.name2path(name);
        let mut buf = BufWriter::new(File::create(path).ok().unwrap());
        let mut enc = ::rmp_serde::Serializer::with(&mut buf, StructMapWriter);
        data.serialize(&mut enc).unwrap();
    }
}
