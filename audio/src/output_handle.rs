use super::errors::AudioError;
use super::output_handle::output_markers::{OutputDisabled, OutputEnabled};
use super::traits::marker::OutputHandlerState;
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use std::marker::PhantomData;
use std::path::Path;

use pyo3::prelude::*;

pub mod output_markers {
    pub struct OutputDisabled;
    pub struct OutputEnabled;
}

#[pyclass]
pub struct OutputHandle<O> {
    pub(super) stream: Option<OutputStream>,
    pub(super) sink: Option<Sink>,
    pub(super) loaded_files: HashMap<String, Cursor<Vec<u8>>>,
    _marker: PhantomData<O>,
}

impl Default for OutputHandle<OutputDisabled> {
    fn default() -> OutputHandle<OutputDisabled> {
        OutputHandle {
            stream: None,
            sink: None,
            loaded_files: HashMap::new(),
            _marker: PhantomData,
        }
    }
}

impl<O: OutputHandlerState> OutputHandle<O> {
    //I'm not that good of a Rust dev, I saw this type trick in the stdlib
    pub fn load_file<P: AsRef<Path>>(
        &mut self,
        path: P,
        new_name: String,
    ) -> Result<(), AudioError> {
        //capacity: 50 Kilobytes, every sound effect has at least this size...
        let mut buffer = Vec::with_capacity(1024 * 50);
        let mut file = File::open(path).map_err(AudioError::IoError)?;
        file.read_to_end(&mut buffer).map_err(AudioError::IoError)?;
        let data = Cursor::new(buffer);
        self.loaded_files.insert(new_name, data);
        Ok(())
    }

    //important: NON-fatal error, can be tried anytime, Result can also be safely ignored - maybe another type??
    pub fn unload_file(&mut self, name: String) -> Result<(), AudioError> {
        if self.loaded_files.remove(&name).is_none() {
            return Err(AudioError::FileNotLoaded(name));
        }
        Ok(())
    }
    pub fn get_all_loaded_files(&self) -> Vec<&String> {
        self.loaded_files.keys().collect::<Vec<&String>>()
    }
}

impl OutputHandle<OutputDisabled> {
    //NEVER set the handle to Some(_) here!!!

    pub fn new() -> OutputHandle<OutputDisabled> {
        Default::default()
    }

    pub fn activate_output(self) -> Result<OutputHandle<OutputEnabled>, AudioError> {
        let builder = OutputStreamBuilder::from_default_device()
            .map_err(AudioError::OutputStreamBuilderError)?;
        let stream = builder.open_stream().map_err(AudioError::StreamError)?;
        let mut result = OutputHandle::<OutputEnabled> {
            stream: Some(stream),
            sink: None,
            loaded_files: self.loaded_files,
            _marker: PhantomData,
        };
        let stream_ref = result.stream.as_ref().expect("Just set it to Some(_)");
        let mixer = stream_ref.mixer();

        result.sink = Some(Sink::connect_new(mixer));
        Ok(result)
    }
}

//important!
//NO get_device (but get_device_name) because it instantly gives an error if you try to use a device that just can't work for whatever reason
//meaning you directly get an error
impl OutputHandle<OutputEnabled> {
    pub fn disable_output(self) -> OutputHandle<OutputDisabled> {
        OutputHandle::<OutputDisabled> {
            stream: None,
            sink: None,
            loaded_files: self.loaded_files,
            _marker: PhantomData,
        }
    }

    pub fn play_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), AudioError> {
        let reader = BufReader::new(File::open(path).map_err(AudioError::IoError)?);
        let decoder = Decoder::new(reader).map_err(AudioError::DecoderError)?;
        self.sink
            .as_mut()
            .expect("Not 'None' if the output is activated")
            .append(decoder);
        Ok(())
    }

    pub fn play_loaded(&mut self, name: String) -> Result<(), AudioError> {
        let data = match self.loaded_files.get(&name) {
            Some(data) => data.clone(),
            None => return Err(AudioError::FileNotLoaded(name)),
        };
        let decoder = Decoder::new(data).map_err(AudioError::DecoderError)?;
        self.sink
            .as_mut()
            .expect("Basically never 'None' if output is enabled")
            .append(decoder);
        Ok(())
    }

    pub fn pause(&mut self) {
        self.sink
            .as_mut()
            .expect("Basically never 'None' if output is enabled")
            .pause();
    }

    pub fn play(&mut self) {
        self.sink
            .as_mut()
            .expect("Basically never 'None' if output is enabled")
            .play();
    }

    pub fn is_paused(&self) -> bool {
        //again, match to destructure without taking ownership
        self.sink
            .as_ref()
            .expect("Basically never 'None' if output is enabled")
            .is_paused()
    }
}
