| Bytes | Pattern                               | Meaning         |
|-------|---------------------------------------|-----------------|
| 1     | `0xxxxxxx`                            | ASCII           |
| 2     | `110xxxxx 10xxxxxx`                   | 2-byte sequence |
| 3     | `1110xxxx 10xxxxxx 10xxxxxx`          | 3-byte sequence |
| 4     | `11110xxx 10xxxxxx 10xxxxxx 10xxxxxx` | 4-byte sequence |
