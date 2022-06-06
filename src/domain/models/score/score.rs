pub struct Score {
    pub uuid: String,
    pub stage_id: i32,
    pub clear_time: f32,
    pub user_name: String,
}

impl Score {
    pub fn new(uuid: String,stage_id: i32, clear_time: f32, user_name: String) -> Self {
        Self {
            uuid,
            stage_id,
            clear_time,
            user_name,
        }
    }
}
