## SigmaOS: SigmaSuite.h — Sovereign Productivity Suite Header
## Migrated from C/C++ to Nim — no stdlib import, no external packages.
## All types hand-defined. OOP via object hierarchy + method dispatch.
{.push raises: [].}

type
  SigmaU8*  = uint8
  SigmaU16* = uint16
  SigmaU32* = uint32
  SigmaU64* = uint64
  SigmaI32* = int32
  SigmaI64* = int64
  SigmaBool* = bool
  SigmaUsize* = uint

type
  TextAlign* = object of RootObj
    initialized*: SigmaBool

proc newTextAlign*(): TextAlign =
  result = TextAlign(initialized: false)

proc open*(self: var TextAlign) =
  self.initialized = true

proc has_next*(self: var TextAlign) =
  self.initialized = true

proc push*(self: var TextAlign) =
  self.initialized = true

proc load*(self: var TextAlign) =
  self.initialized = true

proc save*(self: var TextAlign) =
  self.initialized = true

proc replace_all_text*(self: var TextAlign) =
  self.initialized = true

proc add_row_label*(self: var TextAlign) =
  self.initialized = true

proc add_col_label*(self: var TextAlign) =
  self.initialized = true

proc cell_compare*(self: var TextAlign) =
  self.initialized = true

proc swap_rows*(self: var TextAlign) =
  self.initialized = true

proc eval_filter*(self: var TextAlign) =
  self.initialized = true

proc add_page*(self: var TextAlign) =
  self.initialized = true

proc write_pdf*(self: var TextAlign) =
  self.initialized = true

proc open*(self: var TextAlign) =
  self.initialized = true

proc create*(self: var TextAlign) =
  self.initialized = true

type
  TextRange* = object
    start*: SigmaU32
    end*: SigmaU32

type
  TextFormat* = object
    bold*: SigmaBool
    italic*: SigmaBool
    underline*: SigmaBool
    font_size*: SigmaU64
    color_rgb*: SigmaU32
    align*: SigmaU64
    line_spacing*: SigmaU64

type
  FormatRun* = object
    start*: SigmaU32
    end*: SigmaU32
    bold*: SigmaBool
    italic*: SigmaBool
    underline*: SigmaBool
    font_size*: SigmaU64
    color_rgb*: SigmaU32
    align*: SigmaU64
    line_spacing*: SigmaU64

type
  PageSetup* = object

type
  DocCell* = object

type
  DocTable* = object
    id*: SigmaU64
    rows*: SigmaU32
    cols*: SigmaU32
    position*: SigmaU32

type
  MailMergeField* = object

type
  MailMergeRecord* = object
    field_count*: SigmaU32

type
  CellRange* = object
    row_start*: SigmaU32
    col_start*: SigmaU32
    row_end*: SigmaU32
    col_end*: SigmaU32

type
  Cell* = object
    number*: SigmaU64
    is_formula*: SigmaBool

type
  SheetRow* = object

type
  ActiveFilter* = object
    col*: SigmaU32
    cond*: SigmaU64

type
  PivotTable* = object
    source*: SigmaU64
    row_field*: SigmaU32
    col_field*: SigmaU32
    value_field*: SigmaU32
    agg*: SigmaU64

type
  TextBox* = object
    format*: SigmaU64

type
  ImageElement* = object

type
  VideoElement* = object
    autoplay*: SigmaBool

type
  SlideElement* = object
    type*: SigmaU64
    text_box*: SigmaU64
    image*: SigmaU64
    video*: SigmaU64

type
  Slide* = object
    id*: SigmaU64
    layout*: SigmaU64
    bg_color*: SigmaU32

type
  ColumnDefStub* = object
    type*: SigmaI32

type
  TableSchemaStub* = object
    column_count*: SigmaU32

type
  ResultSetStub* = object

type
  ParsedQuery* = object

type
  FormField* = object
    column*: SigmaU64
    input_type*: SigmaU64

type
  Form* = object

var instance* = newTextAlign()

proc push*() {.exportc.} =
  instance.initialized = true

proc replace_all_text*() {.exportc.} =
  instance.initialized = true

proc add_row_label*() {.exportc.} =
  instance.initialized = true

proc add_col_label*() {.exportc.} =
  instance.initialized = true

proc swap_rows*() {.exportc.} =
  instance.initialized = true

proc add_page*() {.exportc.} =
  instance.initialized = true

