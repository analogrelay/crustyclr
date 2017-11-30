use cli::tables::{TableDecoder, TableIndex};

pub trait Table {
    type Decoder: TableDecoder;
    const INDEX: TableIndex;
}
