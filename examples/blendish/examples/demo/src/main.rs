#![feature(globs)]

extern crate graphics;
extern crate piston;
// extern crate sdl2_game_window;
extern crate glfw_game_window;

extern crate nanovg;
extern crate blendish;

// use Window = sdl2_game_window::GameWindowSDL2;
use Window = glfw_game_window::GameWindowGLFW;
use piston::{
    Game,
    GameIteratorSettings,
    GameWindowSettings,
    UpdateArgs,
    RenderArgs,
    KeyPressArgs,
    KeyReleaseArgs,
    MousePressArgs,
    MouseReleaseArgs,
    MouseMoveArgs,
    MouseRelativeMoveArgs,
    MouseScrollArgs,
};


use blendish::*;
use blendish::WidgetState;
//use blendish::theme::Theme;



pub struct App<'a> {
    _blowup: bool,
    _screenshot: bool,
    _premult: bool,

    theme: Theme<'a>
}

impl<'a> App<'a> {
    /// Creates a new application.
    pub fn new() -> App<'a> {
        App {
            _blowup: false,
            _screenshot: false,
            _premult: false,

            theme: Theme::new(nanovg::Ctx::create_gL3(nanovg::ANTIALIAS | nanovg::STENCIL_STROKES))
        }
    }
    fn nvg(&mut self) -> &mut nanovg::Ctx { self.theme.nvg() }
}



impl<'a> Game for App<'a> {
    /// Perform tasks for loading before showing anything.
    fn load(&mut self) {
        //self.theme = Some(Theme::new(&self.nvg));
    }

    fn update(&mut self, _args: &UpdateArgs) {}
    fn render(&mut self, _args: &RenderArgs) {
        self.theme.draw_label(100.0,100.0, 200.0,40.0, -1, "Hello World!");
    }

    fn key_press(&mut self,  _args: &KeyPressArgs) {}
    fn key_release(&mut self, _args: &KeyReleaseArgs) {}

    fn mouse_press(&mut self, _args: &MousePressArgs) {}
    fn mouse_release(&mut self, _args: &MouseReleaseArgs) {}
    fn mouse_move(&mut self, _args: &MouseMoveArgs) {}
    /// Moved mouse relative, not bounded by cursor.
    fn mouse_relative_move(&mut self, _args: &MouseRelativeMoveArgs) {}
    fn mouse_scroll(&mut self, _args: &MouseScrollArgs) {}
}

fn main() {
    let mut window = Window::new(
        GameWindowSettings {
            title: "NanoMorph Demo".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
        }
    );

    let mut app = App::new();
    let game_iter_settings = GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    app.run(&mut window, &game_iter_settings);
}





