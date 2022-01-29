// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum WelcomeOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Welcome<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Welcome<'a> {
  type Inner = Welcome<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> Welcome<'a> {
  pub const VT_SERVER_NAME: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Welcome { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args WelcomeArgs<'args>
  ) -> flatbuffers::WIPOffset<Welcome<'bldr>> {
    let mut builder = WelcomeBuilder::new(_fbb);
    if let Some(x) = args.server_name { builder.add_server_name(x); }
    builder.finish()
  }


  #[inline]
  pub fn server_name(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Welcome::VT_SERVER_NAME, None)
  }
}

impl flatbuffers::Verifiable for Welcome<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("server_name", Self::VT_SERVER_NAME, false)?
     .finish();
    Ok(())
  }
}
pub struct WelcomeArgs<'a> {
    pub server_name: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for WelcomeArgs<'a> {
  #[inline]
  fn default() -> Self {
    WelcomeArgs {
      server_name: None,
    }
  }
}
pub struct WelcomeBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> WelcomeBuilder<'a, 'b> {
  #[inline]
  pub fn add_server_name(&mut self, server_name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Welcome::VT_SERVER_NAME, server_name);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> WelcomeBuilder<'a, 'b> {
    let start = _fbb.start_table();
    WelcomeBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Welcome<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Welcome<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Welcome");
      ds.field("server_name", &self.server_name());
      ds.finish()
  }
}
