//! API checklist:
//! - [x] TTF_AppendTextString
//! - [x] TTF_CreateText
//! - [x] TTF_DeleteTextString
//! - [x] TTF_DestroyText
//! - [x] TTF_DrawRendererText
//! - [x] TTF_DrawSurfaceText
//! - [x] TTF_GetNextTextSubString
//! - [x] TTF_GetGPUTextDrawData
//! - [x] TTF_GetPreviousTextSubString
//! - [x] TTF_GetTextColor
//! - [x] TTF_GetTextColorFloat
//! - [x] TTF_GetTextDirection
//! - [x] TTF_GetTextFont
//! - [x] TTF_GetTextPosition
//! - [x] TTF_GetTextProperties
//! - [x] TTF_GetTextScript
//! - [x] TTF_GetTextSize
//! - [x] TTF_GetTextSubString
//! - [x] TTF_GetTextSubStringForLine
//! - [x] TTF_GetTextSubStringForPoint
//! - [x] TTF_GetTextSubStringsForRange
//! - [x] TTF_GetTextWrapWidth
//! - [x] TTF_InsertTextString
//! - [x] TTF_SetTextColor
//! - [x] TTF_SetTextColorFloat
//! - [x] TTF_SetTextDirection
//! - [x] TTF_SetTextFont
//! - [x] TTF_SetTextPosition
//! - [x] TTF_SetTextScript
//! - [x] TTF_SetTextString
//! - [x] TTF_SetTextWrapWhitespaceVisible
//! - [x] TTF_SetTextWrapWidth
//! - [x] TTF_TextWrapWhitespaceVisible
//! - [x] TTF_UpdateText
//! - [x] TTF_SetTextEngine
//! - [x] TTF_GetTextEngine

use std::{mem::MaybeUninit, ptr::NonNull};

use sdl3_sys::stdinc::SDL_free;
use sdl3_ttf_sys::ttf::*;

use crate::{
    Result,
    color::{RgbaF32, RgbaU8},
    error::Error,
    impl_enum_transmute,
    properties::{Properties, PropertiesHandle},
    rect::{PointF32, PointI32, RectI32},
    resource::{Handle, Ref, Resource},
    resource_new,
    surface::Surface,
    ttf::{Font, FontHandle, RtStr},
    util::{opt2res, to_result},
};

resource_new!(TTF_Text, Text, TTF_DestroyText);

#[repr(u32)]
#[derive(Clone, Copy)]
#[doc(alias = "TTF_Direction")]
pub enum Direction {
    LeftToRight = TTF_Direction::LTR.0,
    RightToLeft = TTF_Direction::RTL.0,
    TopToBottom = TTF_Direction::TTB.0,
    BottomToTop = TTF_Direction::BTT.0,
}

impl_enum_transmute!(TTF_Direction, Direction);

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    #[doc(alias = "TTF_SubStringFlags")]
    pub struct SubStringFlags: u32 {
        const DIRECTION_MASK = TTF_SubStringFlags::DIRECTION_MASK.0;
        const TEXT_START = TTF_SubStringFlags::TEXT_START.0;
        const LINE_START = TTF_SubStringFlags::LINE_START.0;
        const LINE_END = TTF_SubStringFlags::LINE_END.0;
        const TEXT_END = TTF_SubStringFlags::TEXT_END.0;
    }
}

/// A substring and its position in a [`Text`] object.
#[repr(C)]
#[doc(alias = "TTF_SubString")]
#[derive(Clone, Copy)]
pub struct SubString {
    pub flags: SubStringFlags,
    pub offset: i32,
    pub length: i32,
    pub line_index: i32,
    pub cluster_index: i32,
    pub rect: RectI32,
}

impl From<TTF_SubString> for SubString {
    fn from(value: TTF_SubString) -> Self {
        Self {
            flags: SubStringFlags::from_bits_retain(value.flags.0),
            offset: value.offset,
            length: value.length,
            line_index: value.line_index,
            cluster_index: value.cluster_index,
            rect: RectI32::from_sdl(value.rect),
        }
    }
}

impl TextHandle {
    /// # Safety
    /// Currently provided only for API coverage completeness, without a proper wrapper.
    /// See [`TTF_GetGPUTextDrawData`] docs for more info.
    #[doc(alias = "TTF_GetGPUTextDrawData")]
    pub unsafe fn gpu_draw_data(&self) -> Result<NonNull<TTF_GPUAtlasDrawSequence>> {
        let data = unsafe { TTF_GetGPUTextDrawData(self.as_ptr()) };
        opt2res(NonNull::new(data))
    }

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
    pub fn set_color(&self, color: RgbaU8) -> Result<()> {
        to_result(unsafe {
            TTF_SetTextColor(
                self.as_ptr(),
                color.rgb.r,
                color.rgb.g,
                color.rgb.b,
                color.a,
            )
        })
    }

    #[doc(alias = "TTF_GetTextColorFloat")]
    pub fn color_float(&self) -> RgbaF32 {
        let mut color = MaybeUninit::<RgbaF32>::uninit();
        let ptr = color.as_mut_ptr();
        unsafe {
            TTF_GetTextColorFloat(
                self.as_ptr(),
                &raw mut (*ptr).rgb.r,
                &raw mut (*ptr).rgb.g,
                &raw mut (*ptr).rgb.b,
                &raw mut (*ptr).a,
            );
            color.assume_init()
        }
    }

    #[doc(alias = "TTF_SetTextColorFloat")]
    pub fn set_color_float(&self, color: RgbaF32) -> Result<()> {
        to_result(unsafe {
            TTF_SetTextColorFloat(
                self.as_ptr(),
                color.rgb.r,
                color.rgb.g,
                color.rgb.b,
                color.a,
            )
        })
    }

    #[doc(alias = "TTF_GetTextDirection")]
    pub fn direction(&self) -> Direction {
        Direction::from_sdl(unsafe { TTF_GetTextDirection(self.as_ptr()) })
    }

    #[doc(alias = "TTF_SetTextDirection")]
    pub fn set_direction(&self, direction: Direction) -> Result<()> {
        to_result(unsafe { TTF_SetTextDirection(self.as_ptr(), direction.to_sdl()) })
    }

    #[doc(alias = "TTF_GetTextScript")]
    pub fn script(&self) -> u32 {
        unsafe { TTF_GetTextScript(self.as_ptr()) }
    }

    #[doc(alias = "TTF_SetTextScript")]
    pub fn set_script(&self, script: u32) -> Result<()> {
        to_result(unsafe { TTF_SetTextScript(self.as_ptr(), script) })
    }

    #[doc(alias = "TTF_GetTextPosition")]
    pub fn position(&self) -> Result<PointI32> {
        let mut position = MaybeUninit::<PointI32>::uninit();
        let ptr = position.as_mut_ptr();
        to_result(unsafe {
            TTF_GetTextPosition(self.as_ptr(), &raw mut (*ptr).x, &raw mut (*ptr).y)
        })?;
        Ok(unsafe { position.assume_init() })
    }

    #[doc(alias = "TTF_SetTextPosition")]
    pub fn set_position(&self, position: PointI32) -> Result<()> {
        to_result(unsafe { TTF_SetTextPosition(self.as_ptr(), position.x, position.y) })
    }

    #[doc(alias = "TTF_GetTextWrapWidth")]
    pub fn wrap_width(&self) -> Result<i32> {
        let mut width = 0;
        to_result(unsafe { TTF_GetTextWrapWidth(self.as_ptr(), &raw mut width) })?;
        Ok(width)
    }

    #[doc(alias = "TTF_SetTextWrapWidth")]
    pub fn set_wrap_width(&self, width: i32) -> Result<()> {
        to_result(unsafe { TTF_SetTextWrapWidth(self.as_ptr(), width) })
    }

    #[doc(alias = "TTF_TextWrapWhitespaceVisible")]
    pub fn wrap_whitespace_visible(&self) -> bool {
        unsafe { TTF_TextWrapWhitespaceVisible(self.as_ptr()) }
    }

    #[doc(alias = "TTF_SetTextWrapWhitespaceVisible")]
    pub fn set_wrap_whitespace_visible(&self, visible: bool) -> Result<()> {
        to_result(unsafe { TTF_SetTextWrapWhitespaceVisible(self.as_ptr(), visible) })
    }

    #[doc(alias = "TTF_GetTextFont")]
    pub fn font(&self) -> Result<Ref<'_, Font<'_>>> {
        let font = unsafe { TTF_GetTextFont(self.as_ptr()) };
        let handle = FontHandle::from_ptr(font).ok_or_else(Error::current)?;
        Ok(unsafe { Ref::from_handle(handle) })
    }

    #[doc(alias = "TTF_SetTextFont")]
    pub fn set_font<'a>(&self, font: Option<Ref<'a, Font<'a>>>) -> Result<()> {
        let font = font.map_or(std::ptr::null_mut(), |font| font.as_ptr());
        to_result(unsafe { TTF_SetTextFont(self.as_ptr(), font) })
    }

    #[doc(alias = "TTF_GetTextProperties")]
    pub fn properties(&self) -> Result<Ref<'_, Properties>> {
        let id = unsafe { TTF_GetTextProperties(self.as_ptr()) };
        let handle = PropertiesHandle::from_id(id).ok_or_else(Error::current)?;
        Ok(unsafe { Ref::from_handle(handle) })
    }

    #[doc(alias = "TTF_GetTextSubString")]
    pub fn substring(&self, offset: i32) -> Result<SubString> {
        let mut value = MaybeUninit::uninit();
        to_result(unsafe { TTF_GetTextSubString(self.as_ptr(), offset, value.as_mut_ptr()) })?;
        Ok(SubString::from(unsafe { value.assume_init() }))
    }

    #[doc(alias = "TTF_GetTextSubStringForLine")]
    pub fn substring_for_line(&self, line: i32) -> Result<SubString> {
        let mut value = MaybeUninit::uninit();
        to_result(unsafe { TTF_GetTextSubStringForLine(self.as_ptr(), line, value.as_mut_ptr()) })?;
        Ok(SubString::from(unsafe { value.assume_init() }))
    }

    #[doc(alias = "TTF_GetTextSubStringForPoint")]
    pub fn substring_for_point(&self, point: PointI32) -> Result<SubString> {
        let mut value = MaybeUninit::uninit();
        to_result(unsafe {
            TTF_GetTextSubStringForPoint(self.as_ptr(), point.x, point.y, value.as_mut_ptr())
        })?;
        Ok(SubString::from(unsafe { value.assume_init() }))
    }

    #[doc(alias = "TTF_GetPreviousTextSubString")]
    pub fn previous_substring(&self, value: SubString) -> Result<SubString> {
        let mut previous = MaybeUninit::uninit();
        let value: TTF_SubString = unsafe { std::mem::transmute(value) };
        to_result(unsafe {
            TTF_GetPreviousTextSubString(self.as_ptr(), &raw const value, previous.as_mut_ptr())
        })?;
        Ok(SubString::from(unsafe { previous.assume_init() }))
    }

    #[doc(alias = "TTF_GetNextTextSubString")]
    pub fn next_substring(&self, value: SubString) -> Result<SubString> {
        let mut next = MaybeUninit::uninit();
        let value: TTF_SubString = unsafe { std::mem::transmute(value) };
        to_result(unsafe {
            TTF_GetNextTextSubString(self.as_ptr(), &raw const value, next.as_mut_ptr())
        })?;
        Ok(SubString::from(unsafe { next.assume_init() }))
    }

    #[doc(alias = "TTF_GetTextSubStringsForRange")]
    pub fn substrings_for_range(&self, offset: i32, length: i32) -> Result<Vec<SubString>> {
        let mut count = 0;
        let values =
            unsafe { TTF_GetTextSubStringsForRange(self.as_ptr(), offset, length, &raw mut count) };
        let values = NonNull::new(values).ok_or_else(Error::current)?;
        let values = unsafe { std::slice::from_raw_parts(values.as_ptr(), count.max(0) as usize) };
        let result = values
            .iter()
            .filter_map(|value| unsafe { value.as_ref() })
            .copied()
            .map(SubString::from)
            .collect();
        unsafe { SDL_free(values.as_ptr().cast_mut().cast()) };
        Ok(result)
    }

    #[doc(alias = "TTF_UpdateText")]
    pub fn update(&self) -> Result<()> {
        to_result(unsafe { TTF_UpdateText(self.as_ptr()) })
    }

    #[doc(alias = "TTF_DrawSurfaceText")]
    pub fn draw_to_surface(&self, surf: Ref<Surface>, pos: PointI32) -> Result<()> {
        to_result(unsafe { TTF_DrawSurfaceText(self.as_ptr(), pos.x, pos.y, surf.handle.as_ptr()) })
    }

    #[doc(alias = "TTF_DrawRendererText")]
    pub fn draw_to_renderer(&self, pos: PointF32) -> Result<()> {
        to_result(unsafe { TTF_DrawRendererText(self.as_ptr(), pos.x, pos.y) })
    }

    #[doc(alias = "TTF_SetTextEngine")]
    pub fn set_engine<'this, 'eng, H, T>(&'this self, eng: Ref<'eng, T>) -> Result<()>
    where
        'eng: 'this,
        H: Handle<Raw = *mut TTF_TextEngine>,
        T: Resource<Handle = H>,
    {
        to_result(unsafe { TTF_SetTextEngine(self.as_ptr(), eng.as_raw()) })
    }

    /// # Safety
    /// Currently provided only for API coverage completeness, without a proper wrapper.
    /// See [`TTF_GetTextEngine`] docs for more info.
    #[doc(alias = "TTF_GetTextEngine")]
    pub unsafe fn engine(&self) -> Result<NonNull<TTF_TextEngine>> {
        let eng = unsafe { TTF_GetTextEngine(self.as_ptr()) };
        opt2res(NonNull::new(eng))
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

    #[doc(alias = "TTF_SetTextString")]
    pub fn set_string(&self, text: &str) -> Result<()> {
        let text = RtStr::new(text);
        to_result(unsafe { TTF_SetTextString(self.as_ptr(), text.as_ptr(), text.len()) })
    }

    #[doc(alias = "TTF_InsertTextString")]
    pub fn insert_string(&self, offset: i32, text: &str) -> Result<()> {
        let text = RtStr::new(text);
        to_result(unsafe { TTF_InsertTextString(self.as_ptr(), offset, text.as_ptr(), text.len()) })
    }

    #[doc(alias = "TTF_AppendTextString")]
    pub fn append_string(&self, text: &str) -> Result<()> {
        let text = RtStr::new(text);
        to_result(unsafe { TTF_AppendTextString(self.as_ptr(), text.as_ptr(), text.len()) })
    }

    #[doc(alias = "TTF_DeleteTextString")]
    pub fn delete_string(&self, offset: i32, length: i32) -> Result<()> {
        to_result(unsafe { TTF_DeleteTextString(self.as_ptr(), offset, length) })
    }
}
