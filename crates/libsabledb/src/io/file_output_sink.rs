use crate::SableError;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct FileResponseSink {
    temp_file: crate::io::TempFile,
    pub fp: tokio::fs::File,
}

impl FileResponseSink {
    pub const BUFFER_SIZE: usize = 4096;
    pub async fn new() -> Result<Self, SableError> {
        let temp_file = crate::io::TempFile::with_name("tmp_sink");
        let fp = tokio::fs::File::create(&temp_file.fullpath()).await?;
        Ok(FileResponseSink { temp_file, fp })
    }

    pub async fn read_all(&mut self) -> Result<bytes::BytesMut, SableError> {
        self.read_all_with_size(Self::BUFFER_SIZE).await
    }

    pub async fn read_all_as_string(&mut self) -> Result<String, SableError> {
        let as_bytes = self.read_all_with_size(Self::BUFFER_SIZE).await?;
        Ok(crate::BytesMutUtils::to_string(&as_bytes))
    }

    pub async fn read_all_with_size(&mut self, size: usize) -> Result<bytes::BytesMut, SableError> {
        // The response file is a short-lived buffer, not durable state. Waiting
        // for an fsync here places the backing filesystem's durability latency
        // on every MULTI/EXEC response even though the file is deleted as soon
        // as the response has been copied to the client. `flush` completes any
        // pending writes so the second file descriptor sees the bytes without
        // forcing this disposable file to stable storage.
        self.fp.flush().await?;
        let mut fp = tokio::fs::File::open(&self.temp_file.fullpath()).await?;

        let mut buffer = bytes::BytesMut::with_capacity(size);
        fp.read_buf(&mut buffer).await?;
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::FileResponseSink;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn flushed_response_is_visible_to_the_reader() {
        let mut sink = FileResponseSink::new().await.unwrap();
        sink.fp.write_all(b"*2\r\n+OK\r\n:1\r\n").await.unwrap();

        let response = sink.read_all_with_size(64).await.unwrap();

        assert_eq!(response.as_ref(), b"*2\r\n+OK\r\n:1\r\n");
    }
}
