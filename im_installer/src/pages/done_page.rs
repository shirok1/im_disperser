use vizia::prelude::*;

#[derive(Lens)]
pub(crate) struct DonePage {}

impl DonePage {
    pub fn new(cx: &mut Context) -> Handle<'_, Self> {
        DonePage {}.build(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "安装完成").class("p");
            })
            .class("opt-panel");
        })
    }
}

impl View for DonePage {
    fn element(&self) -> Option<&'static str> {
        Some("select-format-page")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {}
}
