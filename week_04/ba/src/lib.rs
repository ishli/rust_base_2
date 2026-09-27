use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

pub const BUFFER_SIZE: usize = 64 * 1024;

// -----------------------------------------------------------------------------
// MyBufReader
// -----------------------------------------------------------------------------

#[allow(dead_code)]
pub struct MyBufReader {
    buffer: Vec<u8>,
    current_size: usize,
    offset: usize,
    file: File,
}

impl MyBufReader {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = File::open(path)?;
        return Ok(Self {
            buffer: vec![0_u8; BUFFER_SIZE],
            current_size: 0,
            offset: 0,
            file: file,
        });
    }

    pub fn read_byte(&mut self) -> io::Result<Option<u8>> {
        if self.current_size == self.offset {
            self.offset = 0;
            self.current_size = 0;
            self.current_size = self.file.read(&mut self.buffer)?;
            if self.current_size == 0 {
                return Ok(None);
            }
        }

        let result: u8 = self.buffer[self.offset];
        self.offset += 1;
        
        return Ok(Some(result));
    }
}

// -----------------------------------------------------------------------------
// MyBufWriter
// -----------------------------------------------------------------------------

pub struct MyBufWriter {
    buffer: Vec<u8>,
    file: File,
}

impl MyBufWriter {
    pub fn create(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = File::create(path)?;

        return Ok(Self {
            buffer: Vec::with_capacity(BUFFER_SIZE),
            file: file,
        });
    }

    pub fn write_buffered(&mut self, data: &[u8]) -> io::Result<()> {
        let mut slice_begin: usize = 0;

        while slice_begin < data.len() {
            let mut current = BUFFER_SIZE - self.buffer.len();

            if current == 0 {
                self.flush();
                current = BUFFER_SIZE;
            }

            let offset = current.min(data.len() - slice_begin);
            let slice_end = slice_begin + offset;

            self.buffer.extend_from_slice(&data[slice_begin..slice_end]);

            slice_begin += offset;
        }

        return Ok(());
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.file.write_all(&self.buffer)?;
        self.buffer.clear();

        return Ok(());
    }

    pub fn close(mut self) -> io::Result<()> {
        return self.flush();
    }
}

impl Drop for MyBufWriter {
    fn drop(&mut self) {
        // Ошибку из Drop вернуть нельзя.
        // Поэтому в реальном коде лучше явно вызывать close() или flush().
        let _ = self.flush();
    }
}

// -----------------------------------------------------------------------------
// Медленная версия
// -----------------------------------------------------------------------------

pub fn copy_slow(input: impl AsRef<Path>, output: impl AsRef<Path>) -> io::Result<u64> {
    let mut input = File::open(input)?;
    let mut output = File::create(output)?;

    let mut copied = 0;
    let mut byte = [0u8; 1];

    loop {
        let n = input.read(&mut byte)?;
        if n == 0 {
            break;
        }

        output.write_all(&byte[..n])?;
        copied += n as u64;
    }

    output.flush()?;

    Ok(copied)
}

// -----------------------------------------------------------------------------
// Быстрая версия
// -----------------------------------------------------------------------------
// copy_fast специально тоже использует побайтный API.
// Разница должна быть не в коде копирования, а в реализации MyBufReader и MyBufWriter
// эту функцию не нужно менять, она должна работать с любыми реализациями MyBufReader и MyBufWriter,
// которые вы сделаете
pub fn copy_fast(input: impl AsRef<Path>, output: impl AsRef<Path>) -> io::Result<u64> {
    let mut reader = MyBufReader::open(input)?;
    let mut writer = MyBufWriter::create(output)?;

    let mut copied = 0;

    while let Some(byte) = reader.read_byte()? {
        writer.write_buffered(&[byte])?;
        copied += 1;
    }

    writer.close()?;

    Ok(copied)
}

pub const RECORD_SIZE: usize = 10;

pub fn make_record(index: usize) -> [u8; RECORD_SIZE] {
    let mut record = [0u8; RECORD_SIZE];

    (0..RECORD_SIZE).for_each(|i| {
        record[i] = ((index + i) % 251) as u8;
    });

    record
}

pub fn generate_input_file(path: impl AsRef<Path>, records: usize) -> io::Result<()> {
    let mut file = File::create(path)?;

    for i in 0..records {
        let record = make_record(i);
        file.write_all(&record)?;
    }

    file.flush()?;

    Ok(())
}
