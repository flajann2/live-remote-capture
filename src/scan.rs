//! scan for video and audio sources

use super::*;
use std::mem;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanning() {
        let mut avo : AVOutputFormat;
        unsafe {
            let p_avo : *mut AVOutputFormat = mem::transmute(&avo);
            let _av = av_output_video_device_next(p_avo);
        }
    }
}
