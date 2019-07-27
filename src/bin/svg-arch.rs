
extern crate azul;
use std::time::Duration;
use azul::{
    prelude::*,
    widgets::{button::Button, svg::*},
};


const SVG: &str = include_str!("../../img/tiger.svg");

#[derive(Debug)]
struct MyAppData {
    cache: SvgCache,
    layers: Vec<(SvgLayerId, SvgStyle)>,
    frame_count: u64,
}

type CbInfo<'a, 'b> = CallbackInfo<'a, 'b, MyAppData>;

impl Layout for MyAppData {
    fn layout(&self, _info: LayoutInfo<Self>) -> Dom<MyAppData> {
        let ptr = StackCheckedPointer::new(self, self).unwrap();
      Dom::div().with_child(
        Dom::gl_texture(draw_svg, ptr).with_id("svg-container"),
      )
    }
}

fn timer_callback(state: TimerCallbackInfo<MyAppData>) -> (UpdateScreen, TerminateTimer) {
  eprintln!("cb {}", state.state.frame_count);
  (Redraw, if state.state.frame_count >= 256 {
    TerminateTimer::Terminate
  }else {
    TerminateTimer::Continue
  })
}
fn draw_svg(info: GlCallbackInfoUnchecked<MyAppData>) -> GlCallbackReturn {
  let cb = |info: GlCallbackInfo<MyAppData, MyAppData>| {
            use azul::widgets::svg::SvgLayerResource::*;

    info.state.frame_count += 1;
    let map = info.state;
    let logical_size = info.bounds.get_logical_size();
    Some(Svg::with_layers(map.layers.iter().map(|e| Reference(*e)).collect())
         .render_svg(&map.cache, &info.layout_info.window, logical_size))
  };
    unsafe {
        info.invoke_callback(cb)
    }
}


fn main() {

    let mut svg_cache = SvgCache::empty();
    let mut svg_layers = svg_cache.add_svg(&SVG).unwrap();
    let app_data = MyAppData {
        cache: svg_cache,
        layers: svg_layers,
        frame_count: 0,
    };

  let mut app = App::new(app_data, AppConfig::default()).unwrap();
    let window = app.create_window(WindowCreateOptions::default(), css::native()).unwrap();
  let timer = Timer::new(timer_callback).with_interval(Duration::from_millis(5));
  app.app_state.add_timer(TimerId::new(), timer);
    app.run(window).unwrap();
}
