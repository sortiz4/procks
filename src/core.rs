use clap::CommandFactory;
use clap::Parser;
use astd::io as aio;
use astd::net::TcpListener;
use astd::net::TcpStream;
use astd::net::UdpSocket;
use astd::task;
use futures::try_join;
use std::ffi::OsString;
use std::io;
use std::io::Stderr;
use std::io::Write;
use super::Error;
use super::Result;

#[derive(Debug, Parser)]
#[command(version, disable_help_flag = true, disable_version_flag = true, about = "A simple proxy server.")]
struct Options {
    /// The size of the internal buffer (in bytes).
    #[arg(short = 'b', long = "buffer", default_value = "4096")]
    buffer: usize,

    /// The protocol to use (TCP or UDP).
    #[arg(short = 'p', long = "protocol")]
    protocol: Option<String>,

    /// Where traffic will be received.
    #[arg(short = 'r', long = "receive")]
    receive: Option<String>,

    /// Where traffic will be sent.
    #[arg(short = 's', long = "send")]
    send: Option<String>,

    /// Show this message.
    #[arg(short = 'h', long = "help")]
    help: bool,

    /// Show the version.
    #[arg(short = 'v', long = "version")]
    version: bool,
}

pub struct Procks {
    options: Options,
    stderr: Stderr,
}

impl Procks {
    const PROTO_TCP: &'static str = "tcp";
    const PROTO_UDP: &'static str = "udp";

    /// Constructs this program from an iterable of arguments.
    pub fn from_iter<I>(iter: I) -> Result<Self>
    where
        Self: Sized,
        I: IntoIterator,
        I::Item: Into<OsString> + Clone,
    {
        return Ok(
            Self {
                options: Options::try_parse_from(iter)?,
                stderr: io::stderr(),
            }
        );
    }

    /// Replaces the standard error stream for this program.
    pub fn stderr(&mut self, stderr: Stderr) -> &mut Self {
        self.stderr = stderr;
        return self;
    }

    /// Runs this program and writes all errors.
    pub async fn run(&mut self) -> Result<()> {
        let run = async {
            // Write the help or version message
            if self.options.help {
                return self.help();
            }

            if self.options.version {
                return self.version();
            }

            // Validate the options
            self.validate()?;

            // Launch a proxy server
            return match self.options.protocol.as_ref().unwrap().to_lowercase().as_str() {
                Self::PROTO_TCP => self.tcp().await,
                Self::PROTO_UDP => self.udp().await,
                _ => Ok(()),
            };
        };

        match run.await {
            Ok(val) => {
                return Ok(val);
            },
            Err(err) => {
                writeln!(self.stderr, "Error: {}", err)?;
                return Err(err);
            },
        }
    }

    /// Writes the help message to the standard error stream.
    fn help(&mut self) -> Result<()> {
        write!(self.stderr, "{}", Options::command().render_help())?;
        return Ok(());
    }

    /// Writes the version message to the standard error stream.
    fn version(&mut self) -> Result<()> {
        write!(self.stderr, "{}", Options::command().render_version())?;
        return Ok(());
    }

    /// Validates the options.
    fn validate(&self) -> Result<()> {
        return if self.options.protocol.is_none() || self.options.receive.is_none() || self.options.send.is_none() {
            Err(Error::Missing)
        } else {
            Ok(())
        };
    }

    /// Launches a TCP proxy server.
    async fn tcp(&mut self) -> Result<()> {
        let send_addr = self.options.send.as_ref().unwrap();
        let recv_addr = self.options.receive.as_ref().unwrap();

        // Create the listener
        let listener = TcpListener::bind(recv_addr).await?;

        loop {
            // Create the streams
            let recv_stream = listener.accept().await?.0;
            let send_stream = TcpStream::connect(send_addr).await?;

            task::spawn(
                async move {
                    // Set up the readers and writers
                    let (send_reader, send_writer) = &mut (&send_stream, &send_stream);
                    let (recv_reader, recv_writer) = &mut (&recv_stream, &recv_stream);

                    // Forward the traffic
                    let _ = try_join!(
                        aio::copy(recv_reader, send_writer),
                        aio::copy(send_reader, recv_writer),
                    );
                }
            );
        }
    }

    /// Launches a UDP proxy server.
    async fn udp(&mut self) -> Result<()> {
        let send_addr = self.options.send.as_ref().unwrap();
        let recv_addr = self.options.receive.as_ref().unwrap();

        // Create the buffer and socket
        let mut buf = vec![0u8; self.options.buffer];
        let socket = UdpSocket::bind(recv_addr).await?;

        loop {
            // Forward the traffic
            let size = socket.recv_from(&mut buf).await?.0;
            socket.send_to(&buf[..size], send_addr).await?;
        }
    }
}
