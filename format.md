# File format

|Size (bytes)| Type |         Comment        |
|:----------:|:----:|:----------------------:|
|    0x04    |String|   Magic "\xffv\x7fs"   |
|    0x02    |  u8  |Major version identifier|
|    0x02    |  u8  |Minor version identifier|
|  Variable  |Entry |          Root          |
|  Variable  |------|Ignored. you can use this data area for zip and other.|
