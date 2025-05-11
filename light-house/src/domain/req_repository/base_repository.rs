use async_trait::async_trait; // For async traits

#[async_trait]
pub trait BaseRepository<T, ID, CreateDto, UpdateDto>
where
    T: Send + Sync, // T is the Entity type
    ID: Send + Sync + Copy, // ID is the Primary Key type (e.g., i32)
    CreateDto: Send, // DTO for creating a new entity
    UpdateDto: Send, // DTO for updating an existing entity
{
    async fn create(&self, dto: CreateDto) -> Result<T, String>; // Use Result for error handling
    async fn get_by_id(&self, id: ID) -> Result<Option<T>, String>;
    async fn get_all(&self) -> Result<Vec<T>, String>;
    async fn update(&self, id: ID, dto: UpdateDto) -> Result<Option<T>, String>;
    async fn delete(&self, id: ID) -> Result<bool, String>; // Return true if deleted, false otherwise
}