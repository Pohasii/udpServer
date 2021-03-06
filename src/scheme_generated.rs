// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(unused_imports, dead_code)]
pub mod messages {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

pub enum MessOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Mess<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Mess<'a> {
    type Inner = Mess<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Mess<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Mess {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args MessArgs<'args>) -> flatbuffers::WIPOffset<Mess<'bldr>> {
      let mut builder = MessBuilder::new(_fbb);
      builder.add_id(args.id);
      if let Some(x) = args.arr { builder.add_arr(x); }
      if let Some(x) = args.text { builder.add_text(x); }
      if let Some(x) = args.name { builder.add_name(x); }
      builder.finish()
    }

    pub const VT_ID: flatbuffers::VOffsetT = 4;
    pub const VT_NAME: flatbuffers::VOffsetT = 6;
    pub const VT_TEXT: flatbuffers::VOffsetT = 8;
    pub const VT_ARR: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn id(&self) -> u64 {
    self._tab.get::<u64>(Mess::VT_ID, Some(0)).unwrap()
  }
  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Mess::VT_NAME, None)
  }
  #[inline]
  pub fn text(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Mess::VT_TEXT, None)
  }
  #[inline]
  pub fn arr(&self) -> Option<&'a [u8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Mess::VT_ARR, None).map(|v| v.safe_slice())
  }
}

pub struct MessArgs<'a> {
    pub id: u64,
    pub name: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub text: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub arr: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a ,  u8>>>,
}
impl<'a> Default for MessArgs<'a> {
    #[inline]
    fn default() -> Self {
        MessArgs {
            id: 0,
            name: None,
            text: None,
            arr: None,
        }
    }
}
pub struct MessBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MessBuilder<'a, 'b> {
  #[inline]
  pub fn add_id(&mut self, id: u64) {
    self.fbb_.push_slot::<u64>(Mess::VT_ID, id, 0);
  }
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Mess::VT_NAME, name);
  }
  #[inline]
  pub fn add_text(&mut self, text: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Mess::VT_TEXT, text);
  }
  #[inline]
  pub fn add_arr(&mut self, arr: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Mess::VT_ARR, arr);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MessBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MessBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Mess<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

#[inline]
pub fn get_root_as_mess<'a>(buf: &'a [u8]) -> Mess<'a> {
  flatbuffers::get_root::<Mess<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_mess<'a>(buf: &'a [u8]) -> Mess<'a> {
  flatbuffers::get_size_prefixed_root::<Mess<'a>>(buf)
}

#[inline]
pub fn finish_mess_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Mess<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_mess_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Mess<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod messages

