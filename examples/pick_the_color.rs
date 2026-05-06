
use iced::{Element, Size, Theme};
use iced_color_picker::color_picker::{Position, ColorPicker};
use iced_color_picker::state::{ColorPickerEvent, ColorPickerState, ContentMsg};
use iced_color_picker::helpers::btn_style;
use iced::widget::{button, center, container};

pub fn main() -> iced::Result {
    iced::application(
        App::default,
        App::update,
        App::view)
        .theme(Theme::TokyoNight)
        .centered()
        .window_size(Size::new(600.0, 600.0))
        .run()
}

struct App {
    cp: ColorPickerState,
    opened: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            cp: ColorPickerState::new(70, 30, 200),
            opened: false,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Noop,
    SetOpened(bool),
    ColorPicker(ContentMsg),
}

impl App {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::ColorPicker(msg) => {
                match self.cp.update(msg) {
                    Some(ColorPickerEvent::Submitted(color)) => {
                        self.opened = false;
                        println!("Submitted {:?}", color);
                    }
                    Some(ColorPickerEvent::Cancelled) => {
                        self.opened = false;
                        println!("Cancelled");
                    }
                    Some(ColorPickerEvent::Copy(text)) => {
                        return iced::clipboard::write(text).discard();
                    }
                    None => {}
                }
            }
            Message::SetOpened(opened) => {
                self.opened = opened;
            }
            Message::Noop => {}
        }
        iced::Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let content = self.cp.view(Message::ColorPicker);

        let btn = button("Color Picker")
            .on_press(Message::Noop)
            .style(|theme, status| btn_style(theme, status));

        let cp = ColorPicker::new(
            btn,
            content,
            self.cp.current_color(),
            Position::Center,
        )
        .opened(self.opened)
        .on_open(Message::SetOpened)
        .gap(10)
        .style(container::rounded_box);

        center(cp).into()
    }
}
