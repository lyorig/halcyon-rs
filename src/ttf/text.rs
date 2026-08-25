//! API checklist:
//! - [ ] TTF_SetTextEngine
//! - [ ] TTF_GetTextEngine

use std::mem::MaybeUninit;

use sdl3_ttf_sys::ttf::*;

use crate::{
    Result,
    color::RgbaU8,
    rect::{PointF32, PointI32},
    resource::Ref,
    resource_new,
    surface::Surface,
    ttf::{Font, RtStr},
    util::to_result,
};

resource_new!(TTF_Text, Text, TTF_DestroyText);

impl TextHandle {
    #[doc(alias = "TTF_GetTextSize")]
    pub fn size(&self) -> PointI32 {
        let mut ret = MaybeUninit::<PointI32>::uninit();
        let ptr = ret.as_mut_ptr();

        unsafe {
            TTF_GetTextSize(self.handle.as_ptr(), &raw mut (*ptr).x, &raw mut (*ptr).y);
            ret.assume_init()
        }
    }

    #[doc(alias = "TTF_GetTextColor")]
    pub fn color(&self) -> RgbaU8 {
        let mut col = MaybeUninit::<RgbaU8>::uninit();
        let ptr = col.as_mut_ptr();

        unsafe {
            TTF_GetTextColor(
                self.handle.as_ptr(),
                &raw mut (*ptr).rgb.r,
                &raw mut (*ptr).rgb.g,
                &raw mut (*ptr).rgb.b,
                &raw mut (*ptr).a,
            );

            col.assume_init()
        }
    }

    #[doc(alias = "TTF_SetTextColor")]
    pub fn set_color(&self, color: RgbaU8) -> Result {
        to_result(unsafe {
            TTF_SetTextColor(
                self.handle.as_ptr(),
                color.rgb.r,
                color.rgb.g,
                color.rgb.b,
                color.a,
            )
        })
    }

    #[doc(alias = "TTF_UpdateText")]
    pub fn update(&self) -> Result {
        to_result(unsafe { TTF_UpdateText(self.handle.as_ptr()) })
    }

    #[doc(alias = "TTF_DrawSurfaceText")]
    pub fn draw_to_surface(&self, surf: Ref<Surface>, pos: PointI32) -> Result {
        to_result(unsafe {
            TTF_DrawSurfaceText(self.handle.as_ptr(), pos.x, pos.y, surf.handle.as_ptr())
        })
    }

    #[doc(alias = "TTF_DrawRendererText")]
    pub fn draw_to_renderer(&self, pos: PointF32) -> Result {
        to_result(unsafe { TTF_DrawRendererText(self.handle.as_ptr(), pos.x, pos.y) })
    }
}

impl Text {
    #[doc(alias = "TTF_CreateText")]
    pub fn new(font: Ref<Font>, text: &str) -> Result<Self> {
        let text = RtStr::new(text);

        Self::from_ptr(unsafe {
            TTF_CreateText(
                std::ptr::null_mut(),
                font.handle.as_ptr(),
                text.as_ptr(),
                text.len(),
            )
        })
    }
}
