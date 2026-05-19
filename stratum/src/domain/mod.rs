use lru::LruCache;
use std::rc::Rc;

pub mod actor;
pub mod commit;
pub mod mfile;

pub(crate) type RcDiff<'repo> = Rc<git2::Diff<'repo>>;
pub(crate) type RcStat = Rc<git2::DiffStats>;
pub(crate) type RcPatch<'commit> = Rc<git2::Patch<'commit>>;

pub(crate) type CachedDiff<'repo> = LruCache<u8, RcDiff<'repo>>;
pub(crate) type CachedStat = LruCache<u8, RcStat>;
pub(crate) type CachedPatch<'commit> = LruCache<u8, Option<RcPatch<'commit>>>;
