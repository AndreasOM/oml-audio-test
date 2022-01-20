
use minifb::{Key, KeyRepeat, Window, WindowOptions};

use oml_audio::Audio;
use oml_audio::fileloader::FileLoaderDisk;

const WIDTH: usize = 1024;
const HEIGHT: usize = 512;

fn main() {
   let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));


    // setup audio
    let mut fileloader = FileLoaderDisk::new( "./data" );
    fileloader.enable_debug();

    let mut audio = Audio::new();
    audio.start();
    audio.load_sound_bank( &mut fileloader, "test.omsb" );

    let mut scale = 128.0;
    let mut xscale = 1.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let _timestep = audio.update();

        if window.is_key_pressed(Key::C, KeyRepeat::No) {
            // start capture
            audio.capture( 1024*10 );
            audio.play_sound( "COIN" );
        }

        if window.is_key_pressed(Key::S, KeyRepeat::No) {
            // start capture
            audio.capture( 1024 );
            audio.play_sound( "SINE" );
        }

        if window.is_key_pressed(Key::T, KeyRepeat::Yes) {
            scale += 1.0;
        }
        if window.is_key_pressed(Key::G, KeyRepeat::Yes) {
            if scale > 0.0 {
                scale -= 1.0;
            } else {
                scale = 0.0;
            }
        }
        if window.is_key_pressed(Key::F, KeyRepeat::Yes) {
            xscale += 0.01;
        }
        if window.is_key_pressed(Key::H, KeyRepeat::Yes) {
            xscale -= 0.01;
        }

        for i in buffer.iter_mut() {
            *i = 0;
        }

        let capture_buffer = audio.capture_buffer_slice();
        let l = capture_buffer.len();
        for x in 0..l {
            if x >= WIDTH {
                break;
            }
            let xs = ( x as f32 * xscale ) as usize;
            if xs >= l {
                break;
            }

            let v = capture_buffer[ xs ];
            let y = 256 + ( v * scale ) as isize;

            if y > 0 && y < HEIGHT as isize {
                let y = y as usize;
                let o = y * WIDTH + x;
                buffer[ o ] = 0xffffffff;
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }}
