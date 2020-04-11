
use futures_util::future;

pub struct Communicator<RW : AsyncWrite + AsyncRead> {
  write_buffer  : VecDeque<u8>,
  read_expected : usize,
  connection    : RW,
}

type Result = tokio::io::Result;

trait Outputter {
  fn done(&self) -> bool;
  fn got(&mut self, count : usize);
  fn buf(&mut self) -> &mut [u8];
}

impl Communicator<RW : AsyncWrite + AsyncRead> {
  pub async fn communicate<RW>(&mut self, cmd : &u8, response : &mut u8)
                               -> Result<()> {
    self.drain_write_buffer().await?;
    self.ignore_stale_responses().await?;
    self.assert_idle();
    self.write_buffer.append(cmd);
    self.read_expected = response.len();
    self.read_into(response).await?;
    self.assert_idle();
    Ok(())
  }

  fn assert_idle(&mut self) {
    assert!(write_buffer.is_empty());
    assert!(read_expected == 0);
  }

  async fn read_into<O : Outputter>(&mut self, &mut o : O) -> Result<()> {
    loop {
      if o.done() { break }
      future::poll_fn(|cx|{
        let got = self.connection.poll_read(cx, o.buf());
        if let Ready(Ok(n)) = got { o.got(n) }
        else { return got }
      }).await?;
    }
    Ok(())
  }
}
