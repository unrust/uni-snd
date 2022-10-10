use uni_app::AppEvent;

extern crate uni_app;
extern crate uni_snd;

// this block is needed to bootstrap the program on wasm32 target
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    main();
    Ok(())
}

// this is the structure generating the sound
struct Oscillator {
    sample_rate: f32,
    t: f32,
    left: bool,
    freq: f32,
    volume: f32,
}

// the is the kind of event you can send to your generator to alter its behaviour
struct Event {
    freq: f32,
    volume: f32,
}

impl uni_snd::SoundGenerator<Event> for Oscillator {
    // store the device sample rate
    fn init(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
    // this function is called when a new event is sent by the main thread
    fn handle_event(&mut self, evt: Event) {
        self.freq = evt.freq;
        self.volume = evt.volume;
    }
    // this function generates the sound, one sample at a time
    fn next_value(&mut self) -> f32 {
        self.left = !self.left;
        // stereo output. update only every two samples
        if self.left {
            // self.freq Hz sin oscillator
            self.t += self.freq / self.sample_rate;
        }
        (self.t * 3.14159 * 2.0).sin() * self.volume
    }
}

fn main() {
    let app = uni_app::App::new(uni_app::AppConfig::new("sound oscillator", (800, 600)));
    // initialize the sound driver, provide it with our generator
    let mut snd = uni_snd::SoundDriver::new(Box::new(Oscillator {
        sample_rate: 0.0,
        t: 0.0,
        left: false,
        freq: 440.0,
        volume: 1.0,
    }));
    let mut started = false;

    app.run(move |app: &mut uni_app::App| {
        for evt in app.events.borrow_mut().iter() {
            match evt {
                // use the mouse to change the sound frequency (left/right) and volume (up/down)
                AppEvent::MousePos((x, y)) => {
                    if started {
                        let freq = 220.0 + 4400.0 * (*x as f32 / 800.0).clamp(0.0, 1.0);
                        let volume = (1.0 - *y as f32 / 600.0).clamp(0.0, 1.0);
                        snd.send_event(Event { freq, volume });
                    }
                }
                AppEvent::MouseDown(_) => {
                    // start the sound generating thread (in native mode)
                    // on web, this works only on a user event (here a mouse click)
                    if !started {
                        snd.start();
                        started = true;
                    }
                }
                _ => (),
            }
        }
    });
}
