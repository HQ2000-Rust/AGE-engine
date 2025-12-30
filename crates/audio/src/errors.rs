

//use map_err here

#[derive(Debug)]
pub enum AudioError {
    IoError(std::io::Error),
    DecoderError(rodio::decoder::DecoderError),
    OutputStreamBuilderError(rodio::StreamError),
    StreamError(rodio::StreamError),
    FileNotLoaded(String),
}
