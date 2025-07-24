pub struct GameData {
    data: Box<dyn DataTrait>
}

//marker trait
pub trait DataTrait {}
