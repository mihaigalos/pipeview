use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use std::sync::mpsc::Receiver;

pub fn loop_write(outfile: Option<String>, write_rx: Receiver<Vec<u8>>) -> std::io::Result<()> {
    let mut writer: Box<dyn Write> = if let Some(outfile) = outfile {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(std::io::stdout()))
    };

    loop {
        let buffer = write_rx.recv().unwrap();

        if buffer.is_empty() {
            break;
        }

        if let Err(e) = writer.write_all(&buffer) {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                return Ok(());
            }

            return Err(e);
        }
    }

    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use tempfile::NamedTempFile;
    use std::io::Read;

    #[test]
    fn test_write_small_data() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        let (tx, rx) = mpsc::channel();

        let test_data = b"Hello, World!";
        tx.send(test_data.to_vec()).unwrap();
        tx.send(Vec::new()).unwrap();

        let result = loop_write(Some(temp_path.clone()), rx);
        assert!(result.is_ok());

        let mut contents = Vec::new();
        temp_file.read_to_end(&mut contents).unwrap();
        assert_eq!(contents, test_data);
    }

    #[test]
    fn test_write_data_exactly_buffer_size() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        let (tx, rx) = mpsc::channel();

        let test_data = vec![0x42u8; 8192];
        tx.send(test_data.clone()).unwrap();
        tx.send(Vec::new()).unwrap();

        let result = loop_write(Some(temp_path.clone()), rx);
        assert!(result.is_ok());

        let mut contents = Vec::new();
        temp_file.read_to_end(&mut contents).unwrap();
        assert_eq!(contents.len(), 8192);
        assert_eq!(contents, test_data);
    }

    #[test]
    fn test_write_data_larger_than_buffer_size() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        let (tx, rx) = mpsc::channel();

        let test_data = vec![0x55u8; 10000];
        tx.send(test_data.clone()).unwrap();
        tx.send(Vec::new()).unwrap();

        let result = loop_write(Some(temp_path.clone()), rx);
        assert!(result.is_ok());

        let mut contents = Vec::new();
        temp_file.read_to_end(&mut contents).unwrap();
        assert_eq!(contents.len(), 10000);
        assert_eq!(contents, test_data);
    }

    #[test]
    fn test_write_multiple_chunks() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        let (tx, rx) = mpsc::channel();

        let chunk1 = b"First chunk";
        let chunk2 = b"Second chunk";
        let chunk3 = b"Third chunk";

        tx.send(chunk1.to_vec()).unwrap();
        tx.send(chunk2.to_vec()).unwrap();
        tx.send(chunk3.to_vec()).unwrap();
        tx.send(Vec::new()).unwrap();

        let result = loop_write(Some(temp_path.clone()), rx);
        assert!(result.is_ok());

        let mut contents = Vec::new();
        temp_file.read_to_end(&mut contents).unwrap();

        let mut expected = Vec::new();
        expected.extend_from_slice(chunk1);
        expected.extend_from_slice(chunk2);
        expected.extend_from_slice(chunk3);
        assert_eq!(contents, expected);
    }

    #[test]
    fn test_write_empty_chunks_ignored() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_str().unwrap().to_string();

        let (tx, rx) = mpsc::channel();

        let data1 = b"Hello";

        tx.send(data1.to_vec()).unwrap();
        tx.send(Vec::new()).unwrap();

        let result = loop_write(Some(temp_path.clone()), rx);
        assert!(result.is_ok());

        let mut contents = Vec::new();
        temp_file.read_to_end(&mut contents).unwrap();
        assert_eq!(contents, data1);
    }

    #[test]
    fn test_flush_ensures_data_integrity() {
        let test_cases = vec![1, 1000, 4096, 8191, 8192, 8193, 16384];

        for size in test_cases {
            let mut temp_file = NamedTempFile::new().unwrap();
            let temp_path = temp_file.path().to_str().unwrap().to_string();

            let (tx, rx) = mpsc::channel();

            let test_data = (0..size).map(|i| (i % 256) as u8).collect::<Vec<u8>>();

            tx.send(test_data.clone()).unwrap();
            tx.send(Vec::new()).unwrap();

            let result = loop_write(Some(temp_path.clone()), rx);
            assert!(result.is_ok(), "Failed for size {}", size);

            let mut contents = Vec::new();
            temp_file.read_to_end(&mut contents).unwrap();
            assert_eq!(contents.len(), size, "Size mismatch for test size {}", size);
            assert_eq!(contents, test_data, "Data mismatch for test size {}", size);
        }
    }
}
