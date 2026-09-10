use arboard::Clipboard;

pub(super) fn read() -> String {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.get_text().unwrap()
}

pub(super) fn write(text: &String) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(text).unwrap();
}
