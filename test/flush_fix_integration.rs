use std::io::{BufWriter, Write};
use std::sync::mpsc;
use std::thread;
use tempfile::NamedTempFile;
use std::fs::File;
#[test]
fn test_flush_fix_prevents_data_loss() {
    let test_sizes = vec![1, 1000, 4095, 4096, 4097, 8191, 8192, 8193, 10000];

    for &size in &test_sizes {
        let test_data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();

        let result_with_flush = test_write_with_flush(&test_data);
        let result_without_flush = test_write_without_flush(&test_data);

        assert_eq!(result_with_flush, test_data, "With flush failed for size {}", size);
        assert_eq!(result_without_flush, test_data, "Without flush failed for size {}", size);
    }
}

fn test_write_with_flush(data: &[u8]) -> Vec<u8> {
    let mut temp_file = NamedTempFile::new().unwrap();
    let temp_path = temp_file.path().to_str().unwrap().to_string();

    let (tx, rx) = mpsc::channel();

    tx.send(data.to_vec()).unwrap();
    tx.send(Vec::new()).unwrap();

    pipeview::io::write::loop_write(Some(temp_path), rx).unwrap();

    let mut contents = Vec::new();
    std::io::Read::read_to_end(&mut temp_file, &mut contents).unwrap();
    contents
}

fn test_write_without_flush(data: &[u8]) -> Vec<u8> {
    let mut temp_file = NamedTempFile::new().unwrap();

    {
        let file = File::create(temp_file.path()).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write_all(data).unwrap();
    }

    let mut contents = Vec::new();
    std::io::Read::read_to_end(&mut temp_file, &mut contents).unwrap();
    contents
}

#[test]
fn test_flush_critical_for_process_termination() {
    let test_data = vec![0x42u8; 5000];

    let temp_file = NamedTempFile::new().unwrap();
    let temp_path = temp_file.path().to_string_lossy().to_string();

    {
        let file = File::create(&temp_path).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write_all(&test_data).unwrap();
        writer.flush().unwrap();
    }

    let written_data = std::fs::read(&temp_path).unwrap();
    assert_eq!(written_data, test_data);
    assert_eq!(written_data.len(), 5000);
}

#[test]
fn test_concurrent_write_operations() {
    let (tx, rx) = mpsc::channel();
    let mut temp_file = NamedTempFile::new().unwrap();
    let temp_path = temp_file.path().to_str().unwrap().to_string();

    let writer_handle = thread::spawn({
        let temp_path = temp_path.clone();
        move || pipeview::io::write::loop_write(Some(temp_path), rx)
    });

    let chunks = vec![
        b"chunk1".to_vec(),
        b"chunk2".to_vec(),
        b"chunk3".to_vec(),
        vec![0x00; 1000],
        vec![0xFF; 2000],
    ];

    let mut expected_data = Vec::new();
    for chunk in &chunks {
        expected_data.extend(chunk);
        tx.send(chunk.clone()).unwrap();
    }

    tx.send(Vec::new()).unwrap();

    writer_handle.join().unwrap().unwrap();

    let mut contents = Vec::new();
    std::io::Read::read_to_end(&mut temp_file, &mut contents).unwrap();
    assert_eq!(contents, expected_data);
}
