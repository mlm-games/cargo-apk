use std::{
    io::{Cursor, Read, Write},
    time::{Duration, UNIX_EPOCH},
};
use zip::{write::FileOptions, CompressionMethod, DateTime, ZipArchive, ZipWriter};

pub fn normalize_zip(data: &[u8], ts: Option<u64>) -> Result<Vec<u8>, std::io::Error> {
    // Read source
    let mut src = ZipArchive::new(Cursor::new(data))?;

    // Deterministic order: lexicographic filenames
    let mut names = (0..src.len())
        .map(|i| src.by_index(i).map(|f| f.name().to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    names.sort();

    // Choose a stable DOS mtime: clamp SOURCE_DATE_EPOCH to >= 1980-01-01; or use 1980-01-01
    let ts_unix = ts
        .or_else(|| {
            std::env::var("SOURCE_DATE_EPOCH")
                .ok()
                .and_then(|s| s.parse().ok())
        })
        .unwrap_or(315532800); // 1980-01-01 UTC
    let _ = (UNIX_EPOCH + Duration::from_secs(ts_unix.saturating_sub(315532800))); // keep for clarity
    let dos_time = DateTime::from_date_and_time(1980, 1, 1, 0, 0, 0).expect("valid DOS datetime");

    let cursor = Cursor::new(Vec::with_capacity(data.len()));
    let mut writer = ZipWriter::new(cursor);

    for name in names {
        let mut file = src.by_name(&name)?;
        let method = match file.compression() {
            CompressionMethod::Stored => CompressionMethod::Stored,
            _ => CompressionMethod::Deflated,
        };

        let mut buf = Vec::with_capacity(file.size() as usize);
        std::io::copy(&mut file, &mut buf)?;

        let mut opts = FileOptions::default()
            .compression_method(method)
            .last_modified_time(dos_time);
        if file.size() > 0xFFFF_FFFF {
            opts = opts.large_file(true);
        }

        writer.start_file(name, opts)?;
        writer.write_all(&buf)?;
    }

    let cursor = writer.finish()?;
    let out = cursor.into_inner();
    Ok(out)
}
