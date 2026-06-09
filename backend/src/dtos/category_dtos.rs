use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Clone, Serialize, Deserialize)]
pub struct CategoryCreationDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}
