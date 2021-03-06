// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum SubscribeOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Subscribe<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Subscribe<'a> {
  type Inner = Subscribe<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> Subscribe<'a> {
  pub const VT_TOPIC: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Subscribe { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args SubscribeArgs<'args>
  ) -> flatbuffers::WIPOffset<Subscribe<'bldr>> {
    let mut builder = SubscribeBuilder::new(_fbb);
    if let Some(x) = args.topic { builder.add_topic(x); }
    builder.finish()
  }


  #[inline]
  pub fn topic(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Subscribe::VT_TOPIC, None)
  }
}

impl flatbuffers::Verifiable for Subscribe<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("topic", Self::VT_TOPIC, false)?
     .finish();
    Ok(())
  }
}
pub struct SubscribeArgs<'a> {
    pub topic: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for SubscribeArgs<'a> {
  #[inline]
  fn default() -> Self {
    SubscribeArgs {
      topic: None,
    }
  }
}
pub struct SubscribeBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> SubscribeBuilder<'a, 'b> {
  #[inline]
  pub fn add_topic(&mut self, topic: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Subscribe::VT_TOPIC, topic);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> SubscribeBuilder<'a, 'b> {
    let start = _fbb.start_table();
    SubscribeBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Subscribe<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Subscribe<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Subscribe");
      ds.field("topic", &self.topic());
      ds.finish()
  }
}
