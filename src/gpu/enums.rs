//! There are a LOT of boolenums in the GPU module.
//! This is a place to aggregate them all, since some are reusable.

use crate::boolenum;

boolenum!(Cycle);
boolenum!(CycleResolveTexture);
boolenum!(EnableAlphaToCoverage);
boolenum!(EnableAnisotropy);
boolenum!(EnableBlend);
boolenum!(EnableColorWriteMask);
boolenum!(EnableCompare);
boolenum!(EnableDebug);
boolenum!(EnableDepthBias);
boolenum!(EnableDepthClip);
boolenum!(EnableDepthTest);
boolenum!(EnableDepthWrite);
boolenum!(EnableStencilTest);
boolenum!(HasDepthStencilTarget);
boolenum!(WaitAll);
