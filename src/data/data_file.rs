use std::{path::PathBuf, sync::Arc};

use parking_lot::RwLock;

use crate::{errors::Result, fio};

use super::log_record::{LogRecord, ReadLogRecord};

pub const DATA_FILE_NAME_SUFFIX: &str = ".data";
/// 数据文件
pub struct DataFile {
    file_id: Arc<RwLock<u32>>,           // 数据文件id
    write_off: Arc<RwLock<u64>>,         // 当前写偏移，记录数据文件写到哪个位置了
    io_manager: Box<dyn fio::IOManager>, // io管理接口
}

impl DataFile {
    // 创建或打开一个新的数据文件
    pub fn new(dir_path: PathBuf, file_id: u32) -> Result<DataFile> {
        todo!()
    }

    pub fn get_write_off(&self) -> u64 {
        let read_guard = self.write_off.read();
        *read_guard
    }

    pub fn set_write_off(&self, offset: u64) {
        let mut write_guard = self.write_off.write();
        *write_guard = offset;
    }

    pub fn get_file_id(&self) -> u32 {
        let read_guard = self.file_id.read();
        *read_guard
    }

    pub fn read_log_record(&self, offset: u64) -> Result<ReadLogRecord> {
        todo!()
    }

    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        todo!()
    }

    pub fn sync(&self) -> Result<()> {
        todo!()
    }
}
