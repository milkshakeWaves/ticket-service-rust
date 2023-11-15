use serial_test::serial;
use ticket_service_rust::models::CreateTicketBody;

use super::init_db_context;

#[serial]
#[actix_rt::test]
async fn add_ticket_returns_1() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;

    let already_existing_user_event = CreateTicketBody {
        user_email: String::from("gigio@email.com"),
        event_code: String::from("MUSIC"),
    };

    let result = db
        .tickets
        .create_user_ticket(&already_existing_user_event)
        .await;

    assert!(result.is_ok());
    assert_eq!(1, result.unwrap());

    let del_res = db
        .tickets
        .delete_user_ticket(&already_existing_user_event.user_email)
        .await?;
    assert_eq!(1, del_res);

    Ok(())
}

#[actix_rt::test]
async fn get_ticket_returns_empty_ticket_when_user_does_not_exist() -> () {
    let db = init_db_context().await;

    let result = db.tickets.get_tickets_by_user("666").await;
    assert!(result.is_ok_and(|tickets| tickets.is_empty()));
}

#[actix_rt::test]
async fn get_ticket_returns_empty_ticket_when_user_didnt_buy_any_ticket() -> () {
    let db = init_db_context().await;

    let result = db.tickets.get_tickets_by_user("gigio@email.com").await;
    assert!(result.is_ok_and(|tickets| tickets.is_empty()));
}

#[serial]
#[actix_rt::test]
async fn get_tickets_by_email_returns_their_tickets_when_they_exist() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;

    let already_existing_user_event = CreateTicketBody {
        user_email: String::from("gigio@email.com"),
        event_code: String::from("MUSIC"),
    };

    let result = db
        .tickets
        .create_user_ticket(&already_existing_user_event)
        .await;

    assert!(result.is_ok());
    assert_eq!(1, result.unwrap());

    let already_existing_user_event2 = CreateTicketBody {
        user_email: String::from("gigio@email.com"),
        event_code: String::from("GALA"),
    };

    let result2 = db
        .tickets
        .create_user_ticket(&already_existing_user_event2)
        .await;

    assert!(result2.is_ok());
    assert_eq!(1, result2.unwrap());

    let tickets_result = db
        .tickets
        .get_tickets_by_user(&already_existing_user_event.user_email)
        .await;

    assert!(tickets_result.is_ok());
    let tickets = tickets_result.unwrap();
    assert!(tickets.len() > 0);

    let del_rows = db
        .tickets
        .delete_user_ticket(&already_existing_user_event.user_email)
        .await?;
    assert_eq!(2, del_rows);

    Ok(())
}

#[serial]
#[actix_rt::test]
async fn get_tickets_by_events_returns_their_tickets_when_they_exist() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;

    let already_existing_user_event = CreateTicketBody {
        user_email: String::from("gigio@email.com"),
        event_code: String::from("MUSIC"),
    };

    for _ in 0..5 {
        let result = db
            .tickets
            .create_user_ticket(&already_existing_user_event)
            .await;

        assert!(result.is_ok());
        assert_eq!(1, result.unwrap());
    }

    let tickets_result = db
        .tickets
        .get_tickets_by_event(&already_existing_user_event.event_code)
        .await;

    assert!(tickets_result.is_ok());
    let tickets = tickets_result.unwrap();
    assert_eq!(5, tickets.len());

    let del_rows = db.tickets
        .delete_user_ticket(&already_existing_user_event.user_email)
        .await?;
    assert_eq!(5, del_rows);

    Ok(())
}

#[actix_rt::test]
async fn delete_ticket_returns_0_when_ticket_does_not_exist() -> () {
    let db = init_db_context().await;
    let user_email = "666";

    let result = db.tickets.delete_user_ticket(user_email).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(0, result);
}
