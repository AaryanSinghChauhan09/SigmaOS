## SigmaOS: HSN code prefix, e.g. "0401" = milk */
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
  GSTSlab* = object
    rate*: SigmaU64

type
  GSTCalcResult* = object
    taxable_value*: SigmaU64
    cgst*: SigmaU64
    sgst*: SigmaU64
    igst*: SigmaU64
    total_tax*: SigmaU64
    invoice_total*: SigmaU64
    rate*: SigmaU64

