use std::collections::HashMap;
use std::io;

use serde_bytes::ByteBuf;

fn deserialize_bytebufs() -> bincode::Result<()> {
    let example_data = [
        2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 116,
        119, 111, 1, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 111, 110, 101];

    let map: HashMap<u32, ByteBuf> = bincode::deserialize(&example_data[..])?;

    println!("{:?}", map);

    Ok(())
}

pub fn doc_test_bytebuf_main() {
    deserialize_bytebufs().unwrap();
}

use serde_bytes::Bytes;

fn print_encoded_cache() -> bincode::Result<()> {
    let mut cache = HashMap::new();
    cache.insert(3, Bytes::new(b"three"));
    cache.insert(2, Bytes::new(b"two"));
    cache.insert(1, Bytes::new(b"one"));

    bincode::serialize_into(&mut io::stdout(), &cache)
}

pub fn doc_test_byte_main() {
    print_encoded_cache().unwrap();
}
