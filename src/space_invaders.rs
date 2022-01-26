use std::{
    fs::File,
    io::Read,
    thread,
    time::{Duration, Instant},
};

use sdl2::{event::Event, pixels::PixelFormatEnum, keyboard::Keycode, mixer::{self, InitFlag}};

use crate::{cpu::CPU, machine::Machine};

pub struct SpaceInvaders {
    cpu: CPU,
}

impl SpaceInvaders {
    pub fn new() -> Self {
        let mut cpu = CPU::default();
        let mut file = File::open("./roms/space-invaders/invaders").unwrap();
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer).unwrap();
        cpu.load_rom(&buffer, 0);

        Self { cpu }
    }

    pub fn frame_buffer(&self) -> &[u8] {
        &self.cpu.memory().slice(0x2400..0x4000)
    }

    pub fn emulate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let width = 224;
        let height = 256;
        let cycle_per_frame = 2_000_000 / 60;
        let mut machine = SpaceInvadersMachine::default();
        let sdl_context = sdl2::init()?;
        let _sdl_audio = sdl_context.audio()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Space Invaders", width, height)
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas().build()?;
        let texture_creator = canvas.texture_creator();
        let mut texture =
            texture_creator.create_texture_streaming(PixelFormatEnum::RGB332, width, height)?;
        let mut event_pump = sdl_context.event_pump()?;

        self.cpu.show_debug_log = false;

        'running: loop {
            let start = Instant::now();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => match keycode {
                        Keycode::A => machine.key_down(Keys::LeftP1),
                        Keycode::D => machine.key_down(Keys::Right1),
                        Keycode::W => machine.key_down(Keys::Shoot1),
                        Keycode::Num1 => machine.key_down(Keys::StartP1),
                        Keycode::J => machine.key_down(Keys::LeftP2),
                        Keycode::L => machine.key_down(Keys::Right2),
                        Keycode::I => machine.key_down(Keys::Shoot2),
                        Keycode::Num2 => machine.key_down(Keys::StartP2),
                        Keycode::C => machine.key_down(Keys::Coin),
                        _ => (),
                    },
                    Event::KeyUp {
                        keycode: Some(keycode),
                        ..
                    } => match keycode {
                        Keycode::A => machine.key_up(Keys::LeftP1),
                        Keycode::D => machine.key_up(Keys::Right1),
                        Keycode::W => machine.key_up(Keys::Shoot1),
                        Keycode::Num1 => machine.key_up(Keys::StartP1),
                        Keycode::J => machine.key_up(Keys::LeftP2),
                        Keycode::L => machine.key_up(Keys::Right2),
                        Keycode::I => machine.key_up(Keys::Shoot2),
                        Keycode::Num2 => machine.key_up(Keys::StartP2),
                        Keycode::C => machine.key_up(Keys::Coin),
                        _ => (),
                    },
                    _ => {}
                }
            }

            for i in 1..3 {
                let mut cycles_remaining = cycle_per_frame / 2;
                while cycles_remaining > 0 {
                    cycles_remaining -= self.cpu.emulate(&mut machine) as i32;
                }

                self.cpu.interrupt(i);
            }

            let mut i = 0;
            let frame_buffer = self.frame_buffer();
            texture.with_lock(None, |buffer, _pitch| {
                for x in 0..width {
                    for y in (0..height).step_by(8) {
                        let mut byte = frame_buffer[i];
                        i += 1;
                        for bit in 0..8 {
                            let index = ((255 - y - bit) * 224 + x) as usize;
                            if byte & 1 > 0 {
                                buffer[index] = 28;
                            } else {
                                buffer[index] = 0;
                            }
                            byte >>= 1;
                        }
                    }
                }
            })?;

            canvas.copy(&texture, None, None)?;
            canvas.present();

            let elapsed = start.elapsed();
            if elapsed <= Duration::from_secs_f64(1f64 / 60f64) {
                thread::sleep(Duration::from_secs_f64(1f64 / 60f64) - elapsed);
            }
        }

        Ok(())
    }
}

pub enum Keys {
    Coin = 0,
    StartP1 = 2,
    StartP2 = 1,
    LeftP1 = 5,
    LeftP2 = 8 + 5,
    Right1 = 6,
    Right2 = 8 + 6,
    Shoot1 = 4,
    Shoot2 = 8 + 4,
}

pub struct SpaceInvadersMachine {
    shift0: u8,
    shift1: u8,
    shift_offset: u8,
    port: u16,
    audio: SpaceInvadersAudio
}

impl Default for SpaceInvadersMachine {
    fn default() -> Self {
        Self {
            shift0: 0,
            shift1: 0,
            shift_offset: 0,
            port: 0,
            audio: SpaceInvadersAudio::new(),
        }
    }
}

impl SpaceInvadersMachine {
    pub fn key_down(&mut self, key: Keys) {
        self.port = self.port | (1 << key as usize)
    }

    pub fn key_up(&mut self, key: Keys) {
        self.port = self.port & !(1 << key as usize)
    }
}

impl Machine for SpaceInvadersMachine {
    fn input(&self, port: u8) -> u8 {
        match port {
            1 => (self.port & 0xff) as u8,
            2 => (self.port >> 8) as u8,
            3 => {
                let value = ((self.shift1 as u16) << 8) | self.shift0 as u16;
                (value >> (8 - self.shift_offset)) as u8
            }
            _ => unimplemented!(),
        }
    }

    fn output(&mut self, port: u8, value: u8) {
        match port {
            2 => self.shift_offset = value & 0x7,
            3 => {
                if value & 0x01 == 0x01 {
                    self.audio.play(Audios::UfoHighPitch, 1);
                }
                else {
                    self.audio.pause(Audios::UfoHighPitch);
                }

                if value & 0x02 == 0x02 {
                    self.audio.play(Audios::Shoot, 0);
                }
                else {
                    self.audio.pause(Audios::Shoot)
                }

                if value & 0x04 == 0x04 {
                    self.audio.play(Audios::Explosion, 0);
                }
                else {
                    self.audio.pause(Audios::Explosion);
                }

                if value & 0x08 == 0x08 {
                    self.audio.play(Audios::InvaderKilled, 0);
                }
                else {
                    self.audio.pause(Audios::InvaderKilled);
                }
            },
            4 => {
                self.shift0 = self.shift1;
                self.shift1 = value;
            }
            5 => {
                if value & 0x01 == 0x01 {
                    self.audio.play(Audios::FastInvader1, 0);
                }
                else {
                    self.audio.pause(Audios::FastInvader1);
                }

                if value & 0x02 == 0x02 {
                    self.audio.play(Audios::FastInvader2, 0);
                }
                else {
                    self.audio.pause(Audios::FastInvader2);
                }

                if value & 0x04 == 0x04 {
                    self.audio.play(Audios::FastInvader3, 0);
                }
                else {
                    self.audio.pause(Audios::FastInvader3);
                }

                if value & 0x08 == 0x08 {
                    self.audio.play(Audios::FastInvader4, 0);
                }
                else {
                    self.audio.pause(Audios::FastInvader4);
                }
            },
            6 => (),
            _ => unimplemented!(),
        }
    }
}

pub enum Audios {
    UfoHighPitch = 0,
    FastInvader1,
    FastInvader2,
    FastInvader3,
    FastInvader4,
    InvaderKilled,
    Explosion,
    Shoot,
}

struct SpaceInvadersAudio {
    ufo_high_pitch: mixer::Chunk,
    fast_invader1: mixer::Chunk,
    fast_invader2: mixer::Chunk,
    fast_invader3: mixer::Chunk,
    fast_invader4: mixer::Chunk,
    invader_killed: mixer::Chunk,
    explosion: mixer::Chunk,
    shoot: mixer::Chunk,
}

impl SpaceInvadersAudio {
    pub fn new() -> Self {
        mixer::open_audio(44_100, sdl2::mixer::AUDIO_S8, 1, 256).unwrap();
        mixer::init(InitFlag::MID).unwrap();
        mixer::allocate_channels(8);
        mixer::Channel::all().set_volume(mixer::MAX_VOLUME / 2);

        let ufo_high_pitch = mixer::Chunk::from_file("./sounds/ufo_highpitch.wav").unwrap();
        let fast_invader1 = mixer::Chunk::from_file("./sounds/fastinvader1.wav").unwrap();
        let fast_invader2 = mixer::Chunk::from_file("./sounds/fastinvader2.wav").unwrap();
        let fast_invader3 = mixer::Chunk::from_file("./sounds/fastinvader3.wav").unwrap();
        let fast_invader4 = mixer::Chunk::from_file("./sounds/fastinvader4.wav").unwrap();
        let invader_killed = mixer::Chunk::from_file("./sounds/invaderkilled.wav").unwrap();
        let explosion = mixer::Chunk::from_file("./sounds/explosion.wav").unwrap();
        let shoot = mixer::Chunk::from_file("./sounds/shoot.wav").unwrap();

        Self {
            ufo_high_pitch,
            fast_invader1,
            fast_invader2,
            fast_invader3,
            fast_invader4,
            invader_killed,
            explosion,
            shoot,
        }
    }

    #[allow(dead_code)]
    pub fn set_volume(&self, volume: i32) {
        mixer::Channel::all().set_volume(volume);
    }

    pub fn play(&self, audio: Audios, loops: i32) {
        let chunk = self.audio_chunk(&audio);
        mixer::Channel(audio as i32).play(chunk, loops).unwrap();
    }

    pub fn pause(&self, audio: Audios) {
        mixer::Channel(audio as i32).pause();
    }

    fn audio_chunk(&self, audio: &Audios) -> &mixer::Chunk {
        match audio {
            Audios::UfoHighPitch => &self.ufo_high_pitch,
            Audios::FastInvader1 => &self.fast_invader1,
            Audios::FastInvader2 => &self.fast_invader2,
            Audios::FastInvader3 => &self.fast_invader3,
            Audios::FastInvader4 => &self.fast_invader4,
            Audios::InvaderKilled => &self.invader_killed,
            Audios::Explosion => &self.explosion,
            Audios::Shoot => &self.shoot,
        }
    }
}