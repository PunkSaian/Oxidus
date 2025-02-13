# OXIDUS
remake of [oxide](https://github.com/ooxymoron)

dev discord [discord](https://discord.gg/8EygS3t8xq)

includes a full netvar dumper and struct generater `cargo make dump-netvars`

## Running
1. add `-gl` to your startup flags

## Development
### Requirements
cargo\
cargo-make

### Running developemnt
1. cargo make start-tf2
2. cargo make inject

### Common issues
1. when running `start-tf2`

#### Issue
`AppFramework : Unable to load module engine.so!
Unable to load interface VCvarQuery001 from engine.so`

#### Fix 
install openal 
`sudo pacman -S openal`
