use cli::tables::TableRow;

pub struct TableData<'a, T: TableRow>(&'a [u8]);