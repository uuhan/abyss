use super::*;

/// Multithread IO support for Files
pub(crate) trait Pio {
    /// Read from a specific offset without changing
    /// the underlying file offset.
    fn pread_exact(&self, to_buf: &mut [u8], offset: LogId) -> io::Result<()>;

    /// Write to a specific offset without changing
    /// the underlying file offset.
    fn pwrite_all(&self, from_buf: &[u8], offset: LogId) -> io::Result<()>;
}

// On systems that support pread/pwrite, use them underneath.
#[cfg(unix)]
mod unix {
    use super::*;
    use std::os::unix::fs::FileExt;

    impl Pio for std::fs::File {
        fn pread_exact(&self, buf: &mut [u8], offset: LogId) -> io::Result<()> {
            self.read_exact_at(buf, offset)
        }

        fn pwrite_all(&self, buf: &[u8], offset: LogId) -> io::Result<()> {
            self.write_all_at(buf, offset)
        }
    }
}

#[cfg(not(unix))]
mod window {
    use super::*;
    use parking_lot::Mutex;
    use std::io::{Read, Seek, Write};

    type MutexInit = fn() -> Mutex<()>;
    static GLOBAL_FILE_LOCK: std::sync::LazyLock<Mutex<()>, MutexInit> =
        std::sync::LazyLock::new(|| Mutex::new(()));

    impl Pio for std::fs::File {
        fn pread_exact(&self, mut buf: &mut [u8], offset: LogId) -> io::Result<()> {
            let _lock = GLOBAL_FILE_LOCK.lock();

            let mut f = self.try_clone()?;

            f.seek(std::io::SeekFrom::Start(offset))?;

            while !buf.is_empty() {
                match f.read(buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        let tmp = buf;
                        buf = &mut tmp[n..];
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
                    Err(e) => return Err(e),
                }
            }
            if !buf.is_empty() {
                Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "failed to fill whole buffer",
                ))
            } else {
                Ok(())
            }
        }

        fn pwrite_all(&self, mut buf: &[u8], offset: LogId) -> io::Result<()> {
            let _lock = GLOBAL_FILE_LOCK.lock();

            let mut f = self.try_clone()?;

            f.seek(std::io::SeekFrom::Start(offset))?;

            while !buf.is_empty() {
                match f.write(buf) {
                    Ok(0) => {
                        return Err(io::Error::new(
                            io::ErrorKind::WriteZero,
                            "failed to write whole buffer",
                        ))
                    }
                    Ok(n) => buf = &buf[n..],
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        }
    }
}
