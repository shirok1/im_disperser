use vizia::prelude::*;

#[derive(Lens)]
pub(crate) struct InstallingPage {
    subtitle: Localized,
    show_install_btn: bool,
}

pub(crate) enum InstallingPageEvent {
    Install,
}

impl InstallingPage {
    pub fn new(cx: &mut Context) -> Handle<'_, Self> {
        InstallingPage {
            subtitle: Localized::new("confirm-install"),
            show_install_btn: true,
        }
        .build(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, Localized::new("install")).class("p");
                Label::new(cx, Self::subtitle).class("p-xs");

                Button::new(cx, |cx| Label::new(cx, Localized::new("install")))
                    .on_press(|cx| {
                        cx.emit(InstallingPageEvent::Install);
                    })
                    .visibility(Self::show_install_btn);
            })
            .class("opt-panel");

            cx.emit(InstallingPageEvent::Install);
        })
    }
}

impl View for InstallingPage {
    fn element(&self) -> Option<&'static str> {
        Some("select-format-page")
    }

    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _meta| match app_event {
            InstallingPageEvent::Install => {
                self.show_install_btn = false;
                self.subtitle = Localized::new("installing");
            }
        });
    }
}
