use crate::constants::BUFFER_SIZE;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use std::sync::mpsc::Sender;

pub fn loop_read(
    infile: Option<String>,
    stats_tx: Sender<usize>,
    write_tx: Sender<Vec<u8>>,
) -> std::io::Result<()> {
    let mut reader: Box<dyn Read> = if let Some(infile) = infile {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(std::io::stdin()))
    };

    let mut buffer = [0; BUFFER_SIZE];

    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        let _ = stats_tx.send(num_read);

        if write_tx.send(Vec::from(&buffer[..num_read])).is_err() {
            break;
        }
    }

    let _ = stats_tx.send(0);
    let _ = write_tx.send(Vec::new());

    Ok(())
}
