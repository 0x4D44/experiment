"""
Binary Ninja custom loader + helpers for F1GP GP.EXE.
Expected usage:
    bn -t f1gp -s re/binary_ninja/plugins/f1gp_loader.py f1gp-disasm/GP.EXE
"""

import json
import os
from typing import Any, Dict

try:
    import yaml  # type: ignore
except ImportError:  # pragma: no cover
    yaml = None

from binaryninja import (  # type: ignore
    Architecture,
    BinaryView,
    BinaryViewType,
    SegmentFlag,
    SectionSemantics,
    log_error,
    log_info,
)

SEGMENTS_REL_PATH = os.path.join("re", "artifacts", "segments.yml")


def load_segments(repo_root: str) -> Dict[str, Any]:
    path = os.path.join(repo_root, SEGMENTS_REL_PATH)
    if not os.path.exists(path):
        raise FileNotFoundError(path)
    with open(path, "r", encoding="utf-8") as fh:
        if yaml:
            return yaml.safe_load(fh)
        data = fh.read()
    # naive YAML fallback (placeholder) – prefer PyYAML
    return json.loads(data)


class F1GPView(BinaryView):
    name = "F1GP"
    long_name = "F1 Grand Prix 16-bit"

    def __init__(self, data):
        BinaryView.__init__(self, parent_view=data, file_metadata=data.file)
        self.data = data
        self.repo_root = os.getenv("F1GP_REPO_ROOT", os.getcwd())
        self.segments_doc = None

    @classmethod
    def is_valid_for_data(cls, data):
        if len(data) < 2:
            return False
        magic = data.read(0, 2)
        return magic == b"MZ"

    def init(self):
        try:
            self.segments_doc = load_segments(self.repo_root)
        except Exception as exc:  # pragma: no cover
            log_error(f"Failed loading segments.yml: {exc}")
            return False

        arch = Architecture["x86"]
        self.platform = arch.standalone_platform
        self._apply_segments()
        self._add_entry_point()
        log_info("F1GP Binary Ninja view initialized")
        return True

    def perform_is_executable(self):
        return True

    def perform_get_address_size(self):
        return 2

    def _apply_segments(self):
        if not self.segments_doc:
            return
        for seg in self.segments_doc.get("segments", []):
            start = seg["start"]
            if isinstance(start, str):
                start = int(start, 16)
            size = int(seg["size"])
            data = self.parent_view.read(start, size)
            if data is None:
                data = b"\x00" * size
            self.write(start, data)
            flags = SegmentFlag.SegmentReadable
            if "X" in seg.get("permissions", ""):
                flags |= SegmentFlag.SegmentExecutable
            if "W" in seg.get("permissions", ""):
                flags |= SegmentFlag.SegmentWritable
            self.add_auto_segment(start, size, start, size, flags)
            self.add_auto_section(
                seg.get("name", f"seg_{start:x}"),
                start,
                size,
                SectionSemantics.ReadOnlyCodeSectionSemantics,
            )

    def _add_entry_point(self):
        header = self.parent_view.read(0x18, 4)
        if not header:
            return
        ip = int.from_bytes(header[:2], "little")
        cs = int.from_bytes(header[2:], "little")
        linear = (cs << 4) + ip
        self.add_entry_point(linear)


class F1GPViewType(BinaryViewType):
    name = F1GPView.name
    long_name = F1GPView.long_name

    def create(self, data):
        return F1GPView(data)

    def is_valid_for_data(self, data):
        return F1GPView.is_valid_for_data(data)


F1GPViewType.register()
