
pub mod fb_generated;
use flatbuffers::*;

pub struct ConnectionMessage {
    pub name: String
}

pub struct PublishMessage {
    topic: String,
    data: Vec<u8>,
}

pub struct RequestMessage {
    topic: String,
    reply_subject: String,
    data: Vec<u8>
}


pub struct MessageFromCoordinator {
    pub topic: Option<String>,
    pub message_data: Vec<u8>
}

pub enum MessageToCoordinator {
    Connect(ConnectionMessage),
    Publish(PublishMessage),
    Request(RequestMessage),
}

impl MessageFromCoordinator {
    pub fn new(topic: Option<String>, message_data: Vec<u8>) -> Self {
        MessageFromCoordinator { topic, message_data }
    }
}


pub fn make_connect(builder: &mut FlatBufferBuilder, dest: &mut Vec<u8>, client_name: &str) {
    dest.clear();
    builder.reset();

    let name = builder.create_string(client_name);

    let connection_message = fb_generated::ConnectionArgs {
        name: Some(name)
    };

    let connection = fb_generated::Connection::create(builder, &connection_message);

    let args = fb_generated::ProtoArgs {
        timestamp: 0,
        message_type: fb_generated::All::Connection,
        message: Some(connection.as_union_value())
    };

    let conn_buf = fb_generated::Proto::create(builder, &args);

    builder.finish(conn_buf, None);

    let extended_data = builder.finished_data();
    dest.extend_from_slice(extended_data);
}

pub fn make_welcome(builder: &mut FlatBufferBuilder, dest: &mut Vec<u8>, server_name: &str) {
    dest.clear();
    builder.reset();
    let name = builder.create_string(server_name);

    let welcome_message = fb_generated::WelcomeArgs {
        server_name: Some(name)
    };

    let welcome = fb_generated::Welcome::create(builder, &welcome_message);

    let args = fb_generated::ProtoArgs {
        timestamp: 0,
        message_type: fb_generated::All::Welcome,
        message: Some(welcome.as_union_value())
    };

    let welcome_buf = fb_generated::Proto::create(builder, &args);
    builder.finish_minimal(welcome_buf);
    dest.extend_from_slice(builder.finished_data());
}

pub fn make_publish(builder: &mut FlatBufferBuilder, dest: &mut Vec<u8>, topic: &str, data: &[u8]) {
    dest.clear();
    builder.reset();

    let topic = builder.create_string(topic);
    let data = builder.create_vector(data);

    let publish_message = fb_generated::PublishArgs {
        topic: Some(topic),
        data: Some(data)
    };

    let publish = fb_generated::Publish::create(builder, &&publish_message);

    let proto_args = fb_generated::ProtoArgs {
        timestamp: 0,
        message_type: fb_generated::All::Publish,
        message: Some(publish.as_union_value())
    };

    let publish_buf = fb_generated::Proto::create(builder, &proto_args);
    builder.finish_minimal(publish_buf);

    dest.extend_from_slice(builder.finished_data());
}