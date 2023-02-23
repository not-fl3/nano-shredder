#![allow(warnings)]

pub mod util;
pub mod span;
pub mod live_token;
pub mod live_error;
pub mod live_parser;
pub mod live_node;
pub mod live_node_vec;
pub mod live_document;
pub mod live_file; 
pub mod live_expander;
pub mod live_ptr;
pub mod live_eval;

pub use makepad_math;
pub use makepad_live_tokenizer;
pub use makepad_live_tokenizer::makepad_live_id;

pub use {
    makepad_live_tokenizer::{
        LiveId,
        LiveIdMap
    },
    makepad_live_tokenizer::vec4_ext,
    crate::{
        live_eval::{
            live_eval,
            LiveEval
        },
        live_file::{            
            LiveFile
        },
        live_ptr::{
            LivePtr,
            LiveRef,            
            LiveFileGeneration,
            LiveFileId,
        },
        live_node_vec::{
            LiveNodeSlice,
            LiveNodeVec,
            LiveNodeReader,
        },
        live_node::{
            LiveProp,
            LiveIdAsProp,
            LiveEditInfo,
            LiveValue,
            LiveNode,
            LiveType,
            LiveTypeInfo,
            LiveTypeField,
            LiveFieldKind,
            LiveBinOp,
            LiveUnOp,
            LiveNodeOrigin,
            InlineString,
            FittedString, 
            LivePropType,
            //LiveTypeKind,
        },
        live_token::{TokenWithSpan, LiveToken, LiveTokenId},
        span::{
            TextSpan,
            TokenSpan,
            TextPos
        },
        makepad_live_tokenizer::{LiveErrorOrigin, live_error_origin},
        live_error::{
            LiveError,
            LiveFileError,
            LiveErrorSpan
        },
        live_document::{LiveOriginal, LiveExpanded}
    }
};
