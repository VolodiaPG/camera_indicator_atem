pub struct AtemCamStatus {
    pub cam_prod: i8,
    pub cam_preview: i8
}

#[derive(Serialize)]
pub struct AtemCamStatusJson {
    pub cam_prod: i8,
    pub cam_preview: i8
}