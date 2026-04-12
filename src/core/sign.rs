use crate::Message;
use iced::Element;
use iced::widget::text;

pub fn sign_widget<'a>() -> Element<'a, Message> {
    iced::widget::column![text("Multitul"), text("by @rubi_ck @efcolipt")].into()
}
