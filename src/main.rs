use space_invaders::SpaceInvaders;

mod cpu;
mod opcode;
mod register;
mod memory;
mod machine;
mod space_invaders;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut space_invaders = SpaceInvaders::new();
    space_invaders.emulate()?;

    Ok(())
}