use chrono::Utc;
use ticket_service_rust::models::CreateEventBody;

use super::init_db_context;

#[actix_rt::test]
async fn add_event_returns_1() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;
    
    let event_code = String::from("EV0");
    let event_req = CreateEventBody {
        code: event_code.clone(),
        description: String::from("This is a big event"),
        place: String::from("London"),
        available_seats: 250,
        price: 25,
        date: Utc::now(),
    };

    let result = db.events.add_event(&event_req).await;

    assert!(result.is_ok());
    assert_eq!(1, result.unwrap());

    let del_res = db.events.delete_event(&event_code).await?;
    assert_eq!(1, del_res);

    Ok(())
}

#[actix_rt::test]
async fn get_event_by_code_returns_empty_event_when_event_does_not_exist() -> () {
    let db = init_db_context().await;

    let result = db.events.get_event_by_code("666").await;
    assert!(result.is_ok_and(|event_opt| event_opt.is_none()));
}

#[actix_rt::test]
async fn get_event_by_place_returns_event_when_event_exists() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;

    let event_code = String::from("EV1");
    let event_req = CreateEventBody {
        code: event_code.clone(),
        description: String::from("This is a big event"),
        place: String::from("Milan"),
        available_seats: 250,
        price: 25,
        date: Utc::now(),
    };

    let add_query_res = db.events.add_event(&event_req).await;

    assert!(add_query_res.is_ok());
    assert_eq!(1, add_query_res.unwrap());

    let query_res = db.events.get_event_by_place("Milan").await;
    assert!(query_res.is_ok());
    let event_option = query_res.unwrap();
    assert!(event_option.is_some());
    
    let event = event_option.unwrap();
    assert_eq!(event.description, event_req.description);
    assert_eq!(event.place, event_req.place);
    assert_eq!(event.available_seats, event_req.available_seats);
    assert_eq!(event.price, event_req.price);

    let del_res = db.events.delete_event(&event_code).await?;
    assert_eq!(1, del_res);

    Ok(())
}

#[actix_rt::test]
async fn get_event_by_code_returns_event_when_event_exists() -> Result<(), sqlx::Error> {
    let db = init_db_context().await;

    let event_code = String::from("EV2");
    let event_req = CreateEventBody {
        code: event_code.clone(),
        description: String::from("This is a big event"),
        place: String::from("Milan"),
        available_seats: 250,
        price: 25,
        date: Utc::now(),
    };

    let add_query_res = db.events.add_event(&event_req).await;

    assert!(add_query_res.is_ok());
    assert_eq!(1, add_query_res.unwrap());

    let query_res = db.events.get_event_by_code(&event_code).await;
    assert!(query_res.is_ok());
    let event_option = query_res.unwrap();
    assert!(event_option.is_some());
    
    let event = event_option.unwrap();
    assert_eq!(event.description, event_req.description);
    assert_eq!(event.place, event_req.place);
    assert_eq!(event.available_seats, event_req.available_seats);
    assert_eq!(event.price, event_req.price);

    let del_res = db.events.delete_event(&event_code).await?;
    assert_eq!(1, del_res);

    Ok(())
}

#[actix_rt::test]
async fn delete_event_returns_0_when_event_does_not_exist() -> () {
    let db = init_db_context().await;
    let event_code = "666";

    let result = db.events.delete_event(event_code).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(0, result);
}

#[actix_rt::test]
async fn get_all_events_return_all_events() -> () {
    let db = init_db_context().await;

    let result = db.events.get_all_events().await;
    assert!(result.is_ok());
    let result = result.unwrap();

    assert!(result.len() > 0);
}