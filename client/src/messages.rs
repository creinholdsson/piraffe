

pub(crate) enum Message {
    Publish(PublishMessage),
    Subscribe(SubscribeMessage),
}

pub(crate) struct PublishMessage {
    pub(crate) subject: String,
    pub(crate) data: Vec<u8>,
}

pub(crate) struct SubscribeMessage {
    pub(crate) subject: String,
    pub(crate) sender: crossbeam::channel::Sender<DataMessage>,
}

pub struct DataMessage {
    pub subject: String,
    pub data: Vec<u8>
}

impl PublishMessage {
    pub fn new(subject: &str, data: &[u8]) -> Self {
        let mut v: Vec<u8> = vec![0; data.len()];
        println!("v len {} src len: {}", v.len(), data.len());
        v.copy_from_slice(data);

        PublishMessage {
            subject: subject.to_string(),
            data: v
        }
    }
}

impl SubscribeMessage {
    pub fn new(subject: &str, sender: crossbeam::channel::Sender<DataMessage>) -> Self {
        SubscribeMessage { subject: subject.to_string(), sender: sender }
    }
}



