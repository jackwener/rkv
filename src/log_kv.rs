use std::collections::{HashMap};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};


use std::ffi::OsStr;

pub struct KvStore {
    // directory for the log and other data.
    path: PathBuf,
    // map generation number to the file reader.
    readers: HashMap<u64, BufReaderWithPos<File>>,
    // writer of the current log.
    writer: BufWriterWithPos<File>,
    current_gen: u64,
    index: HashMap<String, CommandPos>,
    // the number of bytes representing "stale" commands that could be
    // deleted during a compaction.
    uncompacted: u64,
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {


    }

    pub fn set(&mut self, key: String, value: String) -> Result<> {

    }

    pub fn get(&mut self, key: String, value: String) ->
}