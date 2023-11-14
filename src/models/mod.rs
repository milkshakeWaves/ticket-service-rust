mod user;
mod event;

pub type User = user::User;
pub type CreateUserBody = user::CreateUserBody;
pub type LoginUser = user::LoginUser;
pub type InternalUser = user::InternalUser;

pub type Event = event::Event;
pub type CreateEventBody = event::CreateEventBody;