use std::ops::Deref;

use memmap;
use slog;

use error::Error;

use ecma355metadata;

pub struct Assembly {
    image: 
}

impl Assembly {
    pub fn load(data: memmap::Mmap, logger: &slog::Logger) -> Result<Assembly, Error> {
        let mut cursor = ::std::io::Cursor::new(data.deref());

        debug!(logger, "loading PE image...");
        let image = ecma355metadata::PeImage::read(&mut cursor)?;
        debug!(logger, "loaded PE image.");

        // Load CLI metadata
        debug!(logger, "reading CLI header...");
        let cli_header = ecma355metadata::CliHeader::from_pe_image(&image)?;
        debug!(logger, "read CLI header.");

        // Create a metadata reader
        debug!(logger, "reading metadata headers...");
        let metadata = ecma355metadata::MetadataReader::from_pe_image(&image, &cli_header)?;
        debug!(logger, "read metadata headers.");

        unimplemented!()
    }
}
