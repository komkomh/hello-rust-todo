
use crate::entities::*;

impl user::Model {
    pub fn get_id_name(&self) -> String {
        format!("{}:{}", self.id, self.name)
    }
}