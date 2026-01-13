//! Ada Remote Video Codec
//!
//! Video encoding and decoding using H.264 and VP9.
//! Hardware acceleration used when available.

use ada_remote_core::Result;

/// Video codec type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodecType {
    /// H.264 (AVC) - widely supported, good hardware acceleration
    H264,
    /// VP9 - royalty-free, good compression
    VP9,
}

/// Encoder configuration
#[derive(Debug, Clone)]
pub struct EncoderConfig {
    /// Codec to use
    pub codec: CodecType,
    /// Video width in pixels
    pub width: u32,
    /// Video height in pixels
    pub height: u32,
    /// Target frame rate
    pub fps: u32,
    /// Target bitrate in kbps
    pub bitrate: u32,
    /// Enable hardware acceleration if available
    pub use_hardware_accel: bool,
}

impl Default for EncoderConfig {
    fn default() -> Self {
        Self {
            codec: CodecType::H264,
            width: 1920,
            height: 1080,
            fps: 30,
            bitrate: 2000, // 2 Mbps
            use_hardware_accel: true,
        }
    }
}

/// Decoder configuration
#[derive(Debug, Clone)]
pub struct DecoderConfig {
    /// Expected codec
    pub codec: CodecType,
    /// Enable hardware acceleration if available
    pub use_hardware_accel: bool,
}

impl Default for DecoderConfig {
    fn default() -> Self {
        Self {
            codec: CodecType::H264,
            use_hardware_accel: true,
        }
    }
}

/// Raw video frame (unencoded)
#[derive(Debug, Clone)]
pub struct RawFrame {
    /// RGBA pixel data
    pub data: Vec<u8>,
    /// Frame width
    pub width: u32,
    /// Frame height
    pub height: u32,
    /// Timestamp in microseconds
    pub timestamp: u64,
}

/// Encoded video frame
#[derive(Debug, Clone)]
pub struct EncodedFrame {
    /// Compressed frame data
    pub data: Vec<u8>,
    /// Timestamp in microseconds
    pub timestamp: u64,
    /// Whether this is a keyframe
    pub is_keyframe: bool,
}

/// Video encoder trait
pub trait VideoEncoder: Send + Sync {
    /// Initialize the encoder
    fn init(&mut self, config: EncoderConfig) -> Result<()>;

    /// Encode a raw frame
    fn encode(&mut self, frame: RawFrame) -> Result<EncodedFrame>;

    /// Force generation of a keyframe
    fn force_keyframe(&mut self) -> Result<()>;

    /// Adjust bitrate dynamically
    fn set_bitrate(&mut self, bitrate: u32) -> Result<()>;

    /// Clean up resources
    fn cleanup(&mut self) -> Result<()>;
}

/// Video decoder trait
pub trait VideoDecoder: Send + Sync {
    /// Initialize the decoder
    fn init(&mut self, config: DecoderConfig) -> Result<()>;

    /// Decode an encoded frame
    fn decode(&mut self, frame: EncodedFrame) -> Result<RawFrame>;

    /// Clean up resources
    fn cleanup(&mut self) -> Result<()>;
}

/// Create a video encoder
pub fn create_encoder(codec: CodecType) -> Result<Box<dyn VideoEncoder>> {
    match codec {
        CodecType::H264 => Ok(Box::new(H264Encoder::new())),
        CodecType::VP9 => Ok(Box::new(VP9Encoder::new())),
    }
}

/// Create a video decoder
pub fn create_decoder(codec: CodecType) -> Result<Box<dyn VideoDecoder>> {
    match codec {
        CodecType::H264 => Ok(Box::new(H264Decoder::new())),
        CodecType::VP9 => Ok(Box::new(VP9Decoder::new())),
    }
}

// Stub implementations - will be replaced with FFmpeg bindings

struct H264Encoder {
    config: Option<EncoderConfig>,
}

impl H264Encoder {
    fn new() -> Self {
        Self { config: None }
    }
}

impl VideoEncoder for H264Encoder {
    fn init(&mut self, config: EncoderConfig) -> Result<()> {
        self.config = Some(config);
        tracing::info!("H.264 encoder initialized");
        Ok(())
    }

    fn encode(&mut self, _frame: RawFrame) -> Result<EncodedFrame> {
        // TODO: Implement H.264 encoding using FFmpeg
        Err(ada_remote_core::Error::Encoding(
            "H.264 encoding not yet implemented".to_string(),
        ))
    }

    fn force_keyframe(&mut self) -> Result<()> {
        Ok(())
    }

    fn set_bitrate(&mut self, _bitrate: u32) -> Result<()> {
        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        tracing::info!("H.264 encoder cleaned up");
        Ok(())
    }
}

struct VP9Encoder {
    config: Option<EncoderConfig>,
}

impl VP9Encoder {
    fn new() -> Self {
        Self { config: None }
    }
}

impl VideoEncoder for VP9Encoder {
    fn init(&mut self, config: EncoderConfig) -> Result<()> {
        self.config = Some(config);
        tracing::info!("VP9 encoder initialized");
        Ok(())
    }

    fn encode(&mut self, _frame: RawFrame) -> Result<EncodedFrame> {
        // TODO: Implement VP9 encoding using FFmpeg
        Err(ada_remote_core::Error::Encoding(
            "VP9 encoding not yet implemented".to_string(),
        ))
    }

    fn force_keyframe(&mut self) -> Result<()> {
        Ok(())
    }

    fn set_bitrate(&mut self, _bitrate: u32) -> Result<()> {
        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        tracing::info!("VP9 encoder cleaned up");
        Ok(())
    }
}

struct H264Decoder {
    config: Option<DecoderConfig>,
}

impl H264Decoder {
    fn new() -> Self {
        Self { config: None }
    }
}

impl VideoDecoder for H264Decoder {
    fn init(&mut self, config: DecoderConfig) -> Result<()> {
        self.config = Some(config);
        tracing::info!("H.264 decoder initialized");
        Ok(())
    }

    fn decode(&mut self, _frame: EncodedFrame) -> Result<RawFrame> {
        // TODO: Implement H.264 decoding using FFmpeg
        Err(ada_remote_core::Error::Decoding(
            "H.264 decoding not yet implemented".to_string(),
        ))
    }

    fn cleanup(&mut self) -> Result<()> {
        tracing::info!("H.264 decoder cleaned up");
        Ok(())
    }
}

struct VP9Decoder {
    config: Option<DecoderConfig>,
}

impl VP9Decoder {
    fn new() -> Self {
        Self { config: None }
    }
}

impl VideoDecoder for VP9Decoder {
    fn init(&mut self, config: DecoderConfig) -> Result<()> {
        self.config = Some(config);
        tracing::info!("VP9 decoder initialized");
        Ok(())
    }

    fn decode(&mut self, _frame: EncodedFrame) -> Result<RawFrame> {
        // TODO: Implement VP9 decoding using FFmpeg
        Err(ada_remote_core::Error::Decoding(
            "VP9 decoding not yet implemented".to_string(),
        ))
    }

    fn cleanup(&mut self) -> Result<()> {
        tracing::info!("VP9 decoder cleaned up");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_config_default() {
        let config = EncoderConfig::default();
        assert_eq!(config.codec, CodecType::H264);
        assert_eq!(config.width, 1920);
        assert_eq!(config.height, 1080);
        assert_eq!(config.fps, 30);
    }
}
