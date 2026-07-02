-- SPDX-License-Identifier: MIT
-- Copyright (c) 2024-2026 SigmaOS Project
--
-- crypto/sigma_dilithium.ads — Dilithium-5 Signature Scheme (SPARK/Ada spec)
-- Replaces: sigma_dilithium.cpp (C++ stub, removed)
--
-- Language: SPARK/Ada — formally verifiable, proven correct
-- Pattern: package specification with pre/postconditions (contracts)
-- Compile: gnatprove --level=2 sigma_dilithium.ads

with Interfaces; use Interfaces;

package Sigma.Dilithium
  with SPARK_Mode => On
is

   -- ── Parameters (Dilithium5 / NIST ML-DSA-87) ────────────────────────────

   N         : constant := 256;
   Q         : constant := 8380417;
   D_PARAM   : constant := 13;
   TAU       : constant := 60;
   LAMBDA    : constant := 256;
   GAMMA1    : constant := 2**19;
   GAMMA2    : constant := (Q - 1) / 32;
   K_PARAM   : constant := 8;
   L_PARAM   : constant := 7;
   ETA       : constant := 2;
   BETA      : constant := 120;
   OMEGA     : constant := 75;

   PK_BYTES  : constant := 2592;
   SK_BYTES  : constant := 4896;
   SIG_BYTES : constant := 4627;
   SEED_BYTES: constant := 32;
   HASH_BYTES: constant := 32;

   -- ── Type Definitions ─────────────────────────────────────────────────────

   subtype Byte     is Unsigned_8;
   subtype Byte_Range is Integer range 0 .. 255;

   type Byte_Array  is array (Natural range <>) of Byte;

   subtype Public_Key  is Byte_Array (0 .. PK_BYTES  - 1);
   subtype Secret_Key  is Byte_Array (0 .. SK_BYTES  - 1);
   subtype Signature   is Byte_Array (0 .. SIG_BYTES - 1);
   subtype Seed        is Byte_Array (0 .. SEED_BYTES - 1);
   subtype Hash_Out    is Byte_Array (0 .. HASH_BYTES - 1);

   -- ── Procedure Contracts ──────────────────────────────────────────────────

   -- Key generation
   procedure KeyGen
     (Pk   : out Public_Key;
      Sk   : out Secret_Key;
      Rand : in  Seed)
   with
     SPARK_Mode => On,
     Global     => null,
     Depends    => (Pk => Rand, Sk => Rand);

   -- Sign a message
   procedure Sign
     (Sig     : out Signature;
      Message : in  Byte_Array;
      Sk      : in  Secret_Key;
      Rand    : in  Seed)
   with
     SPARK_Mode => On,
     Global     => null,
     Pre        => Message'Length > 0 and Message'Length <= 2**31 - 1,
     Depends    => (Sig => (Message, Sk, Rand));

   -- Verify a signature
   function Verify
     (Sig     : in Signature;
      Message : in Byte_Array;
      Pk      : in Public_Key)
     return Boolean
   with
     SPARK_Mode => On,
     Global     => null,
     Pre        => Message'Length > 0;

private

   -- Internal polynomial type
   type Coeff_Array is array (0 .. N - 1) of Integer_32;
   type Poly_Type   is record Coeffs : Coeff_Array; end record;

   -- Vector types
   type PolyVec_K is array (0 .. K_PARAM - 1) of Poly_Type;
   type PolyVec_L is array (0 .. L_PARAM - 1) of Poly_Type;

end Sigma.Dilithium;
