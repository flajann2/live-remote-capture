//! Live Remote Capture -- core library of functions

#![allow(unused)]
use std::convert::AsRef;
use std::path::{PathBuf, Path};
pub use std::ffi::{CStr, CString};
pub use std::os::raw::{c_char, c_int};
pub use libc::{size_t, c_float, c_void};
pub use ffmpeg4_ffi::sys::{
    self,
    av_output_video_device_next,
    av_output_audio_device_next,
    AVFrame,
    AVDictionary,
    AVCodec,
    AVCodecContext,
    AVStream,
    AVPacket,
    AVFormatContext,
    AVOutputFormat,
    AVDeviceCapabilitiesQuery,
    AVDeviceInfo,
    AVDeviceInfoList,
    AVCodecParameters,
    AVCodecParserContext,
    AVMediaType,
    AVMediaType_AVMEDIA_TYPE_UNKNOWN as AVMEDIA_TYPE_UNKNOWN,
    AVMediaType_AVMEDIA_TYPE_VIDEO as AVMEDIA_TYPE_VIDEO,
    AVMediaType_AVMEDIA_TYPE_AUDIO as AVMEDIA_TYPE_AUDIO,
    AVMediaType_AVMEDIA_TYPE_DATA as AVMEDIA_TYPE_DATA,
    AVMediaType_AVMEDIA_TYPE_SUBTITLE as AVMEDIA_TYPE_SUBTITLE,
    AVMediaType_AVMEDIA_TYPE_ATTACHMENT as AVMEDIA_TYPE_ATTACHMENT,
    AVMediaType_AVMEDIA_TYPE_NB as AVMEDIA_TYPE_NB,
    AVFMT_NOFILE,
    AVIO_FLAG_WRITE,
    AVRounding_AV_ROUND_NEAR_INF as AV_ROUND_NEAR_INF,
    AVRounding_AV_ROUND_PASS_MINMAX as AV_ROUND_PASS_MINMAX,
    AVCodecID_AV_CODEC_ID_H264 as AV_CODEC_ID_H264,
    AV_INPUT_BUFFER_PADDING_SIZE,
};

pub mod scan;
pub mod stream;
pub mod capture;
