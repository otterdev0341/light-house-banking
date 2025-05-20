use std::sync::Arc;

use rocket::fairing::AdHoc;
use sea_orm::DatabaseConnection;

use crate::{application::usecase::{asset_type_usecase::AssetTypeUseCase, asset_usecase::AssetUseCase, contact_type_usecase::ContactTypeUseCase, contact_usecase::ContactUseCase, current_sheet_usecase::CurrentUseCase, expense_type_usecase::ExpenseTypeUseCase, expense_usecase::ExpenseUseCase, transaction::{income_usecase::{self}, transaction_type_usecase::TransactionTypeUseCase}, user_usecase::UserUseCase, wrapper::{income_wrapper::IncomeRepositoryComposite, payment_wrapper::PaymentRepositoryComposite, transfer_wrapper::TransferRepositoryComposite, user_wrapper::UserRepositoryComposite}}, infrastructure::database::mysql::impl_repository::{asset_repo::AssetRepositoryImpl, asset_type_repo::AssetTypeRepositoryImpl, auth_repo::AuthRepositoryImpl, balance_repo::BalanceRepositoryImpl, contact_repo::ContactRepositoryImpl, contact_type_repo::ContactTypeRepositoryImpl, expense_repo::ExpenseRepositoryImpl, expense_type_repos::ExpenseTypeRepositoryImpl, gender_repo::GenderRepositoryImpl, role_repo::RoleManagementRepositoryImpl, transaction::{income_repo::IncomeRepositoryImpl, payment_repo::PaymentRepositoryImpl, transfer_repo::TransferRepositoryImpl}, transaction_type_repo::TransactionTypeRepositoryImpl, user_repo::UserRepositoryImpl}};





pub fn init_usecase_setup(db_connection: Arc<DatabaseConnection>) -> AdHoc {
    AdHoc::on_ignite("Initialize usecases", |rocket| async move {
        // user repositories && user usecase
        let user_repository = Arc::new(UserRepositoryImpl {
            db_pool: Arc::clone(&db_connection),
        });
        let auth_repository = Arc::new(AuthRepositoryImpl {
            db_pool: Arc::clone(&db_connection),
        });
        let role_repository = Arc::new(RoleManagementRepositoryImpl {
            db_pool: Arc::clone(&db_connection),
        });
        let gender_repository = Arc::new(GenderRepositoryImpl {
            db_pool: Arc::clone(&db_connection),
        });

        
        let repository_composite = Arc::new(UserRepositoryComposite {
            user_repository,
            auth_repository,
            role_repository,
            gender_repository,
        });

        
        let user_usecase = Arc::new(UserUseCase::new(repository_composite));

        // asset type repositories && asset type usecase
        let asset_type_repository = AssetTypeRepositoryImpl {
            db_pool: Arc::clone(&db_connection),
        };
      
        let asset_type_usecase: Arc<AssetTypeUseCase<AssetTypeRepositoryImpl>> = Arc::new(AssetTypeUseCase::new(Arc::new(asset_type_repository)));
        
        // asset repository && asset usecase
        let asset_repository = AssetRepositoryImpl{
            db_pool: Arc::clone(&db_connection),
        };

        let asset_repository = Arc::new(AssetUseCase::new(Arc::new(asset_repository)));

        // expense type repositories && expense type usecase
        let expense_type_repository = ExpenseTypeRepositoryImpl{
            db_pool: Arc::clone(&db_connection),
        };
        let expense_type_usecase = Arc::new(ExpenseTypeUseCase::new(Arc::new(expense_type_repository)));

        // expense repository && expense usecase
        let expense_repository = ExpenseRepositoryImpl {
            db_pool: Arc::clone(&db_connection),
        };
        let expense_usecase = Arc::new(ExpenseUseCase::new(Arc::new(expense_repository)));

        // contact type repository && contact type usecase
        let contact_type_repository = ContactTypeRepositoryImpl{
            db_pool: Arc::clone(&db_connection),
        };
        let contact_type_usecase = Arc::new(ContactTypeUseCase::new(Arc::new(contact_type_repository)));

        // contact repository && contact usecase
        let contact_repository = ContactRepositoryImpl{
            db_pool: Arc::clone(&db_connection),
        };
        let contact_usecase = Arc::new(ContactUseCase::new(Arc::new(contact_repository)));
        // transaction type repository && transaction type usecase
        let transaction_type_repository = TransactionTypeRepositoryImpl {
            db_pool: Arc::clone(&db_connection),
        };
        let transaction_type_usecase = Arc::new(TransactionTypeUseCase::new(Arc::new(transaction_type_repository)));

        // tranfer repository && transaction usecase
        let the_transfer_repository = Arc::new(TransferRepositoryImpl{
            db_pool: Arc::clone(&db_connection),
        });
        let the_asset_repository = Arc::new(AssetRepositoryImpl{
            db_pool: Arc::clone(&db_connection),
        });
        let the_asset_type_repository = Arc::new(AssetTypeRepositoryImpl{
            db_pool: Arc::clone(&db_connection),
        });
        let the_expense_repository = Arc::new(ExpenseRepositoryImpl{
            db_pool: Arc::clone(&db_connection),
        });
        let the_expense_type_repository = Arc::new(ExpenseTypeRepositoryImpl{
            db_pool: Arc::clone(&db_connection),
        });
        let the_contact_repository = Arc::new(ContactRepositoryImpl{
            db_pool: Arc::clone(&db_connection),
        });
        let tranfer_composite = Arc::new(TransferRepositoryComposite {
            transfer_repository: the_transfer_repository,
            asset_repository: the_asset_repository.clone(),
            asset_type_repository: the_asset_type_repository.clone(),
            expense_repository: the_expense_repository,
            expense_type_repository: the_expense_type_repository,
            contact_repository: the_contact_repository.clone(),
        });

        // income repository && income usecase
        let the_income_repository = Arc::new(IncomeRepositoryImpl{
            db_pool: Arc::clone(&db_connection),
        });
  
        let income_composit = Arc::new(IncomeRepositoryComposite{
            income_repository: the_income_repository,
            asset_repository: the_asset_repository.clone(),
            contact_repository: the_contact_repository.clone(),
        });
        let transaction_type_repository = Arc::new(TransactionTypeRepositoryImpl {
            db_pool: Arc::clone(&db_connection),
        });

        let income_usecase = Arc::new(income_usecase::IncomeUseCase::new(
            income_composit.clone(),
            the_asset_repository.clone(),
            the_contact_repository.clone(),
            transaction_type_repository.clone(),
        ));

        // balance repository && balance usecase
        let the_balance_repository = Arc::new(BalanceRepositoryImpl{
            db_pool: Arc::clone(&db_connection),
        });
        
        let current_usecase = Arc::new(CurrentUseCase::new(
            the_balance_repository.clone(),
            the_asset_repository.clone()
        ));

        // payment repository && payment usecase
        let the_payment_repository = Arc::new(PaymentRepositoryImpl{
            db_pool: Arc::clone(&db_connection),
        });
        let payment_composit = Arc::new(PaymentRepositoryComposite{
            payment_repository: the_payment_repository,
            asset_repository: the_asset_repository.clone(),
            contact_repository: the_contact_repository.clone(),
        });

        // >>>>>  Manage the usecase and database connection in Rocket's state <<<<<
        rocket
            .manage(Arc::clone(&db_connection)) // Manage the database connection
            .manage(user_usecase) // Manage the UserUseCase
            .manage(asset_type_usecase) // Manage the AssetTypeUseCase
            .manage(asset_repository) // Manage the AssetUseCase
            .manage(expense_type_usecase)
            .manage(expense_usecase)
            .manage(contact_type_usecase)
            .manage(contact_usecase)
            .manage(transaction_type_usecase)
            .manage(tranfer_composite)
            .manage(income_usecase)
            .manage(payment_composit)
            .manage(current_usecase)
    })      
}