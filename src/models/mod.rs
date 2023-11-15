mod user;
mod event;
mod ticket;
mod aggregates;

pub type User = user::User;
pub type CreateUserBody = user::CreateUserBody;
pub type LoginUser = user::LoginUser;
pub type InternalUser = user::InternalUser;

pub type Event = event::Event;
pub type CreateEventBody = event::CreateEventBody;

pub type Ticket = ticket::Ticket;
pub type CreateTicketBody = ticket::CreateTicketBody;

pub type SoldTicketPerUser = aggregates::SoldTicketPerUser;