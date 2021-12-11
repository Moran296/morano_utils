pub mod callback_answer;
pub mod inline_keyboard;
pub mod menu_handler;
pub mod multi_sender;
pub mod regular_message;
pub mod sender;
pub mod stream_updates;
pub mod traits;

pub use callback_answer::*;
pub use inline_keyboard::*;
pub use menu_handler::*;
pub use multi_sender::*;
pub use regular_message::*;
pub use sender::*;
pub use stream_updates::*;
pub use traits::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
