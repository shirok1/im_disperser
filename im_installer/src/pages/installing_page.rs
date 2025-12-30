use vizia::prelude::*;

use crate::{AppEvent, err_msgbox, utils::msgbox};

#[derive(Lens)]
pub(crate) struct InstallingPage {
    subtitle: String,
    show_install_btn: bool,
}

pub(crate) enum InstallingPageEvent {
    Install,
    Finish,
}

impl InstallingPage {
    pub fn new(cx: &mut Context) -> Handle<'_, Self> {
        InstallingPage {
            subtitle: "确认安装？".to_string(),
            show_install_btn: true,
        }
        .build(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "安装").class("p");
                Label::new(cx, Self::subtitle).class("p-xs");

                Button::new(cx, |cx| Label::new(cx, "安装"))
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

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, meta| match app_event {
            InstallingPageEvent::Install => {
                self.show_install_btn = false;
                self.subtitle = "正在安装...".to_string();
            }
            InstallingPageEvent::Finish => {
                self.show_install_btn = false;
                self.subtitle = "安装完成".to_string();
            }
        });
    }
}
