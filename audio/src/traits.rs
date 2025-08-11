//maybe I will remove them idk
pub mod marker {
    use crate::output_handle::{output_markers};

    pub trait OutputHandlerState {}
    impl OutputHandlerState for output_markers::OutputDisabled {}
    impl OutputHandlerState for output_markers::OutputEnabled {}
}
