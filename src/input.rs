use std::io;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::error;

pub fn receiver(sender: UnboundedSender<String>) {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() && sender.send(input).is_err() {
            error!("Couldn't report input to main thread!");
        }
    }
}

pub async fn read_line(receiver: &mut UnboundedReceiver<String>) -> Option<String> {
    Some(receiver.recv().await?.trim().to_string())
}
