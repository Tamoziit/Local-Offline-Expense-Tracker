use uuid::Uuid;

pub fn get_id() -> Uuid {
    Uuid::new_v4()
}
