pub mod websocket;

#[derive(RustcEncodable, RustcDecodable)]
pub struct Mat {
    pub id: u32,
    pub level: f32,
    pub beer_on_mat: bool
}
