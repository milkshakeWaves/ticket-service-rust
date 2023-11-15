use super::init_db_context;

#[actix_rt::test]
async fn add_user_returns_1() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;
    let email = "gigi@email.com";
    let password = "fake-password";

    let result = db.users.add_user(email, password).await;

    assert!(result.is_ok());
    assert_eq!(1, result.unwrap());

    let del_res = db.users.delete_user(email).await?;
    assert_eq!(1, del_res);

    Ok(())
}

#[actix_rt::test]
async fn add_user_returns_err_when_duplicate_email_is_added(
) -> Result<(), sqlx::Error> {
    let db = init_db_context().await;
    
    let email = "bob@email.com";
    let password = "password";

    let result = db.users.add_user(email, password).await?;
    assert_eq!(1, result);

    let result = db.users.add_user(email, password).await;
    assert!(result.is_err());

    let del_res = db.users.delete_user(email).await?;
    assert_eq!(1, del_res);

    Ok(())
}

#[actix_rt::test]
async fn get_user_by_email_returns_empty_user_when_user_does_not_exist() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;

    let result = db.users.get_user_by_email("non_existing@email.com").await;
    assert!(result.is_ok_and(|user_opt| user_opt.is_none()));

    Ok(())
}

#[actix_rt::test]
async fn get_user_by_email_returns_user_when_user_exists() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;
        
    let email = "email@email.com";
    let password = "password";

    let _ = db.users.add_user(email, password).await?;

    let result = db.users.get_user_by_email(email).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.is_some_and(|r| email == r.email));

    let del_res = db.users.delete_user(email).await?;
    assert_eq!(1, del_res);

    Ok(())
}

#[actix_rt::test]
async fn delete_user_returns_0_when_user_does_not_exist() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;
    let email = "fake@email.com";

    let result = db.users.delete_user(email).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(0, result);

    Ok(())
}