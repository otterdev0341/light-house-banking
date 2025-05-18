use std::{ops::Deref, sync::Arc};

use uuid::Uuid;

use crate::{domain::{dto::{assest_type_dto::{ReqCreateAssetTypeDto, ReqUpdateAssestTypeDto}, asset_dto::{ReqCreateAssetDto, ReqUpdateAssetDto}, contact_dto::{ReqCreateContactDto, ReqUpdateContactDto}, expense_dto::{ReqCreateExpenseDto, ReqUpdateExpenseDto}, expense_type_dto::{ReqCreateExpenseTypeDto, ReqUpdateExpenseTypeDto}, transaction_dto::{ReqCreateTransferDto, ReqUpdateTransferDto}}, entities::{asset, asset_type, contact, expense, expense_type, transaction}, req_repository::{asset_repository::{AssetRepositoryBase, AssetRepositoryUtility}, asset_type_repository::{AssetTypeRepositoryBase, AssetTypeRepositoryUtility}, contact_repository::{ContactRepositoryBase, ContactRepositoryUtility}, expense_repository::{ExpenseRepositoryBase, ExpenseRepositoryUtill}, expense_type_repository::{ExpenseTypeRepositoryBase, ExpenseTypeRepositoryUtility}, transaction_repository::TransferRepositoryUtility}}, infrastructure::database::mysql::impl_repository::{asset_repo::AssetRepositoryImpl, asset_type_repo::AssetTypeRepositoryImpl, contact_repo::ContactRepositoryImpl, expense_repo::ExpenseRepositoryImpl, expense_type_repos::ExpenseTypeRepositoryImpl, transaction::transfer_repo::TransferRepositoryImpl}, soc::soc_repository::RepositoryError};






pub struct TransferRepositoryComposite{
    pub transfer_repository: Arc<TransferRepositoryImpl>,
    pub asset_repository: Arc<AssetRepositoryImpl>,
    pub asset_type_repository: Arc<AssetTypeRepositoryImpl>,
    pub expense_repository: Arc<ExpenseRepositoryImpl>,
    pub expense_type_repository: Arc<ExpenseTypeRepositoryImpl>,
    pub contact_repository: Arc<ContactRepositoryImpl>,
}


impl TransferRepositoryComposite{
    pub fn transfer_repository(&self) -> &TransferRepositoryImpl {
        &self.transfer_repository
    }

    pub fn asset_repository(&self) -> &AssetRepositoryImpl {
        &self.asset_repository
    }

    pub fn asset_type_repository(&self) -> &AssetTypeRepositoryImpl {
        &self.asset_type_repository
    }

    pub fn expense_repository(&self) -> &ExpenseRepositoryImpl {
        &self.expense_repository
    }

    pub fn expense_type_repository(&self) -> &ExpenseTypeRepositoryImpl {
        &self.expense_type_repository
    }

    pub fn contact_repository(&self) -> &ContactRepositoryImpl {
        &self.contact_repository
    }
}

impl Deref for TransferRepositoryComposite {
    type Target = TransferRepositoryImpl;

    fn deref(&self) -> &Self::Target {
        &self.transfer_repository
    }
}

#[async_trait::async_trait]
impl TransferRepositoryUtility for TransferRepositoryComposite {
    async fn create_transfer(&self, user_id: Uuid, transfer_dto: ReqCreateTransferDto) -> Result<transaction::Model, RepositoryError>{
        self.transfer_repository.create_transfer(user_id, transfer_dto).await
    }
    async fn update_transfer(&self, user_id: Uuid, transaction_id: Uuid, transfer_dto: ReqUpdateTransferDto) -> Result<transaction::Model, RepositoryError>
    {
        self.transfer_repository.update_transfer(user_id, transaction_id, transfer_dto).await
    }
    async fn delete_transfer(&self, user_id: Uuid, transaction_id: Uuid) -> Result<(), RepositoryError>
    {
        self.transfer_repository.delete_transfer(user_id, transaction_id).await
    }
    async fn get_transfer_by_id(&self, user_id: Uuid, transaction_id: Uuid) -> Result<Option<transaction::Model>, RepositoryError>
    {
        self.transfer_repository.get_transfer_by_id(user_id, transaction_id).await
    }
    async fn get_all_transfers_by_user(&self, user_id: Uuid) -> Result<Vec<transaction::Model>, RepositoryError>
    {
        self.transfer_repository.get_all_transfers_by_user(user_id).await
    }
}

#[async_trait::async_trait]
impl AssetRepositoryBase for TransferRepositoryComposite {
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
impl AssetRepositoryUtility for TransferRepositoryComposite {
    async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<asset::Model>, RepositoryError>
    {
        self.asset_repository.find_all_by_user_id(user_id).await
    }
}

#[async_trait::async_trait]
impl AssetTypeRepositoryBase for TransferRepositoryComposite {
    async fn create(&self, user_id: Uuid, dto: ReqCreateAssetTypeDto) -> Result<asset_type::Model, RepositoryError>
    {
        self.asset_type_repository.create(user_id, dto).await
    }
    async fn find_by_id(&self, user_id: Uuid, asset_type_id: Uuid) -> Result<Option<asset_type::Model>, RepositoryError>
    {
        self.asset_type_repository.find_by_id(user_id, asset_type_id).await
    }
    async fn find_all(&self) -> Result<Vec<asset_type::Model>, RepositoryError>
    {
        self.asset_type_repository.find_all().await
    }
    async fn update(&self, user_id: Uuid, asset_type_id: Uuid ,dto: ReqUpdateAssestTypeDto  ) -> Result<asset_type::Model, RepositoryError>
    {
        self.asset_type_repository.update(user_id, asset_type_id, dto).await
    }
    async fn delete(&self,user_id: Uuid, asset_type_id : Uuid) -> Result<(), RepositoryError>
    {
        self.asset_type_repository.delete(user_id, asset_type_id).await
    }
}

#[async_trait::async_trait]
impl AssetTypeRepositoryUtility for TransferRepositoryComposite {
    async fn is_in_use(&self, user_id: Uuid, asset_type_id: Uuid) -> Result<bool, RepositoryError>
    {
        self.asset_type_repository.is_in_use(user_id, asset_type_id).await
    }
    async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<asset_type::Model>, RepositoryError>
    {
        self.asset_type_repository.find_all_by_user_id(user_id).await
    }
}

#[async_trait::async_trait]
impl ExpenseRepositoryBase for TransferRepositoryComposite{
    async fn create(&self, user_id: Uuid, dto: ReqCreateExpenseDto) -> Result<expense::Model, RepositoryError>
    {
        self.expense_repository.create(user_id, dto).await
    }
    async fn find_by_id(&self, expense_id: Uuid) -> Result<Option<expense::Model>, RepositoryError>
    {
        self.expense_repository.find_by_id(expense_id).await
    }
    async fn find_all(&self) -> Result<Vec<expense::Model>, RepositoryError>
    {
        self.expense_repository.find_all().await
    }
    async fn update(&self, dto: ReqUpdateExpenseDto, user_id: Uuid, expense_id: Uuid) -> Result<expense::Model, RepositoryError>
    {
        self.expense_repository.update(dto, user_id, expense_id).await
    }
    async fn delete(&self,user_id: Uuid, expense_id : Uuid) -> Result<(), RepositoryError>
    {
        self.expense_repository.delete(user_id, expense_id).await
    }
}

#[async_trait::async_trait]
impl ExpenseRepositoryUtill for TransferRepositoryComposite {
    async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<expense::Model>, RepositoryError>
    {
        self.expense_repository.find_all_by_user_id(user_id).await
    }
    async fn find_by_user_id_and_expense_id(&self, user_id: Uuid, expense_id: Uuid) -> Result<Option<expense::Model>, RepositoryError>
    {
        self.expense_repository.find_by_user_id_and_expense_id(user_id, expense_id).await
    }
    async fn find_by_user_id_and_expense_type_id(&self, user_id: uuid::Uuid, expense_id: Uuid) -> Result<Option<expense::Model>, RepositoryError>
    {
        self.expense_repository.find_by_user_id_and_expense_type_id(user_id, expense_id).await
    }
    async fn is_in_use_in_transaction(&self, user_id: Uuid, expense_id: Uuid) -> Result<bool, RepositoryError>
    {
        self.expense_repository.is_in_use_in_transaction(user_id, expense_id).await
    }
}


#[async_trait::async_trait]
impl ExpenseTypeRepositoryBase for TransferRepositoryComposite {
    async fn create(&self, user_id: Uuid, dto: ReqCreateExpenseTypeDto) -> Result<expense_type::Model, RepositoryError>
    {
        self.expense_type_repository.create(user_id, dto).await
    }
    async fn find_by_id(&self, expesne_type_id: Uuid) -> Result<Option<expense_type::Model>, RepositoryError>
    {
        self.expense_type_repository.find_by_id(expesne_type_id).await
    }
    async fn find_all(&self) -> Result<Vec<expense_type::Model>, RepositoryError>
    {
        self.expense_type_repository.find_all().await
    }
    async fn update(&self, dto: ReqUpdateExpenseTypeDto, user_id: Uuid, expesne_type_id: Uuid) -> Result<expense_type::Model, RepositoryError>
    {
        self.expense_type_repository.update(dto, user_id, expesne_type_id).await
    }
    async fn delete(&self,user_id: Uuid, expesne_type_id : Uuid) -> Result<(), RepositoryError>
    {
        self.expense_type_repository.delete(user_id, expesne_type_id).await
    }
}

#[async_trait::async_trait]
impl ExpenseTypeRepositoryUtility for TransferRepositoryComposite{
    async fn is_in_use(&self, user_id: Uuid, expesne_type_id: Uuid) -> Result<bool, RepositoryError>
    {
        self.expense_type_repository.is_in_use(user_id, expesne_type_id).await
    }
    async fn find_all_by_user_id(&self, user_id: Uuid) -> Result<Vec<expense_type::Model>, RepositoryError>
    {
        self.expense_type_repository.find_all_by_user_id(user_id).await
    }
    async fn find_by_user_id_and_expense_type_id(&self, expesne_type_id: Uuid, user_id: Uuid) -> Result<Option<expense_type::Model>, RepositoryError>
    {
        self.expense_type_repository.find_by_user_id_and_expense_type_id(expesne_type_id, user_id).await
    }
}

#[async_trait::async_trait]
impl ContactRepositoryBase for TransferRepositoryComposite {
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
impl ContactRepositoryUtility for TransferRepositoryComposite {
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
}