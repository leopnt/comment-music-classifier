use std::fs::File;
use std::io::Read;

use std::time::Instant;

#[derive(Debug)]
struct Chunk {
    id: String,
    size: usize,
}

fn read_aiff_chunk(data: &Vec<u8>, pos: usize) -> Result<Chunk, Box<dyn std::error::Error>> {
    let id: String = String::from_utf8(data[pos..pos + 4].to_vec())?;
    let size: usize = i32::from_be_bytes(data[pos + 4..pos + 8].try_into()?) as usize;

    Ok(Chunk { id, size })
}

fn main() -> std::io::Result<()> {
    let start_time = Instant::now();

    let mut file = File::open("/Users/leopnt/Music/TCOTC/01 View Source.aiff")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let mut cursor: usize = 0;

    loop {
        if cursor >= data.len() {
            println!("Cursor reached EOF");
            break;
        }

        let chunk: Chunk = read_aiff_chunk(&data, cursor).unwrap();
        println!("cursor: {} {} {}", cursor, chunk.id, chunk.size);

        match chunk.id.as_str() {
            "FORM" => cursor += 12, // FORM is a container chunk, we jump to the first data byte of this chunk
            "COMT" => cursor += 8 + chunk.size,
            "COMM" => cursor += 8 + chunk.size,
            "SSND" => cursor += 8 + chunk.size,
            "ID3 " => {
                println!("Found ID3 Tag!");
                let elapsed_time = start_time.elapsed();
                println!("Seeking ID3 took {:?}", elapsed_time);
                break;
            }
            _ => {
                println!("Unknown identifier");
                break;
            }
        }
    }

    Ok(())
}
