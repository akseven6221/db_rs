use prost::length_delimiter_len;

#[derive(PartialEq)]
pub enum LogRecordType {
    // 正常 put 的数据
    NORMAL = 1,

    // 被删除的数据标识，墓碑值
    DELETED = 2,
}
/// LogRecord 写入到数据文件的记录
/// 之所以叫日志，是因为数据文件中的数据是追加写入的，类似日志的格式
pub struct LogRecord {
    pub(crate) key: Vec<u8>,
    pub(crate) value: Vec<u8>,
    pub(crate) rec_type: LogRecordType,
}

/// 数据位置索引信息，描述数据存储到了哪个位置
#[derive(Clone, Copy, Debug)]
pub struct LogRecordPos {
    pub(crate) file_id: u32, // 文件 id，表示将数据存储在了哪个文件中
    pub(crate) offset: u64,  // 偏移，表示将数据存储在了数据文件的哪个位置
}

/// 从数据文件中读取的 log_record 信息，包含其 size
pub struct ReadLogRecord {
    pub(crate) record: LogRecord,
    pub(crate) size: usize,
}

impl LogRecord {
    pub fn encode(&mut self) -> Vec<u8> {
        todo!()
    }

    pub fn get_crc(&mut self) -> u32 {
        todo!()
    }
}

impl LogRecordType {
    pub fn from_u8(v: u8) -> Self {
        match v {
            1 => LogRecordType::NORMAL,
            2 => LogRecordType::DELETED,
            _ => panic!("unkown log record type"),
        }
    }
}

/// rust 中的处理方式是把 CRC字段放在了最后面，前面也就只有 Type,KeySize,Value_size三个字段
/// 获取 LogRecord header 部分的最大长度
pub fn max_log_record_header_size() -> usize {
    std::mem::size_of::<u8>() + length_delimiter_len(std::u32::MAX as usize * 2)
}
