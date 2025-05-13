use uuid::Uuid;


pub struct BinaryUuid(pub Vec<u8>);


impl From<Uuid> for BinaryUuid {
    fn from(uuid: Uuid) -> Self {
        BinaryUuid(uuid.as_bytes().to_vec())
    }
}