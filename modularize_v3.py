"""
SigmaOS Modular Shim
"""
from ._shards.safe_write import safe_write
from ._shards.make_init import make_init
from ._shards.collect_imports import collect_imports
from ._shards.source_of import source_of
from ._shards.dedent_src import dedent_src
from ._shards.node_header_comment import node_header_comment
from ._shards.get_top_imports import get_top_imports
from ._shards.get_top_constants import get_top_constants
from ._shards.split_file import split_file
from ._shards.main import main
