mod async_buf_read_ext;
mod async_read_ext;
mod async_write_ext;
mod buf_reader;
mod buf_writer;
pub(super) mod chain;
pub(super) mod copy;
pub(super) mod flush;
pub(super) mod lines;
pub(super) mod read;
pub(super) mod read_exact;
pub(super) mod read_line;
pub(super) mod read_to_end;
pub(super) mod read_to_string;
pub(super) mod read_until;
pub(super) mod shutdown;
pub(super) mod take;
pub(super) mod write;
pub(super) mod write_all;

#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use self::async_buf_read_ext::AsyncBufReadExt;
#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use self::async_read_ext::AsyncReadExt;
#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use self::async_write_ext::AsyncWriteExt;
#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use self::buf_reader::BufReader;
#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use self::buf_writer::BufWriter;

// used by `BufReader` and `BufWriter`
// https://github.com/rust-lang/rust/blob/master/src/libstd/sys_common/io.rs#L1
const DEFAULT_BUF_SIZE: usize = 8 * 1024;
