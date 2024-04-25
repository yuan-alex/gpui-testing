use gpui::*;

struct Wrapper {
    text: SharedString,
}

struct Button {
    text: SharedString,
}

impl Render for Button {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().border_1().child(self.text.clone())
    }
}

impl Render for Wrapper {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .text_xl()
            .child(div())
            .child(format!("Hello, {}!", &self.text))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| Button {
                text: "World".into(),
            })
        });
    });
}
