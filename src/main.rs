use miniquad::{
    conf::Conf,
    UserData,
    EventHandler,
    graphics::Context,
};

struct Stage;

impl EventHandler for Stage {
    fn update(&mut self, context: &mut Context) { 
    }

    fn draw(&mut self, context: &mut Context) { 
        //Sets clear color, depth buffer and stencil buffer
        context.clear(Some((0.390, 0.186, 0.100, 1.0)), None, None);
    }
}

fn main() {
    //Allocate sapp_desc and fill it with conf data and UserData fomr closure.
    //Calls _sapp_run which calls native platform specific code to create window and init. (on windows it WNDCLASSW wndclassw;)
    miniquad::start(Conf::default(), |ctx| UserData::owning(Stage, ctx));
}
