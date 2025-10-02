use sdl3_sys::clipboard::*;

pub fn has_text() -> bool {
    unsafe { SDL_HasClipboardText() }
}

// TODO: GetClipboardText (SDL allocator?)
