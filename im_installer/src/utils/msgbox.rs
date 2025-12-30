#[macro_export]
macro_rules! err_msgbox {
    // if no title
    ($msg:expr) => {{
        rfd::MessageDialog::new()
            .set_title("Error")
            .set_description($msg)
            .show();
        return;
    }};

    ($msg:expr, $title:expr) => {{
        rfd::MessageDialog::new()
            .set_title($title)
            .set_description($msg)
            .show();
        return;
    }};
}

#[macro_export]
macro_rules! some_or_msgbox {
    ($opt:expr, $msg:expr) => {{
        match $opt {
            Some(v) => v,
            None => {
                rfd::MessageDialog::new()
                    .set_title("Error")
                    .set_description($msg)
                    .show();
                return;
            }
        }
    }};

    ($opt:expr) => {{
        match $opt {
            Some(v) => v,
            None => {
                rfd::MessageDialog::new()
                    .set_title("Error")
                    .set_description("here is an error but idk what happened")
                    .show();
                return;
            }
        }
    }};
}

#[macro_export]
macro_rules! ok_or_msgbox {
    ($opt:expr, $msg:expr) => {{
        match $opt {
            Ok(v) => v,
            Err(e) => {
                rfd::MessageDialog::new()
                    .set_title("Error")
                    .set_description(format!("{}\n{}", $msg, e))
                    .show();
                return;
            }
        }
    }};

    ($opt:expr) => {{
        match $opt {
            Ok(v) => v,
            Err(e) => {
                rfd::MessageDialog::new()
                    .set_title("Error")
                    .set_description(format!("here is an error but idk what happened\n{}", e))
                    .show();
                return;
            }
        }
    }};
}
