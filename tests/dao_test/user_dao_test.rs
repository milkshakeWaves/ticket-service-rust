use ticket_service_rust::models::CreateUserBody;

use super::init_db_context;

#[actix_rt::test]
async fn add_user_returns_1() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;
    let user = CreateUserBody {
        email: String::from("gigi@email.com"),
        password: String::from("fake-password"),
    };

    let result = db.users.add_user(&user.email, &user.password).await;

    assert!(result.is_ok());
    assert_eq!(1, result.unwrap());

    Ok(())
}