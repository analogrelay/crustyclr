use cli::MetadataSizes;
use cli::tables::{self, Table, TableReader};
use error::Error;

pub struct TableStream<'a> {
    metadata_sizes: MetadataSizes,
    module: Table<'a, tables::ModuleReader>,
    type_ref: Table<'a, tables::TypeRefReader>,
}

impl<'a> TableStream<'a> {
    pub fn new(mut data: &'a [u8]) -> Result<TableStream<'a>, Error> {
        let sizes = MetadataSizes::read(&mut data)?;

        let module = load_table::<tables::ModuleReader>(&mut data, &sizes)?;
        let type_ref = load_table::<tables::TypeRefReader>(&mut data, &sizes)?;

        Ok(TableStream {
            metadata_sizes: sizes,
            module: module,
            type_ref: type_ref,
        })
    }

    pub fn metadata_sizes(&self) -> &MetadataSizes {
        &self.metadata_sizes
    }

    pub fn module(&self) -> &Table<'a, tables::ModuleReader> {
        &self.module
    }

    pub fn type_ref(&self) -> &Table<'a, tables::TypeRefReader> {
        &self.type_ref
    }
}

fn load_table<'a, T: TableReader>(buffer: &mut &'a [u8], sizes: &MetadataSizes) -> Result<Table<'a, T>, Error> {
    let idx = T::INDEX;
    let row_count = sizes.row_count(idx);
    if row_count > 0 {
        // Create the reader
        let reader = T::new(sizes);

        // Determine the table size
        let table_size = row_count * reader.row_size();

        if table_size > buffer.len() {
            Err(Error::InvalidMetadata("There is insufficient space in the metadata stream for this table."))
        }
        else {
            // Slice out the buffer containing the data
            let table_data = &buffer[0..table_size];

            // Update the provided buffer to the remaining space
            *buffer = &buffer[table_size..];

            // Create the table
            Ok(Table::new(table_data, reader))
        }
    } else {
        Ok(Table::<T>::EMPTY)
    }
}