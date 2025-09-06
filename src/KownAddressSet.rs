use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// 管理字符串集合的结构体
pub struct StringSet {
    data: HashSet<String>,
}

impl StringSet {
    /// 从文本文件初始化（按行分割）
    pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut data = HashSet::new();
        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();
            if data.contains(trimmed) {
                println!("found duplicate: {}", trimmed);
            }
            if !trimmed.is_empty() {
                data.insert(trimmed.to_string());
            }
        }

        Ok(Self { data })
    }

    /// 检查某个字符串是否在集合中
    pub fn contains(&self, s: &str) -> bool {
        self.data.contains(s)
    }

    /// 获取集合大小（可选）
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
