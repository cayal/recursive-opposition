use nannou::prelude::*;

struct Model {
    _window: window::Id,
    split: usize,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    return Model { _window, split: 1 };
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    if _model.split == 1 { _model.split = _model.split + 1 }
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    let draw = _app.draw();
    draw.background().color(GREY);
    let win = _app.window_rect();
    if _model.split == 1 {
        let rs = win.subdivisions();
        draw.rect().xy(rs[0].xy()).wh(rs[0].wh()).color(BLACK);
        draw.rect().xy(rs[2].xy()).wh(rs[2].wh()).color(BLACK);
        draw.rect().xy(rs[1].xy()).wh(rs[1].wh()).color(WHITE);
        draw.rect().xy(rs[3].xy()).wh(rs[3].wh()).color(WHITE);
    }
    if _model.split == 2 {
        let rs = win.subdivisions();
        for r in rs.iter() {
            let r2s = r.subdivisions();
            draw.rect().xy(r2s[0].xy()).wh(r2s[0].wh()).color(BLACK);
            draw.rect().xy(r2s[2].xy()).wh(r2s[2].wh()).color(WHITE);
            draw.rect().xy(r2s[1].xy()).wh(r2s[1].wh()).color(BLACK);
            draw.rect().xy(r2s[3].xy()).wh(r2s[3].wh()).color(WHITE);
        }
    }
    draw.to_frame(_app, &_frame).unwrap();
}
