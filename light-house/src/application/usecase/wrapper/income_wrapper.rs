use std::{ops::Deref, sync::Arc};

use uuid::Uuid;

use crate::{domain::{dto::{asset_dto::{ReqCreateAssetDto, ReqUpdateAssetDto}, contact_dto::{ReqCreateContactDto, ReqUpdateContactDto}, transaction_dto::{ReqCreateIncomeDto, ReqUpdateIncomeDto}}, entities::{asset, asset_type, contact, contact_type, transaction}, req_repository::{asset_repository::{AssetRepositoryBase, AssetRepositoryUtility}, contact_repository::{ContactRepositoryBase, ContactRepositoryUtility}, transaction_repository::RecordIncomeRepositoryUtility}}, infrastructure::database::mysql::impl_repository::{asset_repo::AssetRepositoryImpl, contact_repo::ContactRepositoryImpl, transaction::income_repo::IncomeRepositoryImpl}, soc::soc_repository::RepositoryError};






pub struct IncomeRepositoryComposite {
    pub income_repository: Arc<IncomeRepositoryImpl>,
    pub asset_repository: Arc<AssetRepositoryImpl>,
    pub contact_repository: Arc<ContactRepositoryImpl>,
    
}


impl IncomeRepositoryComposite{
    pub fn income_repository(&self) -> &Arc<IncomeRepositoryImpl>{
        &self.income_repository
    }

    pub fn asset_repository(&self) -> &Arc<AssetRepositoryImpl>{
        &self.asset_repository
    }
    pub fn contact_repository(&self) -> &Arc<ContactRepositoryImpl>{
        &self.contact_repository
    }

}

impl Deref for IncomeRepositoryComposite {
    type Target = IncomeRepositoryImpl;

    fn deref(&self) -> &Self::Target {
        &self.income_repository
    }
}


#[async_trait::async_trait]
impl RecordIncomeRepositoryUtility for IncomeRepositoryComposite {
    async fn create_income_record(&self, user_id: Uuid, income_record_dto: ReqCreateIncomeDto) -> Result<transaction::Model, RepositoryError>
    {
        self.income_repository.create_income_record(user_id, income_record_dto).await
    }
    async fn update_income_record(&self, user_id: Uuid, transaction_id: Uuid, income_record_dto: ReqUpdateIncomeDto) -> Result<transaction::Model, RepositoryError>
    {
        self.income_repository.update_income_record(user_id, transaction_id, income_record_dto).await
    }
    async fn delete_income_record(&self, user_id: Uuid, transaction_id: Uuid) -> Result<(), RepositoryError>
    {
        self.income_repository.delete_income_record(user_id, transaction_id).await
    }
    async fn get_income_record_by_id(&self, user_id: Uuid, transaction_id: Uuid) -> Result<Option<transaction::Model>, RepositoryError>
    {
        self.income_repository.get_income_record_by_id(user_id, transaction_id).await
    }
    async fn get_all_income_record_by_user(&self, user_id: Uuid) -> Result<Vec<transaction::Model>, RepositoryError>
    {
        self.income_repository.get_all_income_record_by_user(user_id).await
    }
}


#[async_trait::async_trait]
impl AssetRepositoryBase for IncomeRepositoryComposite {
    async fn create(&self, user_id: Uuid, dto: ReqCreateAssetDto) -> Result<asset::Model, RepositoryError>
    {
        self.asset_repository.create(user_id, dto).await
    }
    async fn find_by_id(&self, user_id: Uuid, asset_id: Uuid) -> Result<Option<asset::Model>, RepositoryError>
    {
        self.asset_repository.find_by_id(user_id, asset_id).await
    }
    async fn find_all(&self) -> Result<Vec<asset::Model>, RepositoryError>
    {
        self.asset_repository.find_all().await
    }
    async fn update(&self, dto: ReqUpdateAssetDto, user_id: Uuid, asset_id: Uuid) -> Result<asset::Model, RepositoryError>
    {
        self.asset_repository.update(dto, user_id, asset_id).await
    }
    async fn delete(&self,user_id: Uuid, asset_id : Uuid) -> Result<(), RepositoryError>
    {
        self.asset_repository.delete(user_id, asset_id).await
    }
}

#[async_trait::async_trait]
impl AssetRepositoryUtility for IncomeRepositoryComposite{
    async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<asset::Model>, RepositoryError>
    {
        self.asset_repository.find_all_by_user_id(user_id).await
    }
    async fn find_by_user_and_asset_type_id(&self, asset_id: Uuid, user_id: Uuid) -> Result<Option<asset_type::Model>, RepositoryError>
    {
        self.asset_repository.find_by_user_and_asset_type_id(asset_id, user_id).await
    }
}


#[async_trait::async_trait]
impl ContactRepositoryBase for IncomeRepositoryComposite{
    async fn create(&self, user_id: Uuid, dto: ReqCreateContactDto) -> Result<contact::Model, RepositoryError>
    {
        self.contact_repository.create(user_id, dto).await
    }
    async fn find_by_id(&self, contact_id: Uuid) -> Result<Option<contact::Model>, RepositoryError>
    {
        self.contact_repository.find_by_id(contact_id).await
    }
    async fn find_all(&self) -> Result<Vec<contact::Model>, RepositoryError>
    {
        self.contact_repository.find_all().await
    }
    async fn update(&self, dto: ReqUpdateContactDto, user_id: Uuid, contact_id: Uuid) -> Result<contact::Model, RepositoryError>
    {
        self.contact_repository.update(dto, user_id, contact_id).await
    }
    async fn delete(&self,user_id: Uuid, contact_id : Uuid) -> Result<(), RepositoryError>
    {
        self.contact_repository.delete(user_id, contact_id).await
    }
}

#[async_trait::async_trait]
impl ContactRepositoryUtility for IncomeRepositoryComposite{
    async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<contact::Model>, RepositoryError>
    {
        self.contact_repository.find_all_by_user_id(user_id).await
    }
    async fn find_by_user_id_and_contact_id(&self, user_id: Uuid, contact_id: Uuid) -> Result<Option<contact::Model>, RepositoryError>
    {
        self.contact_repository.find_by_user_id_and_contact_id(user_id, contact_id).await
    }
    async fn find_by_user_id_and_contact_type_id(&self, user_id: uuid::Uuid, contact_id: Uuid) -> Result<Option<contact::Model>, RepositoryError>
    {
        self.contact_repository.find_by_user_id_and_contact_type_id(user_id, contact_id).await
    }
    async fn is_in_use_in_transaction(&self, user_id: Uuid, contact_id: Uuid) -> Result<bool, RepositoryError>
    {
        self.contact_repository.is_in_use_in_transaction(user_id, contact_id).await
    }
    async fn find_contact_type_by_id(&self, user_id: Uuid, contact_type_id: Uuid) -> Result<Option<contact_type::Model>, RepositoryError>
    {
        self.contact_repository.find_contact_type_by_id(user_id, contact_type_id).await
    }
}



