use memflow::prelude::v1::*;

use super::*;

pub type SchemaClassBinding = SchemaClassInfoData;

#[rustfmt::skip]
#[derive(Pod)]
#[repr(C)]
pub struct SchemaClassInfoData {
    pub base: Pointer64<SchemaClassInfoData>,                  // 0x0000
    pub name: Pointer64<ReprCString>,                          // 0x0008
    pub module_name: Pointer64<ReprCString>,                   // 0x0010
    pub size: i32,                                             // 0x0018
    pub field_count: u16,                                      // 0x001C
    pub static_field_count: u16,                               // 0x001E
    pub static_metadata_count: u16,                            // 0x0020
    pub alignment: u8,                                         // 0x0022
    pub base_class_count: u8,                                  // 0x0023
    pub multiple_inheritance_depth: u16,                       // 0x0024
    pub single_inheritance_depth: u16,                         // 0x0026
    pub fields: Pointer64<[SchemaClassFieldData]>,             // 0x0028
    pub static_fields: Pointer64<[SchemaStaticFieldData]>,     // 0x0030
    pub base_classes: Pointer64<SchemaBaseClassInfoData>,      // 0x0038
    pad_0040: [u8; 0x8],                                       // 0x0040
    pub static_metadata: Pointer64<[SchemaMetadataEntryData]>, // 0x0048
    pub type_scope: Pointer64<SchemaSystemTypeScope>,          // 0x0050
    pub schema_type: Pointer64<SchemaType>,                    // 0x0058
    pad_0060: [u8; 0x10],                                      // 0x0060
}
