# Intel 8080 Space Invaders Emulator with Rust
A project for me to learn emulator development and rust. This emulator uses SDL2 for rendering.

## Build
You need to place the space invaders rom file and the audio files to the corresponding folder.  
This emulator uses SDL2 for rendering, you need to setup the SDL2 and SDL2-mixer development libraries first.  
You can follow the guide in [Rust-SDL2](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries) repo.

### Rom
The rom file should concatednate into a single file, name it `invaders` and place it to `roms/` folder.  
On Unix, you can concatednate those 4 parts of rom files using the following command: 
```
cat invaders.h > invaders
cat invaders.g >> invaders
cat invaders.f >> invaders
cat invaders.e >> invaders
```

### Sound
You need to place the following audio files into `sounds` folder.
 - explosion.wav
 - fastinvader1.wav
 - fastinvader2.wav
 - fastinvader3.wav
 - fastinvader4.wav
 - invaderKilled.wav
 - shoot.wav
 - ufo_highpitch.wav

### Run
```
cargo run --release
```

## Game Control
### Player 
|                | Player1 | Player2 |
| :------------- | :-----: | :-----: |
| **Move Left**  | A       | J       |
| **Move Right** | D       | L       |
| **Shoot**      | W       | I       |
| **Start**      | 1       | 2       |

### Other 
| Key | Action |
| --- | :----: |
|  C  | Coin   |

## Remarks
The opcodes which are not used by the space invaders may contains bugs because I have not test it. For example `DAA` and `Auxiliary Carry Flag`.  
Maybe I will test it with `cpudiag` someday...

## Useful Resources
There are some resources I used when I developing this project.  
- [Emulator 101](http://www.emulator101.com/welcome.html)  
- [Computer Archeology](http://computerarcheology.com/Arcade/SpaceInvaders/)  
- [Javascript 8080 emulator](https://bluishcoder.co.nz/js8080/)
- [Intel 8080 instruction set](https://pastraiser.com/cpu/i8080/i8080_opcodes.html)
- [8080 instruction encoding](http://dunfield.classiccmp.org/r/8080.txt)