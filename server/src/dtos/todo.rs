use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use crate::entities::sea_orm_active_enums::Status;

#[derive(Default)]
pub struct SearchParams {
    pub(crate) title: Option<String>,
    pub(crate) contents: Option<String>,
    pub(crate) status: Option<Status>,
    pub(crate) stated_date_from: Option<NaiveDate>,
    pub(crate) stated_date_to: Option<NaiveDate>,
    pub(crate) ended_date_from: Option<NaiveDate>,
    pub(crate) ended_date_to: Option<NaiveDate>,
    pub(crate) rank_from: Option<Decimal>,
    pub(crate) rank_to: Option<Decimal>,
    pub(crate) updated_at_from: Option<NaiveDateTime>,
    pub(crate) updated_at_to: Option<NaiveDateTime>,
    pub(crate) created_at_from: Option<NaiveDateTime>,
    pub(crate) created_at_to: Option<NaiveDateTime>,
    pub(crate) user_id: i32,
}
