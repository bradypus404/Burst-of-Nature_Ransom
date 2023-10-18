extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, Entry, Label, Window, WindowType};
use gtk::Inhibit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GTK 초기화
    gtk::init();

    // 창 생성
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Input Window");
    window.set_default_size(400, 400);

    // 입력 상자 생성
    let entry = Entry::new();
    entry.set_text("Type here");

    // 버튼 생성
    let button = Button::with_label("Submit");

    // 레이블 생성
    let label = Label::new(None);
    label.set_text("");

    let entry_clone = entry.clone();
    let label_clone = label.clone();

    // 버튼 클릭 이벤트 핸들러
    button.connect_clicked(move |_| {
        let input_text = entry_clone.get_text().to_string();
        label_clone.set_text(&format!("You entered: {}", input_text));
    });

    // 창 종료 이벤트 핸들러
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });

    // 레이아웃 구성
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.pack_start(&entry, false, false, 0);
    vbox.pack_start(&button, false, false, 0);
    vbox.pack_start(&label, false, false, 0);

    // 위젯들을 창에 추가
    window.add(&vbox);

    // 모든 위젯을 표시
    window.show_all();

    // 메인 루프 시작
    gtk::main();

    Ok(())
}
