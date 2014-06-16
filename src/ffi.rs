
#![allow(non_camel_case_types)]
#![allow(non_snake_case_functions)]

use libc::*;

// Basic Data Types
pub type FT_Byte = c_uchar;
pub type FT_Bytes = *FT_Byte;
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
pub type FT_Pointer = *c_void;
pub type FT_Pos = c_long;
pub type FT_FWord = c_short;
pub type FT_UFWord = c_ushort;
pub type FT_F2Dot14 = c_short;
pub type FT_F26Dot6 = c_long;
pub type FT_Generic_Finalizer = extern fn(*c_void);
pub type FT_StreamDesc = *c_void;
pub type FT_Stream_IoFunc = extern fn(FT_Stream, c_ulong, *c_uchar, c_ulong) -> c_ulong;
pub type FT_Stream_CloseFunc = extern fn(FT_Stream);
pub type FT_Alloc_Func = extern fn(FT_Memory, c_long) -> *c_void;
pub type FT_Free_Func = extern fn(FT_Memory, *c_void);
pub type FT_Realloc_Func = extern fn(FT_Memory, c_long, c_long, *c_void) -> *c_void;

// Structs
pub struct FT_Vector {
    pub x: FT_Pos,
    pub y: FT_Pos,
}

pub struct FT_BBox {
    pub xMin: FT_Pos,
    pub yMin: FT_Pos,
    pub xMax: FT_Pos,
    pub yMax: FT_Pos,
}

pub struct FT_Matrix {
    pub xx: FT_Fixed,
    pub xy: FT_Fixed,
    pub yx: FT_Fixed,
    pub yy: FT_Fixed,
}

pub struct FT_UnitVector {
    pub x: FT_F2Dot14,
    pub y: FT_F2Dot14,
}

pub struct FT_Bitmap {
    pub rows: c_int,
    pub width: c_int,
    pub pitch: c_int,
    pub buffer: *c_uchar,
    pub num_grays: c_short,
    pub pixel_mode: c_char,
    pub palette_mode: c_char,
    pub palette: *c_void,
}

pub struct FT_Data {
    pub pointer: *FT_Byte,
    pub length: FT_Int,
}

pub struct FT_Generic {
    pub data: *c_void,
    pub finalizer: FT_Generic_Finalizer,
}

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

pub struct FT_Outline {
    pub n_contours: c_short,
    pub n_points: c_short,

    pub points: *FT_Vector,
    pub tags: *c_char,
    pub contours: *c_short,

    pub flags: c_int,
}

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

pub struct FT_Parameter {
    pub tag: FT_ULong,
    pub data: FT_Pointer,
}

pub struct FT_Open_Args {
    pub flags: FT_UInt,
    pub memory_base: *FT_Byte,
    pub memory_size: FT_Long,
    pub pathname: *mut FT_String,
    pub stream: FT_Stream,
    pub driver: FT_Module,
    pub num_params: FT_Int,
    pub params: *FT_Parameter,
}

pub struct FT_Bitmap_Size {
    pub height: FT_Short,
    pub width: FT_Short,

    pub size: FT_Pos,

    pub x_ppem: FT_Pos,
    pub y_ppem: FT_Pos
}

// Enums
pub type FT_Pixel_Mode = c_uint;
pub static FT_PIXEL_MODE_NONE  : FT_Pixel_Mode = 0;
pub static FT_PIXEL_MODE_MONO  : FT_Pixel_Mode = 1;
pub static FT_PIXEL_MODE_GRAY  : FT_Pixel_Mode = 2;
pub static FT_PIXEL_MODE_GRAY2 : FT_Pixel_Mode = 3;
pub static FT_PIXEL_MODE_GRAY4 : FT_Pixel_Mode = 4;
pub static FT_PIXEL_MODE_LCD   : FT_Pixel_Mode = 5;
pub static FT_PIXEL_MODE_LCD_V : FT_Pixel_Mode = 6;
pub static FT_PIXEL_MODE_BGRA  : FT_Pixel_Mode = 7;
pub static FT_PIXEL_MODE_MAX   : FT_Pixel_Mode = 8;

pub type FT_Glyph_Format = c_uint;
pub static FT_GLYPH_FORMAT_NONE      : FT_Glyph_Format = 0;
pub static FT_GLYPH_FORMAT_COMPOSITE : FT_Glyph_Format = 1668246896;
pub static FT_GLYPH_FORMAT_BITMAP    : FT_Glyph_Format = 1651078259;
pub static FT_GLYPH_FORMAT_OUTLINE   : FT_Glyph_Format = 1869968492;
pub static FT_GLYPH_FORMAT_PLOTTER   : FT_Glyph_Format = 1886154612;

pub type FT_Render_Mode = c_uint;
pub static FT_RENDER_MODE_NORMAL : FT_Render_Mode = 0;
pub static FT_RENDER_MODE_LIGHT  : FT_Render_Mode = 1;
pub static FT_RENDER_MODE_MONO   : FT_Render_Mode = 2;
pub static FT_RENDER_MODE_LCD    : FT_Render_Mode = 3;
pub static FT_RENDER_MODE_LCD_V  : FT_Render_Mode = 4;
pub static FT_RENDER_MODE_MAX    : FT_Render_Mode = FT_RENDER_MODE_LCD_V + 1;

pub type FT_Encoding = c_uint;
pub static FT_ENCODING_NONE           : FT_Encoding = 0;
pub static FT_ENCODING_MS_SYMBOL      : FT_Encoding = 1937337698;
pub static FT_ENCODING_UNICODE        : FT_Encoding = 1970170211;
pub static FT_ENCODING_SJIS           : FT_Encoding = 1936353651;
pub static FT_ENCODING_GB2312         : FT_Encoding = 1734484000;
pub static FT_ENCODING_BIG5           : FT_Encoding = 1651074869;
pub static FT_ENCODING_WANSUNG        : FT_Encoding = 2002873971;
pub static FT_ENCODING_JOHAB          : FT_Encoding = 1785686113;
pub static FT_ENCODING_MS_SJIS        : FT_Encoding = 1936353651;
pub static FT_ENCODING_MS_GB2312      : FT_Encoding = 1734484000;
pub static FT_ENCODING_MS_BIG5        : FT_Encoding = 1651074869;
pub static FT_ENCODING_MS_WANSUNG     : FT_Encoding = 2002873971;
pub static FT_ENCODING_MS_JOHAB       : FT_Encoding = 1785686113;
pub static FT_ENCODING_ADOBE_STANDARD : FT_Encoding = 1094995778;
pub static FT_ENCODING_ADOBE_EXPERT   : FT_Encoding = 1094992453;
pub static FT_ENCODING_ADOBE_CUSTOM   : FT_Encoding = 1094992451;
pub static FT_ENCODING_ADOBE_LATIN_1  : FT_Encoding = 1818326065;
pub static FT_ENCODING_OLD_LATIN_2    : FT_Encoding = 1818326066;
pub static FT_ENCODING_APPLE_ROMAN    : FT_Encoding = 1634889070;

pub static FT_LOAD_DEFAULT                     : FT_Int32 = 0x0;
pub static FT_LOAD_NO_SCALE                    : FT_Int32 = 0x1 << 0;
pub static FT_LOAD_NO_HINTING                  : FT_Int32 = 0x1 << 1;
pub static FT_LOAD_RENDER                      : FT_Int32 = 0x1 << 2;
pub static FT_LOAD_NO_BITMAP                   : FT_Int32 = 0x1 << 3;
pub static FT_LOAD_VERTICAL_LAYOUT             : FT_Int32 = 0x1 << 4;
pub static FT_LOAD_FORCE_AUTOHINT              : FT_Int32 = 0x1 << 5;
pub static FT_LOAD_CROP_BITMAP                 : FT_Int32 = 0x1 << 6;
pub static FT_LOAD_PENDANTIC                   : FT_Int32 = 0x1 << 7;
pub static FT_LOAD_IGNORE_GLOBAL_ADVANCE_WIDTH : FT_Int32 = 0x1 << 9;
pub static FT_LOAD_NO_RECURSE                  : FT_Int32 = 0x1 << 10;
pub static FT_LOAD_IGNORE_TRANSFORM            : FT_Int32 = 0x1 << 11;
pub static FT_LOAD_MONOCHROME                  : FT_Int32 = 0x1 << 12;
pub static FT_LOAD_LINEAR_DESIGN               : FT_Int32 = 0x1 << 13;
pub static FT_LOAD_NO_AUTOHINT                 : FT_Int32 = 0x1 << 15;

pub static FT_Err_Ok                            : FT_Error = 0;
pub static FT_Err_Cannot_Open_Resource          : FT_Error = 1;
pub static FT_Err_Unknown_File_Format           : FT_Error = 2;
pub static FT_Err_Invalid_File_Format           : FT_Error = 3;
pub static FT_Err_Invalid_Version               : FT_Error = 4;
pub static FT_Err_Lower_Module_Version          : FT_Error = 5;
pub static FT_Err_Invalid_Argument              : FT_Error = 6;
pub static FT_Err_Unimplemented_Feature         : FT_Error = 7;
pub static FT_Err_Invalid_Table                 : FT_Error = 8;
pub static FT_Err_Invalid_Offset                : FT_Error = 9;
pub static FT_Err_Array_Too_Large               : FT_Error = 10;
pub static FT_Err_Invalid_Glyph_Index           : FT_Error = 16;
pub static FT_Err_Invalid_Character_Code        : FT_Error = 17;
pub static FT_Err_Invalid_Glyph_Format          : FT_Error = 18;
pub static FT_Err_Cannot_Render_Glyph           : FT_Error = 19;
pub static FT_Err_Invalid_Outline               : FT_Error = 20;
pub static FT_Err_Invalid_Composite             : FT_Error = 21;
pub static FT_Err_Too_Many_Hints                : FT_Error = 22;
pub static FT_Err_Invalid_Pixel_Size            : FT_Error = 23;
pub static FT_Err_Invalid_Handle                : FT_Error = 32;
pub static FT_Err_Invalid_Library_Handle        : FT_Error = 33;
pub static FT_Err_Invalid_Driver_Handle         : FT_Error = 34;
pub static FT_Err_Invalid_Face_Handle           : FT_Error = 35;
pub static FT_Err_Invalid_Size_Handle           : FT_Error = 36;
pub static FT_Err_Invalid_Slot_Handle           : FT_Error = 37;
pub static FT_Err_Invalid_CharMap_Handle        : FT_Error = 38;
pub static FT_Err_Invalid_Cache_Handle          : FT_Error = 39;
pub static FT_Err_Invalid_Stream_Handle         : FT_Error = 40;
pub static FT_Err_Too_Many_Drivers              : FT_Error = 48;
pub static FT_Err_Too_Many_Extensions           : FT_Error = 49;
pub static FT_Err_Out_Of_Memory                 : FT_Error = 64;
pub static FT_Err_Unlisted_Object               : FT_Error = 65;
pub static FT_Err_Cannot_Open_Stream            : FT_Error = 81;
pub static FT_Err_Invalid_Stream_Seek           : FT_Error = 82;
pub static FT_Err_Invalid_Stream_Skip           : FT_Error = 83;
pub static FT_Err_Invalid_Stream_Read           : FT_Error = 84;
pub static FT_Err_Invalid_Stream_Operation      : FT_Error = 85;
pub static FT_Err_Invalid_Frame_Operation       : FT_Error = 86;
pub static FT_Err_Nested_Frame_Access           : FT_Error = 87;
pub static FT_Err_Invalid_Frame_Read            : FT_Error = 88;
pub static FT_Err_Raster_Uninitialized          : FT_Error = 96;
pub static FT_Err_Raster_Corrupted              : FT_Error = 97;
pub static FT_Err_Raster_Overflow               : FT_Error = 98;
pub static FT_Err_Raster_Negative_Height        : FT_Error = 99;
pub static FT_Err_Too_Many_Caches               : FT_Error = 112;
pub static FT_Err_Invalid_Opcode                : FT_Error = 128;
pub static FT_Err_Too_Few_Arguments             : FT_Error = 129;
pub static FT_Err_Stack_Overflow                : FT_Error = 130;
pub static FT_Err_Code_Overflow                 : FT_Error = 131;
pub static FT_Err_Bad_Argument                  : FT_Error = 132;
pub static FT_Err_Divide_By_Zero                : FT_Error = 133;
pub static FT_Err_Invalid_Reference             : FT_Error = 134;
pub static FT_Err_Debug_OpCode                  : FT_Error = 135;
pub static FT_Err_ENDF_In_Exec_Stream           : FT_Error = 136;
pub static FT_Err_Nested_DEFS                   : FT_Error = 137;
pub static FT_Err_Invalid_CodeRange             : FT_Error = 138;
pub static FT_Err_Execution_Too_Long            : FT_Error = 139;
pub static FT_Err_Too_Many_Function_Defs        : FT_Error = 140;
pub static FT_Err_Too_Many_Instruction_Defs     : FT_Error = 141;
pub static FT_Err_Table_Missing                 : FT_Error = 142;
pub static FT_Err_Horiz_Header_Missing          : FT_Error = 143;
pub static FT_Err_Locations_Missing             : FT_Error = 144;
pub static FT_Err_Name_Table_Missing            : FT_Error = 145;
pub static FT_Err_CMap_Table_Missing            : FT_Error = 146;
pub static FT_Err_Hmtx_Table_Missing            : FT_Error = 147;
pub static FT_Err_Post_Table_Missing            : FT_Error = 148;
pub static FT_Err_Invalid_Horiz_Metrics         : FT_Error = 149;
pub static FT_Err_Invalid_CharMap_Format        : FT_Error = 150;
pub static FT_Err_Invalid_PPem                  : FT_Error = 151;
pub static FT_Err_Invalid_Vert_Metrics          : FT_Error = 152;
pub static FT_Err_Could_Not_Find_Context        : FT_Error = 153;
pub static FT_Err_Invalid_Post_Table_Format     : FT_Error = 154;
pub static FT_Err_Invalid_Post_Table            : FT_Error = 155;
pub static FT_Err_Syntax_Error                  : FT_Error = 160;
pub static FT_Err_Stack_Underflow               : FT_Error = 161;
pub static FT_Err_Ignore                        : FT_Error = 162;
pub static FT_Err_No_Unicode_Glyph_Name         : FT_Error = 163;
pub static FT_Err_Missing_Startfont_Field       : FT_Error = 176;
pub static FT_Err_Missing_Font_Field            : FT_Error = 177;
pub static FT_Err_Missing_Size_Field            : FT_Error = 178;
pub static FT_Err_Missing_Fontboundingbox_Field : FT_Error = 179;
pub static FT_Err_Missing_Chars_Field           : FT_Error = 180;
pub static FT_Err_Missing_Startchar_Field       : FT_Error = 181;
pub static FT_Err_Missing_Encoding_Field        : FT_Error = 182;
pub static FT_Err_Missing_Bbx_Field             : FT_Error = 183;
pub static FT_Err_Bbx_Too_Big                   : FT_Error = 184;
pub static FT_Err_Corrupted_Font_Header         : FT_Error = 185;
pub static FT_Err_Corrupted_Font_Glyphs         : FT_Error = 186;
pub static FT_Err_Max                           : FT_Error = 187;

// Objects
pub type FT_Face = *FT_FaceRec;
pub type FT_GlyphSlot = *FT_GlyphSlotRec;
pub type FT_SubGlyph = *FT_SubGlyphRec;
pub type FT_CharMap = *FT_CharMapRec;
pub type FT_Size = *FT_SizeRec;
pub type FT_Size_Internal = *FT_Size_InternalRec;
pub type FT_Face_Internal = *FT_Face_InternalRec;
pub type FT_Slot_Internal = *FT_Slot_InternalRec;
pub type FT_Driver = *FT_DriverRec;
pub type FT_Library = *FT_LibraryRec;
pub type FT_Stream = *FT_StreamRec;
pub type FT_Module = *FT_ModuleRec;
pub type FT_Memory = *FT_MemoryRec;
pub type FT_ListNode = *FT_ListNodeRec;

// Internal Types
pub type FT_LibraryRec = c_void;
pub type FT_DriverRec = c_void;
pub type FT_ModuleRec = c_void;
pub type FT_SubGlyphRec = c_void;
pub type FT_Size_InternalRec = c_void;
pub type FT_Face_InternalRec = c_void;
pub type FT_Slot_InternalRec = c_void;

pub struct FT_FaceRec {
    pub num_faces: FT_Long,
    pub face_index: FT_Long,

    pub face_flags: FT_Long,
    pub style_flags: FT_Long,

    pub num_glyphs: FT_Long,

    pub family_name: *FT_String,
    pub style_name: *FT_String,

    pub num_fixed_sizes: FT_Int,
    pub available_sizes: *FT_Bitmap_Size,

    pub num_charmaps: FT_Int,
    pub charmaps: *FT_CharMap,

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
    pub extensions: *c_void,

    pub internal: FT_Face_Internal,
    /* @private end */
}

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

    pub control_data: *c_void,
    pub control_len: c_long,

    pub lsb_delta: FT_Pos,
    pub rsb_delta: FT_Pos,

    pub other: *c_void,

    pub internal: FT_Slot_Internal,
}

pub struct FT_CharMapRec {
    pub face: FT_Face,
    pub encoding: FT_Encoding,
    pub platform_id: FT_UShort,
    pub encoding_id: FT_UShort,
}

pub struct FT_SizeRec {
    pub face: FT_Face,
    pub generic: FT_Generic,
    pub metrics: FT_Size_Metrics,
    pub internal: FT_Size_Internal,
}

pub struct FT_StreamRec {
    pub base: *c_uchar,
    pub size: c_ulong,
    pub pos: c_ulong,

    pub descriptor: FT_StreamDesc,
    pub pathname: FT_StreamDesc,
    pub read: FT_Stream_IoFunc,
    pub close: FT_Stream_CloseFunc,

    pub memory: FT_Memory,
    pub cursor: *c_uchar,
    pub limit: *c_uchar,
}

pub struct FT_MemoryRec {
    pub user: *c_void,
    pub alloc: FT_Alloc_Func,
    pub free: FT_Free_Func,
    pub realloc: FT_Realloc_Func,
}

pub struct FT_ListRec {
    pub head: FT_ListNode,
    pub tail: FT_ListNode,
}

pub struct FT_ListNodeRec {
    pub prev: FT_ListNode,
    pub next: FT_ListNode,
    pub data: *c_void,
}

#[link(name = "freetype")]
extern "C" {
    pub fn FT_Init_FreeType(alibrary: *FT_Library) -> FT_Error;
    pub fn FT_Done_FreeType(library: FT_Library) -> FT_Error;
    pub fn FT_New_Face(library: FT_Library, filepathname: *u8, face_index: FT_Long, aface: *FT_Face) -> FT_Error;
    pub fn FT_New_Memory_Face(library: FT_Library, file_base: *FT_Byte, file_size: FT_Long, face_index: FT_Long, aface: *FT_Face) -> FT_Error;
    pub fn FT_Open_Face(library: FT_Library, args: *FT_Open_Args, face_index: FT_Long, aface: *FT_Face) -> FT_Error;
    pub fn FT_Done_Face(face: FT_Face) -> FT_Error;
    pub fn FT_Set_Char_Size(face: FT_Face, char_width: FT_F26Dot6, char_height: FT_F26Dot6, horz_resolution: FT_UInt, vert_resolution: FT_UInt) -> FT_Error;
    pub fn FT_Set_Pixel_Sizes(face: FT_Face, pixel_width: FT_UInt, pixel_height: FT_UInt) -> FT_Error;
    pub fn FT_Get_Char_Index(face: FT_Face, charcode: FT_ULong) -> FT_UInt;
    pub fn FT_Load_Glyph(face: FT_Face, glyph_index: FT_UInt, load_flags: FT_Int32) -> FT_Error;
    pub fn FT_Load_Char(face: FT_Face, char_code: FT_ULong, load_flags: FT_Int32) -> FT_Error;
    pub fn FT_Render_Glyph(slot: FT_GlyphSlot, render_mode: FT_Render_Mode) -> FT_Error;
    pub fn FT_Select_CharMap(face: FT_Face, encoding: FT_Encoding) -> FT_Error;
    pub fn FT_Set_Transform(face: FT_Face, matrix: *FT_Matrix, delta: *FT_Vector);
}

