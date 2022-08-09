# File format

```Rust
type ZtString = String; // Zero terminated string
enum ItemData {
  File(Vec<u8>),
  Folder(Vec<Item>), // terminator: 0xff
  URLFile(ZtString),
  Shortcut(ZtString),
  ZipArchive(Vec<u8>),
  TarArchive(Vec<u8>)
}
struct Item {
  utime: u64,
  name: ZtString,
  kind: u8,
  data: ItemData
}
struct File {
  magic: [u8; 4], //{0xff, 'v', 0x7f, 's'}
  major: u16,
  minor: u16,
  reserved: [u32; 2],
  Item rootfs;
  // And other datas;
}
```
