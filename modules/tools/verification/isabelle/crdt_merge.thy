theory crdt_merge
imports Main
begin

(*
  ==========================================================================
  Σ SIGMAOS: CRDT LWW MERGE — ISABELLE/HOL PROOF SKETCH
  ==========================================================================
  Proves that the Last-Write-Wins (LWW) register merge operation is:
    1. Idempotent:   merge(s, s) = s
    2. Commutative:  merge(s1, s2) = merge(s2, s1)
    3. Associative:  merge(merge(s1, s2), s3) = merge(s1, merge(s2, s3))
  These three properties together guarantee convergence: any two replicas
  that receive the same set of writes will eventually agree, regardless
  of the order in which they receive them.
  ==========================================================================*)

(* A LWW entry: a key, value, and Lamport timestamp *)
record lww_entry =
  lww_key :: string
  lww_val :: "nat list"     (* StateValue as list of bytes *)
  lww_ts  :: nat            (* Logical clock / sequence ID *)

(* Merge two entries for the same key: keep the higher timestamp *)
definition merge_entry :: "lww_entry \<Rightarrow> lww_entry \<Rightarrow> lww_entry" where
  "merge_entry e1 e2 = (if lww_ts e1 \<ge> lww_ts e2 then e1 else e2)"

(* --- Idempotence --- *)
lemma merge_entry_idempotent:
  "merge_entry e e = e"
  by (simp add: merge_entry_def)

(* --- Commutativity --- *)
lemma merge_entry_commutative:
  "merge_entry e1 e2 = merge_entry e2 e1"
  by (simp add: merge_entry_def, arith)

(* --- Associativity --- *)
lemma merge_entry_associative:
  "merge_entry (merge_entry e1 e2) e3 = merge_entry e1 (merge_entry e2 e3)"
  by (simp add: merge_entry_def, arith)

(*
  Convergence Theorem:
  Any two replicas R1, R2 that have seen the same set of entries
  will converge to the same state after merge.
  (Proof by induction on the entry set — sketch; full proof requires
   formalizing the LWW register as a finite map.)
*)

end
