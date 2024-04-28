use rodio::Sink;
use rodio::{source::Source, Decoder, OutputStream, OutputStreamHandle};
use std::fs::File;
use std::io::BufReader;

pub struct Sound {
    background_sink: Sink,
    effects_sink: Sink,
}

impl Sound {
    pub fn new(stream_handle: &OutputStreamHandle) -> Sound {
        let background_sink = Sink::try_new(stream_handle).unwrap();
        let file = BufReader::new(File::open("resources/test.mp3").unwrap());
        let source = Decoder::new(file).unwrap().repeat_infinite();

        stream_handle
            .play_raw(source.convert_samples())
            .expect("ERROR playing music");

        return Sound { background_sink };
    }

    pub fn play_sound_effects(&self, file_names: Vec<String>) {
        for (file_name) in file_names {
            let file_path = format!("resources/sounds/{}", file_name);
            let file = BufReader::new(File::open(file_path).unwrap());
            let source = Decoder::new(file).unwrap();
            self.effects_sink.append(source);
        }
    }
}
