use libc::c_long;
use *;

#[repr(C)]
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum Channel {
  Invalid           = -1,
  Mono              = 0,
  Left              = 1,
  Right             = 2,
  Center            = 3,
  LeftSurround      = 4,
  RightSurround     = 5,
  RearLeftSurround  = 6,
  RearCenter        = 7,
  RearRightSurround = 8,
  LowFrequency      = 9,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ChannelMap {
    pub channels: u32,
    pub map: [Channel;10],
}
impl ::std::default::Default for ChannelMap {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

static CHANNEL_LAYOUT_UNDEFINED: &'static [Channel] = &[ Channel::Invalid ];
static CHANNEL_LAYOUT_DUAL_MONO: &'static [Channel] = &[ Channel::Left, Channel::Right ];
static CHANNEL_LAYOUT_DUAL_MONO_LFE: &'static [Channel] = &[ Channel::Left, Channel::Right, Channel::LowFrequency ];
static CHANNEL_LAYOUT_MONO: &'static [Channel] = &[ Channel::Mono ];
static CHANNEL_LAYOUT_MONO_LFE: &'static [Channel] = &[ Channel::Mono, Channel::LowFrequency ];
static CHANNEL_LAYOUT_STEREO: &'static [Channel] = &[ Channel::Left, Channel::Right ];
static CHANNEL_LAYOUT_STEREO_LFE: &'static [Channel] = &[ Channel::Left, Channel::Right, Channel::LowFrequency ];
static CHANNEL_LAYOUT_3F: &'static [Channel] = &[ Channel::Left, Channel::Right, Channel::Center ];
static CHANNEL_LAYOUT_3FLFE: &'static [Channel] = &[ Channel::Left, Channel::Right, Channel::Center, Channel::LowFrequency ];
static CHANNEL_LAYOUT_2F1: &'static [Channel] = &[ Channel::Left, Channel::Right, Channel::RearCenter ];
static CHANNEL_LAYOUT_2F1LFE: &'static [Channel] = &[ Channel::Left, Channel::Right, Channel::LowFrequency, Channel::RearCenter ];
static CHANNEL_LAYOUT_3F1: &'static [Channel] = &[ Channel::Left, Channel::Right, Channel::Center, Channel::RearCenter ];
static CHANNEL_LAYOUT_3F1LFE: &'static [Channel] = &[ Channel::Left, Channel::Right, Channel::Center, Channel::LowFrequency, Channel::RearCenter ];
static CHANNEL_LAYOUT_2F2: &'static [Channel] = &[ Channel::Left, Channel::Right, Channel::LeftSurround, Channel::RightSurround ];
static CHANNEL_LAYOUT_2F2LFE: &'static [Channel] = &[ Channel::Left, Channel::Right, Channel::LowFrequency, Channel::LeftSurround, Channel::RightSurround ];
static CHANNEL_LAYOUT_3F2: &'static [Channel] = &[ Channel::Left, Channel::Right, Channel::Center, Channel::LeftSurround, Channel::RightSurround ];
static CHANNEL_LAYOUT_3F2LFE: &'static [Channel] = &[ Channel::Left, Channel::Right, Channel::Center, Channel::LowFrequency, Channel::LeftSurround, Channel::RightSurround ];
static CHANNEL_LAYOUT_3F3RLFE: &'static [Channel] = &[ Channel::Left, Channel::Right, Channel::Center, Channel::LowFrequency, Channel::RearCenter, Channel::LeftSurround, Channel::RightSurround ];
static CHANNEL_LAYOUT_3F4LFE: &'static [Channel] = &[ Channel::Left, Channel::Right, Channel::Center, Channel::LowFrequency, Channel::RearLeftSurround, Channel::RearRightSurround, Channel::LeftSurround, Channel::RightSurround ];

pub fn channel_index_to_order(layout: ChannelLayout) -> &'static [Channel]
{
    match layout {
        ChannelLayout::DualMono => CHANNEL_LAYOUT_DUAL_MONO,
        ChannelLayout::DualMonoLfe => CHANNEL_LAYOUT_DUAL_MONO_LFE,
        ChannelLayout::Mono => CHANNEL_LAYOUT_MONO,
        ChannelLayout::MonoLfe => CHANNEL_LAYOUT_MONO_LFE,
        ChannelLayout::Stereo => CHANNEL_LAYOUT_STEREO,
        ChannelLayout::StereoLfe => CHANNEL_LAYOUT_STEREO_LFE,
        ChannelLayout::Layout3F => CHANNEL_LAYOUT_3F,
        ChannelLayout::Layout3FLfe => CHANNEL_LAYOUT_3FLFE,
        ChannelLayout::Layout2F1 => CHANNEL_LAYOUT_2F1,
        ChannelLayout::Layout2F1Lfe => CHANNEL_LAYOUT_2F1LFE,
        ChannelLayout::Layout3F1 => CHANNEL_LAYOUT_3F1,
        ChannelLayout::Layout3F1Lfe => CHANNEL_LAYOUT_3F1LFE,
        ChannelLayout::Layout2F2 => CHANNEL_LAYOUT_2F2,
        ChannelLayout::Layout2F2Lfe => CHANNEL_LAYOUT_2F2LFE,
        ChannelLayout::Layout3F2 => CHANNEL_LAYOUT_3F2,
        ChannelLayout::Layout3F2Lfe => CHANNEL_LAYOUT_3F2LFE,
        ChannelLayout::Layout3F3RLfe => CHANNEL_LAYOUT_3F3RLFE,
        ChannelLayout::Layout3F4Lfe => CHANNEL_LAYOUT_3F4LFE,
        _ => CHANNEL_LAYOUT_UNDEFINED,
    }
}

extern "C" {
    pub fn cubeb_channel_map_to_layout(channel_map: *const ChannelMap) -> ChannelLayout;
    pub fn cubeb_should_upmix(stream: *const StreamParams, mixer: *const StreamParams) -> bool;
    pub fn cubeb_should_downmix(stream: *const StreamParams, mixer: *const StreamParams) -> bool;
    pub fn cubeb_downmix_float(input: *const f32, inframes: c_long, output: *mut f32,
                               in_channels: u32, out_channels: u32,
                               in_layout: ChannelLayout, out_layout: ChannelLayout);
    pub fn cubeb_upmix_float(input: *const f32, inframes: c_long, output: *mut f32,
                             in_channels: u32, out_channels: u32);
}

#[test]
fn test_layout_cubeb_channel_map() {
    assert_eq!(::std::mem::size_of::<ChannelMap>(),
               44usize,
               concat!("Size of: ", stringify!(ChannelMap)));
    assert_eq!(::std::mem::align_of::<ChannelMap>(),
               4usize,
               concat!("Alignment of ", stringify!(ChannelMap)));
    assert_eq!(unsafe { &(*(0 as *const ChannelMap)).channels as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(ChannelMap),
                       "::",
                       stringify!(channels)));
    assert_eq!(unsafe { &(*(0 as *const ChannelMap)).map as *const _ as usize },
               4usize,
               concat!("Alignment of field: ",
                       stringify!(ChannelMap),
                       "::",
                       stringify!(map)));
}
