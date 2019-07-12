use std::io::BufWriter;
use std::fs::OpenOptions;
use std::io::Write;

const BATCH_SIZE: u8 = 1;

pub enum LogType {
    Put = 0,
    Delete = 1,
}

impl ToString for LogType{
    fn to_string(&self) -> String{
        match self{
            LogType::Put    => String::from("0"),
            LogType::Delete => String::from("1"),
        }
    }
}

pub type LogFmt = Vec<(LogType, String, String)>;

pub struct Log {
    pub path: String,
    batch: LogFmt,
}


impl Log {
    pub fn new(path: &str) -> Self {
        Log {
            path: path.to_owned(),
            batch: Vec::new(),
        }
    }

    pub fn record(&mut self, log_type: LogType, key: &str, value: &str) {
        self.batch.push((log_type, key.to_owned(), value.to_owned()));
        self.flush_if_full();
    }

    pub fn flush(&mut self) {
        let file = OpenOptions::new()
            .append(true).create(true)
            .open(& self.path).unwrap();
        let mut writer = BufWriter::new(file);
        for (t, k, v) in self.batch.iter() {
            writer.write_fmt(format_args!("{} {} {}\n", t.to_string(), k, v)).expect("flush failed"); // Write Trait
        }
        self.batch.clear();
    }

    fn flush_if_full(&mut self){
        if self.batch.len() as u8 >= BATCH_SIZE {
            self.flush();
        }
    }
}