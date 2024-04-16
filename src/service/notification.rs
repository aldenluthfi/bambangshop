use std::thread;

use bambangshop::{Result, compose_error_response};
use rocket::http::Status;
use rocket::response::status::NotFound;
use crate::model::notification::Notification;
use crate::model::product::Product;
use crate::repository::subscriber::SubscriberRepository;

pub struct NotificationService;

impl NotificationService{
}