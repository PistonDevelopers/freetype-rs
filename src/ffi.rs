
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::{
    c_char,
    c_void,
    c_int,
    c_short,
    c_ushort,
    c_long,
    c_uchar,
    c_uint,
    c_ulong,
    ptrdiff_t,
    size_t,
};

// Basic Data Types
pub type FT_Byte = c_uchar;
pub type FT_Bytes = *const FT_Byte;
pub type FT_Char = c_char;
pub type FT_Int = c_int;
pub type FT_UInt = c_uint;
pub type FT_Int16 = c_short;
pub type FT_UInt16 = c_ushort;
pub type FT_Int32 = i32;
pub type FT_UInt32 = u32;
pub type FT_Int64 = i64;
pub type FT_UInt64 = u64;
pub type FT_Short = c_short;
pub type FT_UShort = c_ushort;
pub type FT_Long = c_long;
pub type FT_ULong = c_ulong;
pub type FT_Bool = c_uchar;
pub type FT_Offset = size_t;
pub type FT_PtrDist = ptrdiff_t;
pub type FT_String = c_char;
pub type FT_Tag = FT_UInt32;
pub type FT_Error = c_int;
pub type FT_Fixed = c_long;
pub type FT_Pointer = *const c_void;
pub type FT_Pos = c_long;
pub type FT_FWord = c_short;
pub type FT_UFWord = c_ushort;
pub type FT_F2Dot14 = c_short;
pub type FT_F26Dot6 = c_long;
pub type FT_Generic_Finalizer = extern fn(*const c_void);
pub type FT_StreamDesc = *const c_void;
pub type FT_Stream_IoFunc = extern fn(FT_Stream, c_ulong, *const c_uchar, c_ulong) -> c_ulong;
pub type FT_Stream_CloseFunc = extern fn(FT_Stream);
pub type FT_Alloc_Func = extern fn(FT_Memory, c_long) -> *mut c_void;
pub type FT_Free_Func = extern fn(FT_Memory, *mut c_void);
pub type FT_Realloc_Func = extern fn(FT_Memory, c_long, c_long, *mut c_void) -> *mut c_void;

// Structs
#[repr(C)]
pub struct FT_Vector {
    pub x: FT_Pos,
    pub y: FT_Pos,
}

#[repr(C)]
pub struct FT_BBox {
    pub xMin: FT_Pos,
    pub yMin: FT_Pos,
    pub xMax: FT_Pos,
    pub yMax: FT_Pos,
}

#[repr(C)]
pub struct FT_Matrix {
    pub xx: FT_Fixed,
    pub xy: FT_Fixed,
    pub yx: FT_Fixed,
    pub yy: FT_Fixed,
}

#[repr(C)]
pub struct FT_UnitVector {
    pub x: FT_F2Dot14,
    pub y: FT_F2Dot14,
}

#[repr(C)]
pub struct FT_Bitmap {
    pub rows: c_int,
    pub width: c_int,
    pub pitch: c_int,
    pub buffer: *const c_uchar,
    pub num_grays: c_short,
    pub pixel_mode: c_char,
    pub palette_mode: c_char,
    pub palette: *const c_void,
}

#[repr(C)]
pub struct FT_Data {
    pub pointer: *const FT_Byte,
    pub length: FT_Int,
}

#[repr(C)]
pub struct FT_Generic {
    pub data: *const c_void,
    pub finalizer: FT_Generic_Finalizer,
}

#[repr(C)]
pub struct FT_Size_Metrics {
    pub x_ppem: FT_UShort,
    pub y_ppem: FT_UShort,

    pub x_scale: FT_Fixed,
    pub y_scale: FT_Fixed,

    pub ascender: FT_Pos,
    pub descender: FT_Pos,
    pub height: FT_Pos,
    pub max_advance: FT_Pos
}

#[repr(C)]
pub struct FT_Outline {
    pub n_contours: c_short,
    pub n_points: c_short,

    pub points: *const FT_Vector,
    pub tags: *const c_char,
    pub contours: *const c_short,

    pub flags: c_int,
}

#[repr(C)]
pub struct FT_Glyph_Metrics {
    pub width: FT_Pos,
    pub height: FT_Pos,

    pub horiBearingX: FT_Pos,
    pub horiBearingY: FT_Pos,
    pub horiAdvance: FT_Pos,

    pub vertBearingX: FT_Pos,
    pub vertBearingY: FT_Pos,
    pub vertAdvance: FT_Pos,
}

#[repr(C)]
pub struct FT_Parameter {
    pub tag: FT_ULong,
    pub data: FT_Pointer,
}

#[repr(C)]
pub struct FT_Open_Args {
    pub flags: FT_UInt,
    pub memory_base: *const FT_Byte,
    pub memory_size: FT_Long,
    pub pathname: *mut FT_String,
    pub stream: FT_Stream,
    pub driver: FT_Module,
    pub num_params: FT_Int,
    pub params: *const FT_Parameter,
}

#[repr(C)]
pub struct FT_Bitmap_Size {
    pub height: FT_Short,
    pub width: FT_Short,

    pub size: FT_Pos,

    pub x_ppem: FT_Pos,
    pub y_ppem: FT_Pos
}

// Enums
pub type FT_Pixel_Mode = c_uint;
pub const FT_PIXEL_MODE_NONE  : FT_Pixel_Mode = 0;
pub const FT_PIXEL_MODE_MONO  : FT_Pixel_Mode = 1;
pub const FT_PIXEL_MODE_GRAY  : FT_Pixel_Mode = 2;
pub const FT_PIXEL_MODE_GRAY2 : FT_Pixel_Mode = 3;
pub const FT_PIXEL_MODE_GRAY4 : FT_Pixel_Mode = 4;
pub const FT_PIXEL_MODE_LCD   : FT_Pixel_Mode = 5;
pub const FT_PIXEL_MODE_LCD_V : FT_Pixel_Mode = 6;
pub const FT_PIXEL_MODE_BGRA  : FT_Pixel_Mode = 7;
pub const FT_PIXEL_MODE_MAX   : FT_Pixel_Mode = 8;

pub type FT_Glyph_Format = c_uint;
pub const FT_GLYPH_FORMAT_NONE      : FT_Glyph_Format = 0;
pub const FT_GLYPH_FORMAT_COMPOSITE : FT_Glyph_Format = 1668246896;
pub const FT_GLYPH_FORMAT_BITMAP    : FT_Glyph_Format = 1651078259;
pub const FT_GLYPH_FORMAT_OUTLINE   : FT_Glyph_Format = 1869968492;
pub const FT_GLYPH_FORMAT_PLOTTER   : FT_Glyph_Format = 1886154612;

pub type FT_Render_Mode = c_uint;
pub const FT_RENDER_MODE_NORMAL : FT_Render_Mode = 0;
pub const FT_RENDER_MODE_LIGHT  : FT_Render_Mode = 1;
pub const FT_RENDER_MODE_MONO   : FT_Render_Mode = 2;
pub const FT_RENDER_MODE_LCD    : FT_Render_Mode = 3;
pub const FT_RENDER_MODE_LCD_V  : FT_Render_Mode = 4;
pub const FT_RENDER_MODE_MAX    : FT_Render_Mode = FT_RENDER_MODE_LCD_V + 1;

pub type FT_Encoding = c_uint;
pub const FT_ENCODING_NONE           : FT_Encoding = 0;
pub const FT_ENCODING_MS_SYMBOL      : FT_Encoding = 1937337698;
pub const FT_ENCODING_UNICODE        : FT_Encoding = 1970170211;
pub const FT_ENCODING_SJIS           : FT_Encoding = 1936353651;
pub const FT_ENCODING_GB2312         : FT_Encoding = 1734484000;
pub const FT_ENCODING_BIG5           : FT_Encoding = 1651074869;
pub const FT_ENCODING_WANSUNG        : FT_Encoding = 2002873971;
pub const FT_ENCODING_JOHAB          : FT_Encoding = 1785686113;
pub const FT_ENCODING_MS_SJIS        : FT_Encoding = 1936353651;
pub const FT_ENCODING_MS_GB2312      : FT_Encoding = 1734484000;
pub const FT_ENCODING_MS_BIG5        : FT_Encoding = 1651074869;
pub const FT_ENCODING_MS_WANSUNG     : FT_Encoding = 2002873971;
pub const FT_ENCODING_MS_JOHAB       : FT_Encoding = 1785686113;
pub const FT_ENCODING_ADOBE_STANDARD : FT_Encoding = 1094995778;
pub const FT_ENCODING_ADOBE_EXPERT   : FT_Encoding = 1094992453;
pub const FT_ENCODING_ADOBE_CUSTOM   : FT_Encoding = 1094992451;
pub const FT_ENCODING_ADOBE_LATIN_1  : FT_Encoding = 1818326065;
pub const FT_ENCODING_OLD_LATIN_2    : FT_Encoding = 1818326066;
pub const FT_ENCODING_APPLE_ROMAN    : FT_Encoding = 1634889070;

pub type FT_Size_Request_Type = c_uint;
pub const FT_SIZE_REQUEST_TYPE_NOMINAL  : FT_Size_Request_Type = 0;
pub const FT_SIZE_REQUEST_TYPE_REAL_DIM : FT_Size_Request_Type = 1;
pub const FT_SIZE_REQUEST_TYPE_BBOX     : FT_Size_Request_Type = 2;
pub const FT_SIZE_REQUEST_TYPE_CELL     : FT_Size_Request_Type = 3;
pub const FT_SIZE_REQUEST_TYPE_SCALES   : FT_Size_Request_Type = 4;
pub const FT_SIZE_REQUEST_TYPE_MAX      : FT_Size_Request_Type = 5;

pub type FT_Kerning_Mode = c_uint;
pub const FT_KERNING_DEFAULT  : FT_Kerning_Mode = 0;
pub const FT_KERNING_UNFITTED : FT_Kerning_Mode = 1;
pub const FT_KERNING_UNSCALED : FT_Kerning_Mode = 2;

pub type FT_Glyph_BBox_Mode = c_uint;
pub const FT_GLYPH_BBOX_UNSCALED  : FT_Glyph_BBox_Mode = 0;
pub const FT_GLYPH_BBOX_SUBPIXELS : FT_Glyph_BBox_Mode = 0;
pub const FT_GLYPH_BBOX_GRIDFIT   : FT_Glyph_BBox_Mode = 1;
pub const FT_GLYPH_BBOX_TRUNCATE  : FT_Glyph_BBox_Mode = 2;
pub const FT_GLYPH_BBOX_PIXELS    : FT_Glyph_BBox_Mode = 3;

// Constants
pub const FT_FACE_FLAG_SCALABLE         : FT_Long = 1 << 0;
pub const FT_FACE_FLAG_FIXED_SIZES      : FT_Long = 1 << 1;
pub const FT_FACE_FLAG_FIXED_WIDTH      : FT_Long = 1 << 2;
pub const FT_FACE_FLAG_SFNT             : FT_Long = 1 << 3;
pub const FT_FACE_FLAG_HORIZONTAL       : FT_Long = 1 << 4;
pub const FT_FACE_FLAG_VERTICAL         : FT_Long = 1 << 5;
pub const FT_FACE_FLAG_KERNING          : FT_Long = 1 << 6;
pub const FT_FACE_FLAG_FAST_GLYPHS      : FT_Long = 1 << 7;
pub const FT_FACE_FLAG_MULTIPLE_MASTERS : FT_Long = 1 << 8;
pub const FT_FACE_FLAG_GLYPH_NAMES      : FT_Long = 1 << 9;
pub const FT_FACE_FLAG_EXTERNAL_STREAM  : FT_Long = 1 << 10;
pub const FT_FACE_FLAG_HINTER           : FT_Long = 1 << 11;
pub const FT_FACE_FLAG_CID_KEYED        : FT_Long = 1 << 12;
pub const FT_FACE_FLAG_TRICKY           : FT_Long = 1 << 13;
pub const FT_FACE_FLAG_COLOR            : FT_Long = 1 << 14;

pub const FT_STYLE_FLAG_ITALIC : FT_Long = 1 << 0;
pub const FT_STYLE_FLAG_BOLD   : FT_Long = 1 << 1;

pub const FT_OPEN_MEMORY   : FT_UInt = 0x1;
pub const FT_OPEN_STREAM   : FT_UInt = 0x2;
pub const FT_OPEN_PATHNAME : FT_UInt = 0x4;
pub const FT_OPEN_DRIVER   : FT_UInt = 0x8;
pub const FT_OPEN_PARAMS   : FT_UInt = 0x10;

pub const FT_SUBGLYPH_FLAG_ARGS_ARE_WORDS     : FT_UInt = 1;
pub const FT_SUBGLYPH_FLAG_ARGS_ARE_XY_VALUES : FT_UInt = 2;
pub const FT_SUBGLYPH_FLAG_ROUND_XY_TO_GRID   : FT_UInt = 4;
pub const FT_SUBGLYPH_FLAG_SCALE              : FT_UInt = 8;
pub const FT_SUBGLYPH_FLAG_XY_SCALE           : FT_UInt = 0x40;
pub const FT_SUBGLYPH_FLAG_2X2                : FT_UInt = 0x80;
pub const FT_SUBGLYPH_FLAG_USE_MY_METRICS     : FT_UInt = 0x200;

pub const FT_LOAD_DEFAULT                     : FT_Int32 = 0x0;
pub const FT_LOAD_NO_SCALE                    : FT_Int32 = 0x1 << 0;
pub const FT_LOAD_NO_HINTING                  : FT_Int32 = 0x1 << 1;
pub const FT_LOAD_RENDER                      : FT_Int32 = 0x1 << 2;
pub const FT_LOAD_NO_BITMAP                   : FT_Int32 = 0x1 << 3;
pub const FT_LOAD_VERTICAL_LAYOUT             : FT_Int32 = 0x1 << 4;
pub const FT_LOAD_FORCE_AUTOHINT              : FT_Int32 = 0x1 << 5;
pub const FT_LOAD_CROP_BITMAP                 : FT_Int32 = 0x1 << 6;
pub const FT_LOAD_PENDANTIC                   : FT_Int32 = 0x1 << 7;
pub const FT_LOAD_IGNORE_GLOBAL_ADVANCE_WIDTH : FT_Int32 = 0x1 << 9;
pub const FT_LOAD_NO_RECURSE                  : FT_Int32 = 0x1 << 10;
pub const FT_LOAD_IGNORE_TRANSFORM            : FT_Int32 = 0x1 << 11;
pub const FT_LOAD_MONOCHROME                  : FT_Int32 = 0x1 << 12;
pub const FT_LOAD_LINEAR_DESIGN               : FT_Int32 = 0x1 << 13;
pub const FT_LOAD_NO_AUTOHINT                 : FT_Int32 = 0x1 << 15;
// Bits 16..19 are used by `FT_LOAD_TARGET`
pub const FT_LOAD_COLOR                       : FT_Int32 = 0x1 << 20;

pub const FT_LOAD_TARGET_NORMAL               : FT_Int32 = (FT_RENDER_MODE_NORMAL << 16) as FT_Int32;
pub const FT_LOAD_TARGET_LIGHT                : FT_Int32 = (FT_RENDER_MODE_LIGHT  << 16) as FT_Int32;
pub const FT_LOAD_TARGET_MONO                 : FT_Int32 = (FT_RENDER_MODE_MONO   << 16) as FT_Int32;
pub const FT_LOAD_TARGET_LCD                  : FT_Int32 = (FT_RENDER_MODE_LCD    << 16) as FT_Int32;
pub const FT_LOAD_TARGET_LCD_V                : FT_Int32 = (FT_RENDER_MODE_LCD_V  << 16) as FT_Int32;

pub const FT_FSTYPE_INSTALLABLE_EMBEDDING        : FT_UShort = 0x0000;
pub const FT_FSTYPE_RESTRICTED_LICENSE_EMBEDDING : FT_UShort = 0x0002;
pub const FT_FSTYPE_PREVIEW_AND_PRINT_EMBEDDING  : FT_UShort = 0x0004;
pub const FT_FSTYPE_EDITABLE_EMBEDDING           : FT_UShort = 0x0008;
pub const FT_FSTYPE_NO_SUBSETTING                : FT_UShort = 0x0100;
pub const FT_FSTYPE_BITMAP_EMBEDDING_ONLY        : FT_UShort = 0x0200;

pub const FT_Err_Ok                            : FT_Error = 0;
pub const FT_Err_Cannot_Open_Resource          : FT_Error = 1;
pub const FT_Err_Unknown_File_Format           : FT_Error = 2;
pub const FT_Err_Invalid_File_Format           : FT_Error = 3;
pub const FT_Err_Invalid_Version               : FT_Error = 4;
pub const FT_Err_Lower_Module_Version          : FT_Error = 5;
pub const FT_Err_Invalid_Argument              : FT_Error = 6;
pub const FT_Err_Unimplemented_Feature         : FT_Error = 7;
pub const FT_Err_Invalid_Table                 : FT_Error = 8;
pub const FT_Err_Invalid_Offset                : FT_Error = 9;
pub const FT_Err_Array_Too_Large               : FT_Error = 10;
pub const FT_Err_Missing_Module                : FT_Error = 11;
pub const FT_Err_Missing_Property              : FT_Error = 12;
pub const FT_Err_Invalid_Glyph_Index           : FT_Error = 16;
pub const FT_Err_Invalid_Character_Code        : FT_Error = 17;
pub const FT_Err_Invalid_Glyph_Format          : FT_Error = 18;
pub const FT_Err_Cannot_Render_Glyph           : FT_Error = 19;
pub const FT_Err_Invalid_Outline               : FT_Error = 20;
pub const FT_Err_Invalid_Composite             : FT_Error = 21;
pub const FT_Err_Too_Many_Hints                : FT_Error = 22;
pub const FT_Err_Invalid_Pixel_Size            : FT_Error = 23;
pub const FT_Err_Invalid_Handle                : FT_Error = 32;
pub const FT_Err_Invalid_Library_Handle        : FT_Error = 33;
pub const FT_Err_Invalid_Driver_Handle         : FT_Error = 34;
pub const FT_Err_Invalid_Face_Handle           : FT_Error = 35;
pub const FT_Err_Invalid_Size_Handle           : FT_Error = 36;
pub const FT_Err_Invalid_Slot_Handle           : FT_Error = 37;
pub const FT_Err_Invalid_CharMap_Handle        : FT_Error = 38;
pub const FT_Err_Invalid_Cache_Handle          : FT_Error = 39;
pub const FT_Err_Invalid_Stream_Handle         : FT_Error = 40;
pub const FT_Err_Too_Many_Drivers              : FT_Error = 48;
pub const FT_Err_Too_Many_Extensions           : FT_Error = 49;
pub const FT_Err_Out_Of_Memory                 : FT_Error = 64;
pub const FT_Err_Unlisted_Object               : FT_Error = 65;
pub const FT_Err_Cannot_Open_Stream            : FT_Error = 81;
pub const FT_Err_Invalid_Stream_Seek           : FT_Error = 82;
pub const FT_Err_Invalid_Stream_Skip           : FT_Error = 83;
pub const FT_Err_Invalid_Stream_Read           : FT_Error = 84;
pub const FT_Err_Invalid_Stream_Operation      : FT_Error = 85;
pub const FT_Err_Invalid_Frame_Operation       : FT_Error = 86;
pub const FT_Err_Nested_Frame_Access           : FT_Error = 87;
pub const FT_Err_Invalid_Frame_Read            : FT_Error = 88;
pub const FT_Err_Raster_Uninitialized          : FT_Error = 96;
pub const FT_Err_Raster_Corrupted              : FT_Error = 97;
pub const FT_Err_Raster_Overflow               : FT_Error = 98;
pub const FT_Err_Raster_Negative_Height        : FT_Error = 99;
pub const FT_Err_Too_Many_Caches               : FT_Error = 112;
pub const FT_Err_Invalid_Opcode                : FT_Error = 128;
pub const FT_Err_Too_Few_Arguments             : FT_Error = 129;
pub const FT_Err_Stack_Overflow                : FT_Error = 130;
pub const FT_Err_Code_Overflow                 : FT_Error = 131;
pub const FT_Err_Bad_Argument                  : FT_Error = 132;
pub const FT_Err_Divide_By_Zero                : FT_Error = 133;
pub const FT_Err_Invalid_Reference             : FT_Error = 134;
pub const FT_Err_Debug_OpCode                  : FT_Error = 135;
pub const FT_Err_ENDF_In_Exec_Stream           : FT_Error = 136;
pub const FT_Err_Nested_DEFS                   : FT_Error = 137;
pub const FT_Err_Invalid_CodeRange             : FT_Error = 138;
pub const FT_Err_Execution_Too_Long            : FT_Error = 139;
pub const FT_Err_Too_Many_Function_Defs        : FT_Error = 140;
pub const FT_Err_Too_Many_Instruction_Defs     : FT_Error = 141;
pub const FT_Err_Table_Missing                 : FT_Error = 142;
pub const FT_Err_Horiz_Header_Missing          : FT_Error = 143;
pub const FT_Err_Locations_Missing             : FT_Error = 144;
pub const FT_Err_Name_Table_Missing            : FT_Error = 145;
pub const FT_Err_CMap_Table_Missing            : FT_Error = 146;
pub const FT_Err_Hmtx_Table_Missing            : FT_Error = 147;
pub const FT_Err_Post_Table_Missing            : FT_Error = 148;
pub const FT_Err_Invalid_Horiz_Metrics         : FT_Error = 149;
pub const FT_Err_Invalid_CharMap_Format        : FT_Error = 150;
pub const FT_Err_Invalid_PPem                  : FT_Error = 151;
pub const FT_Err_Invalid_Vert_Metrics          : FT_Error = 152;
pub const FT_Err_Could_Not_Find_Context        : FT_Error = 153;
pub const FT_Err_Invalid_Post_Table_Format     : FT_Error = 154;
pub const FT_Err_Invalid_Post_Table            : FT_Error = 155;
pub const FT_Err_Syntax_Error                  : FT_Error = 160;
pub const FT_Err_Stack_Underflow               : FT_Error = 161;
pub const FT_Err_Ignore                        : FT_Error = 162;
pub const FT_Err_No_Unicode_Glyph_Name         : FT_Error = 163;
pub const FT_Err_Missing_Startfont_Field       : FT_Error = 176;
pub const FT_Err_Missing_Font_Field            : FT_Error = 177;
pub const FT_Err_Missing_Size_Field            : FT_Error = 178;
pub const FT_Err_Missing_Fontboundingbox_Field : FT_Error = 179;
pub const FT_Err_Missing_Chars_Field           : FT_Error = 180;
pub const FT_Err_Missing_Startchar_Field       : FT_Error = 181;
pub const FT_Err_Missing_Encoding_Field        : FT_Error = 182;
pub const FT_Err_Missing_Bbx_Field             : FT_Error = 183;
pub const FT_Err_Bbx_Too_Big                   : FT_Error = 184;
pub const FT_Err_Corrupted_Font_Header         : FT_Error = 185;
pub const FT_Err_Corrupted_Font_Glyphs         : FT_Error = 186;
pub const FT_Err_Max                           : FT_Error = 187;

// Objects
pub type FT_Library = *mut FT_LibraryRec;
pub type FT_Face = *mut FT_FaceRec;
pub type FT_Size = *mut FT_SizeRec;
pub type FT_GlyphSlot = *mut FT_GlyphSlotRec;
pub type FT_CharMap = *mut FT_CharMapRec;
pub type FT_Module = *mut FT_ModuleRec;
pub type FT_Driver = *mut FT_DriverRec;
pub type FT_Renderer = *mut FT_RendererRec;
pub type FT_Size_Internal = *mut FT_Size_InternalRec;
pub type FT_SubGlyph = *mut FT_SubGlyphRec;
pub type FT_Slot_Internal = *mut FT_Slot_InternalRec;
pub type FT_Size_Request = *mut FT_Size_RequestRec;
pub type FT_Face_Internal = *mut FT_Face_InternalRec;
pub type FT_Stream = *mut FT_StreamRec;
pub type FT_Memory = *const FT_MemoryRec;
pub type FT_ListNode = *mut FT_ListNodeRec;
pub type FT_Glyph = *mut FT_GlyphRec;
pub type FT_BitmapGlyph = *mut FT_BitmapGlyphRec;
pub type FT_OutlineGlyph = *mut FT_OutlineGlyphRec;

// Internal Types
pub type FT_LibraryRec = c_void;
pub type FT_ModuleRec = c_void;
pub type FT_DriverRec = c_void;
pub type FT_RendererRec = c_void;
pub type FT_Size_InternalRec = c_void;
pub type FT_SubGlyphRec = c_void;
pub type FT_Slot_InternalRec = c_void;
pub type FT_Face_InternalRec = c_void;

#[repr(C)]
pub struct FT_CharMapRec {
    pub face: FT_Face,
    pub encoding: FT_Encoding,
    pub platform_id: FT_UShort,
    pub encoding_id: FT_UShort,
}

#[repr(C)]
pub struct FT_FaceRec {
    pub num_faces: FT_Long,
    pub face_index: FT_Long,

    pub face_flags: FT_Long,
    pub style_flags: FT_Long,

    pub num_glyphs: FT_Long,

    pub family_name: *const FT_String,
    pub style_name: *const FT_String,

    pub num_fixed_sizes: FT_Int,
    pub available_sizes: *const FT_Bitmap_Size,

    pub num_charmaps: FT_Int,
    pub charmaps: *const FT_CharMap,

    pub generic: FT_Generic,

    pub bbox: FT_BBox,

    pub units_per_EM: FT_UShort,
    pub ascender: FT_Short,
    pub descender: FT_Short,
    pub height: FT_Short,

    pub max_advance_width: FT_Short,
    pub max_advance_height: FT_Short,

    pub underline_position: FT_Short,
    pub underline_thickness: FT_Short,

    pub glyph: FT_GlyphSlot,
    pub size: FT_Size,
    pub charmap: FT_CharMap,

    /* @private begin */
    pub driver: FT_Driver,
    pub memory: FT_Memory,
    pub stream: FT_Stream,

    pub sizes_list: FT_ListRec,

    pub autohint: FT_Generic,
    pub extensions: *const c_void,

    pub internal: FT_Face_Internal,
    /* @private end */
}

#[repr(C)]
pub struct FT_GlyphSlotRec {
    pub library: FT_Library,
    pub face: FT_Face,
    pub next: FT_GlyphSlot,
    pub reserved: FT_UInt,
    pub generic: FT_Generic,

    pub metrics: FT_Glyph_Metrics,
    pub linearHoriAdvance: FT_Fixed,
    pub linearVertAdvance: FT_Fixed,
    pub advance: FT_Vector,

    pub format: FT_Glyph_Format,

    pub bitmap: FT_Bitmap,
    pub bitmap_left: FT_Int,
    pub bitmap_top: FT_Int,

    pub outline: FT_Outline,

    pub num_subglyphs: FT_UInt,
    pub subglyphs: FT_SubGlyph,

    pub control_data: *const c_void,
    pub control_len: c_long,

    pub lsb_delta: FT_Pos,
    pub rsb_delta: FT_Pos,

    pub other: *const c_void,

    pub internal: FT_Slot_Internal,
}

#[repr(C)]
pub struct FT_SizeRec {
    pub face: FT_Face,
    pub generic: FT_Generic,
    pub metrics: FT_Size_Metrics,
    pub internal: FT_Size_Internal,
}

#[repr(C)]
pub struct FT_StreamRec {
    pub base: *const c_uchar,
    pub size: c_ulong,
    pub pos: c_ulong,

    pub descriptor: FT_StreamDesc,
    pub pathname: FT_StreamDesc,
    pub read: FT_Stream_IoFunc,
    pub close: FT_Stream_CloseFunc,

    pub memory: FT_Memory,
    pub cursor: *const c_uchar,
    pub limit: *const c_uchar,
}

#[repr(C)]
pub struct FT_MemoryRec {
    pub user: *const c_void,
    pub alloc: FT_Alloc_Func,
    pub free: FT_Free_Func,
    pub realloc: FT_Realloc_Func,
}

#[repr(C)]
pub struct FT_ListRec {
    pub head: FT_ListNode,
    pub tail: FT_ListNode,
}

#[repr(C)]
pub struct FT_ListNodeRec {
    pub prev: FT_ListNode,
    pub next: FT_ListNode,
    pub data: *const c_void,
}

#[repr(C)]
pub struct FT_Size_RequestRec {
    pub size_request_type: FT_Size_Request_Type, // type
    pub width: FT_Long,
    pub height: FT_Long,
    pub horiResolution: FT_UInt,
    pub vertResolution: FT_UInt,
}

#[repr(C)]
pub struct FT_GlyphRec {
    pub library: FT_Library,
    pub clazz: *const c_void, // FT_Glyph_Class
    pub format: FT_Glyph_Format,
    pub advance: FT_Vector,
}

#[repr(C)]
pub struct FT_BitmapGlyphRec {
    pub root: FT_GlyphRec,
    pub left: FT_Int,
    pub top: FT_Int,
    pub bitmap: FT_Bitmap,
}

#[repr(C)]
pub struct FT_OutlineGlyphRec {
    pub root: FT_GlyphRec,
    pub outline: FT_Outline,
}

// Macro functions
#[inline(always)]
pub fn FT_HAS_HORIZONTAL(face: FT_Face) -> bool {
    unsafe {
        (*face).face_flags & FT_FACE_FLAG_HORIZONTAL != 0
    }
}

#[inline(always)]
pub fn FT_HAS_VERTICAL(face: FT_Face) -> bool {
    unsafe {
        (*face).face_flags & FT_FACE_FLAG_VERTICAL != 0
    }
}

#[inline(always)]
pub fn FT_HAS_KERNING(face: FT_Face) -> bool {
    unsafe {
        (*face).face_flags & FT_FACE_FLAG_KERNING != 0
    }
}

#[inline(always)]
pub fn FT_IS_SCALABLE(face: FT_Face) -> bool {
    unsafe {
        (*face).face_flags & FT_FACE_FLAG_SCALABLE != 0
    }
}

#[inline(always)]
pub fn FT_IS_SFNT(face: FT_Face) -> bool {
    unsafe {
        (*face).face_flags & FT_FACE_FLAG_SFNT != 0
    }
}

#[inline(always)]
pub fn FT_IS_FIXED_WIDTH(face: FT_Face) -> bool {
    unsafe {
        (*face).face_flags & FT_FACE_FLAG_FIXED_WIDTH != 0
    }
}

#[inline(always)]
pub fn FT_HAS_FIXED_SIZES(face: FT_Face) -> bool {
    unsafe {
        (*face).face_flags & FT_FACE_FLAG_FIXED_SIZES != 0
    }
}

#[inline(always)]
pub fn FT_HAS_GLYPH_NAMES(face: FT_Face) -> bool {
    unsafe {
        (*face).face_flags & FT_FACE_FLAG_GLYPH_NAMES != 0
    }
}

#[inline(always)]
pub fn FT_HAS_MULTIPLE_MASTERS(face: FT_Face) -> bool {
    unsafe {
        (*face).face_flags & FT_FACE_FLAG_MULTIPLE_MASTERS != 0
    }
}

#[inline(always)]
pub fn FT_IS_CID_KEYED(face: FT_Face) -> bool {
    unsafe {
        (*face).face_flags & FT_FACE_FLAG_CID_KEYED != 0
    }
}

#[inline(always)]
pub fn FT_IS_TRICKY(face: FT_Face) -> bool {
    unsafe {
        (*face).face_flags & FT_FACE_FLAG_TRICKY != 0
    }
}

#[inline(always)]
pub fn FT_HAS_COLOR(face: FT_Face) -> bool {
    unsafe {
        (*face).face_flags & FT_FACE_FLAG_COLOR != 0
    }
}

#[link(name = "freetype")]
extern "C" {
    pub fn FT_Init_FreeType(alibrary: *mut FT_Library) -> FT_Error;
    pub fn FT_Done_FreeType(library: FT_Library) -> FT_Error;
    pub fn FT_New_Library(memory: FT_Memory, alibrary: *mut FT_Library) -> FT_Error;
    pub fn FT_Done_Library(library: FT_Library) -> FT_Error;
    pub fn FT_Reference_Library(library: FT_Library) -> FT_Error;
    pub fn FT_Add_Default_Modules(library: FT_Library);
    pub fn FT_New_Face(library: FT_Library, filepathname: *const i8, face_index: FT_Long, aface: *mut FT_Face) -> FT_Error;
    pub fn FT_New_Memory_Face(library: FT_Library, file_base: *const FT_Byte, file_size: FT_Long, face_index: FT_Long, aface: *mut FT_Face) -> FT_Error;
    pub fn FT_Open_Face(library: FT_Library, args: *const FT_Open_Args, face_index: FT_Long, aface: *mut FT_Face) -> FT_Error;
    pub fn FT_Attach_File(face: FT_Face, filepathname: *const c_char) -> FT_Error;
    pub fn FT_Attach_Stream(face: FT_Face, parameters: *mut FT_Open_Args) -> FT_Error;
    pub fn FT_Reference_Face(face: FT_Face) -> FT_Error;
    pub fn FT_Done_Face(face: FT_Face) -> FT_Error;
    pub fn FT_Select_Size(face: FT_Face, strike_index: FT_Int) -> FT_Error;
    pub fn FT_Request_Size(face: FT_Face, req: FT_Size_Request) -> FT_Error;
    pub fn FT_Set_Char_Size(face: FT_Face, char_width: FT_F26Dot6, char_height: FT_F26Dot6, horz_resolution: FT_UInt, vert_resolution: FT_UInt) -> FT_Error;
    pub fn FT_Set_Pixel_Sizes(face: FT_Face, pixel_width: FT_UInt, pixel_height: FT_UInt) -> FT_Error;
    pub fn FT_Load_Glyph(face: FT_Face, glyph_index: FT_UInt, load_flags: FT_Int32) -> FT_Error;
    pub fn FT_Load_Char(face: FT_Face, char_code: FT_ULong, load_flags: FT_Int32) -> FT_Error;
    pub fn FT_Set_Transform(face: FT_Face, matrix: *const FT_Matrix, delta: *const FT_Vector);
    pub fn FT_Render_Glyph(slot: FT_GlyphSlot, render_mode: FT_Render_Mode) -> FT_Error;
    pub fn FT_Get_Kerning(face: FT_Face, left_glyph: FT_UInt, right_glyph: FT_UInt, kern_mode: FT_UInt, akerning: *mut FT_Vector) -> FT_Error;
    pub fn FT_Get_Track_Kerning(face: FT_Face, point_size: FT_Fixed, degree: FT_Int, akerning: *const FT_Fixed) -> FT_Error;
    pub fn FT_Get_Glyph_Name(face: FT_Face, glyph_index: FT_UInt, buffer: FT_Pointer, buffer_max: FT_UInt) -> FT_Error;
    pub fn FT_Get_Postscript_Name(face: FT_Face) -> *const c_char;
    pub fn FT_Select_CharMap(face: FT_Face, encoding: FT_Encoding) -> FT_Error;
    pub fn FT_Set_Charmap(face: FT_Face, charmap: FT_CharMap) -> FT_Error;
    pub fn FT_Get_Charmap_Index(charmap: FT_CharMap) -> FT_Int;
    pub fn FT_Get_Char_Index(face: FT_Face, charcode: FT_ULong) -> FT_UInt;
    pub fn FT_Get_First_Char(face: FT_Face, agindex: *mut FT_UInt) -> FT_ULong;
    pub fn FT_Get_Next_Char(face: FT_Face, char_code: FT_ULong, agindex: *mut FT_UInt) -> FT_ULong;
    pub fn FT_Get_Name_Index(face: FT_Face, glyph_name: *mut FT_String) -> FT_UInt;
    pub fn FT_Get_SubGlyph_Info(glyph: FT_GlyphSlot, sub_index: FT_UInt, p_index: *mut FT_Int, p_flags: *mut FT_UInt, p_arg1: *mut FT_Int, p_arg2: *mut FT_Int, p_transform: *mut FT_Matrix) -> FT_Error;
    pub fn FT_Get_FSType_Flags(face: FT_Face) -> FT_UShort;
    pub fn FT_Get_Glyph(slot: FT_GlyphSlot, aglyph: *mut FT_Glyph) -> FT_Error;
    pub fn FT_Glyph_Copy(source: FT_Glyph, target: *mut FT_Glyph) -> FT_Error;
    pub fn FT_Glyph_Transform(glyph: FT_Glyph, matrix: *const FT_Matrix, delta: *const FT_Vector) -> FT_Error;
    pub fn FT_Glyph_Get_CBox(glyph: FT_Glyph, bbox_mode: FT_UInt, acbox: *const FT_BBox);
    pub fn FT_Glyph_To_Bitmap(the_glyph: *const FT_Glyph, render_mode: FT_Render_Mode, origin: *const FT_Vector, destroy: FT_Bool) -> FT_Error;
    pub fn FT_Done_Glyph(glyph: FT_Glyph);
}

