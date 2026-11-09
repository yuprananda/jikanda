# Copyright (C) 2026 Yu Prananda
# SPDX-License-Identifier: GPL-3.0-only
"""Read PE metadata without loading the executable or requiring extra packages."""
import json
from pathlib import Path
import struct
import sys

path = Path(sys.argv[1] if len(sys.argv) > 1 else "target/release/jikanda.exe")
data = path.read_bytes()
u16 = lambda offset: struct.unpack_from("<H", data, offset)[0]
u32 = lambda offset: struct.unpack_from("<I", data, offset)[0]
assert data[:2] == b"MZ"
pe = u32(0x3c)
assert data[pe:pe+4] == b"PE\0\0"
assert u16(pe+4) == 0x8664, "Expected x64 artifact"
optional = pe+24
assert u16(optional) == 0x20b
sections = optional + u16(pe+20)


def offset(rva):
    for index in range(u16(pe+6)):
        section = sections + 40*index
        size, start, raw_size, raw = struct.unpack_from("<IIII", data, section+8)
        if start <= rva < start + max(size, raw_size):
            return raw + rva - start
    raise ValueError(f"Invalid RVA: {rva}")


imports = []
descriptor = offset(u32(optional+112+8))
while any(data[descriptor:descriptor+20]):
    name = offset(u32(descriptor+12))
    imports.append(data[name:data.index(0, name)].decode("ascii"))
    descriptor += 20
resource = offset(u32(optional+112+16))
types = []
for index in range(u16(resource+12)+u16(resource+14)):
    kind = u32(resource+16+8*index)
    if not kind & 0x80000000:
        types.append(kind)
assert {3, 14, 16}.issubset(types), "Icon/group icon/version resources are required"
print(json.dumps({"file": path.name, "machine": "x86_64", "subsystem": u16(optional+68),
                  "has_icon_and_version_resources": True,
                  "signed": u32(optional+112+4*8+4) > 0,
                  "imported_dlls": sorted(imports)}, indent=2))
