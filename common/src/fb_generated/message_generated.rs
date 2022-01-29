// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum MessageOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Message<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Message<'a> {
  type Inner = Message<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> Message<'a> {
  pub const VT_TOPIC: flatbuffers::VOffsetT = 4;
  pub const VT_REPLY_REQUESTED: flatbuffers::VOffsetT = 6;
  pub const VT_REPLY_TOPIC: flatbuffers::VOffsetT = 8;
  pub const VT_DATA: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Message { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args MessageArgs<'args>
  ) -> flatbuffers::WIPOffset<Message<'bldr>> {
    let mut builder = MessageBuilder::new(_fbb);
    if let Some(x) = args.data { builder.add_data(x); }
    if let Some(x) = args.reply_topic { builder.add_reply_topic(x); }
    if let Some(x) = args.topic { builder.add_topic(x); }
    builder.add_reply_requested(args.reply_requested);
    builder.finish()
  }


  #[inline]
  pub fn topic(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Message::VT_TOPIC, None)
  }
  #[inline]
  pub fn reply_requested(&self) -> bool {
    self._tab.get::<bool>(Message::VT_REPLY_REQUESTED, Some(false)).unwrap()
  }
  #[inline]
  pub fn reply_topic(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Message::VT_REPLY_TOPIC, None)
  }
  #[inline]
  pub fn data(&self) -> Option<&'a [u8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Message::VT_DATA, None).map(|v| v.safe_slice())
  }
}

impl flatbuffers::Verifiable for Message<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("topic", Self::VT_TOPIC, false)?
     .visit_field::<bool>("reply_requested", Self::VT_REPLY_REQUESTED, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("reply_topic", Self::VT_REPLY_TOPIC, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>("data", Self::VT_DATA, false)?
     .finish();
    Ok(())
  }
}
pub struct MessageArgs<'a> {
    pub topic: Option<flatbuffers::WIPOffset<&'a str>>,
    pub reply_requested: bool,
    pub reply_topic: Option<flatbuffers::WIPOffset<&'a str>>,
    pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
}
impl<'a> Default for MessageArgs<'a> {
  #[inline]
  fn default() -> Self {
    MessageArgs {
      topic: None,
      reply_requested: false,
      reply_topic: None,
      data: None,
    }
  }
}
pub struct MessageBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MessageBuilder<'a, 'b> {
  #[inline]
  pub fn add_topic(&mut self, topic: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_TOPIC, topic);
  }
  #[inline]
  pub fn add_reply_requested(&mut self, reply_requested: bool) {
    self.fbb_.push_slot::<bool>(Message::VT_REPLY_REQUESTED, reply_requested, false);
  }
  #[inline]
  pub fn add_reply_topic(&mut self, reply_topic: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_REPLY_TOPIC, reply_topic);
  }
  #[inline]
  pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_DATA, data);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MessageBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MessageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Message<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Message<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Message");
      ds.field("topic", &self.topic());
      ds.field("reply_requested", &self.reply_requested());
      ds.field("reply_topic", &self.reply_topic());
      ds.field("data", &self.data());
      ds.finish()
  }
}
