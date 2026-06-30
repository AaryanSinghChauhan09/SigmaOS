## SigmaOS: SIGMA_ZKP_H */
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
  SigmaZKPProver* = object
    secret*: SigmaI32
    commitment*: SigmaI32
    challenge*: SigmaI32
    response*: SigmaI32

type
  SigmaZKPVerifier* = object
    generator*: SigmaI32
    public_key*: SigmaI32
    commitment*: SigmaI32
    challenge*: SigmaI32

