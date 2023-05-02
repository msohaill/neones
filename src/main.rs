pub mod bus;
pub mod cpu;
pub mod ppu;
pub mod renderer;
pub mod utils;

use bus::Bus;
use ppu::PPU;
use sdl2::event::Event;
// use sdl2::EventPump;
use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
// use sdl2::render::TextureAccess;

use bus::cartridge::Cartridge;
use cpu::CPU;
use renderer::frame::Frame;

fn main() {
  let sdl_context = sdl2::init().unwrap();
  let window = sdl_context
    .video()
    .unwrap()
    .window("Tile Viewer", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
    .position_centered()
    .build()
    .unwrap();

  let mut canvas = window.into_canvas().present_vsync().build().unwrap();
  let mut event_pump = sdl_context.event_pump().unwrap();

  canvas.set_scale(3.0, 3.0).unwrap();

  let creator = canvas.texture_creator();
  let mut texture = creator
    .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
    .unwrap();

  let rom = Cartridge::new(&std::fs::read("dev/Pac_Man.nes").unwrap()).unwrap();
  let mut frame = Frame::new();

  let bus = Bus::new(rom, move |ppu: &PPU| {
    renderer::render(ppu, &mut frame);
    texture.update(None, &frame.data, 256 * 3).unwrap();
    canvas.copy(&texture, None, None).unwrap();

    canvas.present();
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => std::process::exit(0),
        _ => (),
      }
    }
  });

  let mut cpu = CPU::new(bus);
  cpu.start(|_| {});
}
