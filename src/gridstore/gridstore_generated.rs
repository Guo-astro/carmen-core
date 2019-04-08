// automatically generated by the FlatBuffers compiler, do not modify


#![allow(dead_code)]
#![allow(unused_imports)]
extern crate flatbuffers;

pub enum CoordOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Coord<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Coord<'a> {
    type Inner = Coord<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Coord<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Coord {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args CoordArgs<'args>) -> flatbuffers::WIPOffset<Coord<'bldr>> {
      let mut builder = CoordBuilder::new(_fbb);
      if let Some(x) = args.ids { builder.add_ids(x); }
      builder.add_coord(args.coord);
      builder.finish()
    }

    pub const VT_COORD: flatbuffers::VOffsetT = 4;
    pub const VT_IDS: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn coord(&self) -> u32 {
    self._tab.get::<u32>(Coord::VT_COORD, Some(0)).unwrap()
  }
  #[inline]
  pub fn ids(&self) -> Option<flatbuffers::Vector<'a, u32>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u32>>>(Coord::VT_IDS, None)
  }
}

pub struct CoordArgs<'a> {
    pub coord: u32,
    pub ids: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a ,  u32>>>,
}
impl<'a> Default for CoordArgs<'a> {
    #[inline]
    fn default() -> Self {
        CoordArgs {
            coord: 0,
            ids: None,
        }
    }
}
pub struct CoordBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> CoordBuilder<'a, 'b> {
  #[inline]
  pub fn add_coord(&mut self, coord: u32) {
    self.fbb_.push_slot::<u32>(Coord::VT_COORD, coord, 0);
  }
  #[inline]
  pub fn add_ids(&mut self, ids: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u32>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Coord::VT_IDS, ids);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> CoordBuilder<'a, 'b> {
    let start = _fbb.start_table();
    CoordBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Coord<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum RelevScoreOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct RelevScore<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for RelevScore<'a> {
    type Inner = RelevScore<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> RelevScore<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        RelevScore {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args RelevScoreArgs<'args>) -> flatbuffers::WIPOffset<RelevScore<'bldr>> {
      let mut builder = RelevScoreBuilder::new(_fbb);
      if let Some(x) = args.coords { builder.add_coords(x); }
      builder.add_relev_score(args.relev_score);
      builder.finish()
    }

    pub const VT_RELEV_SCORE: flatbuffers::VOffsetT = 4;
    pub const VT_COORDS: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn relev_score(&self) -> u8 {
    self._tab.get::<u8>(RelevScore::VT_RELEV_SCORE, Some(0)).unwrap()
  }
  #[inline]
  pub fn coords(&self) -> Option<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Coord<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Coord<'a>>>>>(RelevScore::VT_COORDS, None)
  }
}

pub struct RelevScoreArgs<'a> {
    pub relev_score: u8,
    pub coords: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<Coord<'a >>>>>,
}
impl<'a> Default for RelevScoreArgs<'a> {
    #[inline]
    fn default() -> Self {
        RelevScoreArgs {
            relev_score: 0,
            coords: None,
        }
    }
}
pub struct RelevScoreBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> RelevScoreBuilder<'a, 'b> {
  #[inline]
  pub fn add_relev_score(&mut self, relev_score: u8) {
    self.fbb_.push_slot::<u8>(RelevScore::VT_RELEV_SCORE, relev_score, 0);
  }
  #[inline]
  pub fn add_coords(&mut self, coords: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Coord<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(RelevScore::VT_COORDS, coords);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> RelevScoreBuilder<'a, 'b> {
    let start = _fbb.start_table();
    RelevScoreBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<RelevScore<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum PhraseRecordOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct PhraseRecord<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for PhraseRecord<'a> {
    type Inner = PhraseRecord<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> PhraseRecord<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        PhraseRecord {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args PhraseRecordArgs<'args>) -> flatbuffers::WIPOffset<PhraseRecord<'bldr>> {
      let mut builder = PhraseRecordBuilder::new(_fbb);
      if let Some(x) = args.relev_scores { builder.add_relev_scores(x); }
      builder.finish()
    }

    pub const VT_RELEV_SCORES: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn relev_scores(&self) -> Option<flatbuffers::Vector<flatbuffers::ForwardsUOffset<RelevScore<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<RelevScore<'a>>>>>(PhraseRecord::VT_RELEV_SCORES, None)
  }
}

pub struct PhraseRecordArgs<'a> {
    pub relev_scores: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<RelevScore<'a >>>>>,
}
impl<'a> Default for PhraseRecordArgs<'a> {
    #[inline]
    fn default() -> Self {
        PhraseRecordArgs {
            relev_scores: None,
        }
    }
}
pub struct PhraseRecordBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> PhraseRecordBuilder<'a, 'b> {
  #[inline]
  pub fn add_relev_scores(&mut self, relev_scores: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<RelevScore<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(PhraseRecord::VT_RELEV_SCORES, relev_scores);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> PhraseRecordBuilder<'a, 'b> {
    let start = _fbb.start_table();
    PhraseRecordBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<PhraseRecord<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

