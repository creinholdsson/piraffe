use fb_generated::size_prefixed_root_as_proto;
use byteorder::ByteOrder;

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

    builder.finish_size_prefixed(conn_buf, None);

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
    builder.finish_size_prefixed(welcome_buf, None);
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

    let publish = fb_generated::Publish::create(builder, &publish_message);

    let proto_args = fb_generated::ProtoArgs {
        timestamp: 0,
        message_type: fb_generated::All::Publish,
        message: Some(publish.as_union_value())
    };

    let publish_buf = fb_generated::Proto::create(builder, &proto_args);
    builder.finish_size_prefixed(publish_buf, None);

    dest.extend_from_slice(builder.finished_data());
}

pub fn make_subscribe(builder: &mut FlatBufferBuilder, dest: &mut Vec<u8>, topic: &str) {
    dest.clear();
    builder.reset();

    let topic = builder.create_string(topic);

    let subscribe_message = fb_generated::SubscribeArgs {
        topic: Some(topic),
    };

    let subscribe = fb_generated::Subscribe::create(builder, &subscribe_message);

    let proto_args = fb_generated::ProtoArgs {
        timestamp: 0,
        message_type: fb_generated::All::Subscribe,
        message: Some(subscribe.as_union_value())
    };
    let publish_buf = fb_generated::Proto::create(builder, &proto_args);
    builder.finish_size_prefixed(publish_buf, None);

    dest.extend_from_slice(builder.finished_data());
}

pub fn get_proto_message<'a>(buf: &'a [u8]) -> Result<(usize, fb_generated::Proto<'a>),()> {
    if buf.len() == 0 {
        return Err(())
    }
    let size = byteorder::LittleEndian::read_u32(buf);
    println!("size: {} buflen {}", size, buf.len());
    if size > buf.len() as u32 || size == 0 {
        return Err(())
    }
    
    match size_prefixed_root_as_proto(buf) {
        Ok(buf) => Ok((size as usize + 4, buf)),
        Err(_) => Err(())
    }
}