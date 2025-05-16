use std::sync::Arc;

use rocket::fairing::AdHoc;
use sea_orm::DatabaseConnection;

use crate::{application::usecase::{user_usecase::UserUseCase, wrapper::user_wrapper::UserRepositoryComposite}, infrastructure::database::mysql::impl_repository::{auth_repo::AuthRepositoryImpl, gender_repo::GenderRepositoryImpl, role_repo::RoleManagementRepositoryImpl, user_repo::UserRepositoryImpl}};





pub fn init_usecase_setup(db_connection: Arc<DatabaseConnection>) -> AdHoc {
    AdHoc::on_ignite("Initialize usecases", |rocket| async move {
        // Step 1: Initialize repositories and wrap them in Arc
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

        // Step 2: Combine repositories into a single struct
        let repository_composite = Arc::new(UserRepositoryComposite {
            user_repository,
            auth_repository,
            role_repository,
            gender_repository,
        });

        // Step 3: Initialize the UserUseCase with the composite repository
        let user_usecase = Arc::new(UserUseCase::new(repository_composite));

        // Step 4: Manage the usecase and database connection in Rocket's state
        rocket
            .manage(Arc::clone(&db_connection)) // Manage the database connection
            .manage(user_usecase) // Manage the UserUseCase
    })
}