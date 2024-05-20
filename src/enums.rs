use tray_icon::menu::MenuId;

#[derive(Debug, Clone, PartialEq)]
pub enum TrayMessage {
    ShowHide,
    // About,
    Quit,
}

impl TrayMessage {
    pub fn to_menu_id(self) -> MenuId {
        match self {
            TrayMessage::ShowHide => MenuId::new("show"),
            // TrayMessage::About => MenuId::new("about"),
            TrayMessage::Quit => MenuId::new("quit"),
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            TrayMessage::ShowHide => "Show/Hide",
            // TrayMessage::About => "About",
            TrayMessage::Quit => "Quit",
        }
    }

    pub fn from_string(s: String) -> Self {
        let s = s.as_str();
        match s {
            "show" => TrayMessage::ShowHide,
            // "about" => TrayMessage::About,
            "quit" => TrayMessage::Quit,
            _ => panic!("Invalid TrayMessage value"),
        }
    }
}
