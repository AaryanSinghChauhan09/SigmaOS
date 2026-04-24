(* ==========================================================================
   Σ SIGMAOS: IPC ISOLATION — COQ PROOF SKETCH
   ==========================================================================
   Proves that the IPC channel isolation invariant holds:
   No two distinct processes can share the same IPC mailbox region.
   
   This is a proof sketch. Fill in axioms from the real memory model
   once the physical memory layout is finalized.
   ========================================================================== *)

(* --- Definitions --- *)

Definition ShardId := nat.
Definition PhysAddr := nat.

Record IpcChannel := {
  owner   : ShardId;
  base    : PhysAddr;
  size    : nat;
}.

(* Two channels are disjoint if their physical ranges do not overlap *)
Definition disjoint (c1 c2 : IpcChannel) : Prop :=
  c1.(base) + c1.(size) <= c2.(base) \/
  c2.(base) + c2.(size) <= c1.(base).

(* --- Isolation Invariant --- *)

(* All channels in a well-formed system are pairwise disjoint *)
Definition isolated (channels : list IpcChannel) : Prop :=
  forall c1 c2, 
    In c1 channels -> In c2 channels -> 
    c1.(owner) <> c2.(owner) -> 
    disjoint c1 c2.

(* --- Lemma: Disjoint ranges prevent overlap --- *)

Lemma disjoint_no_overlap : forall c1 c2 : IpcChannel,
  disjoint c1 c2 ->
  forall addr,
    c1.(base) <= addr < c1.(base) + c1.(size) ->
    ~ (c2.(base) <= addr < c2.(base) + c2.(size)).
Proof.
  intros c1 c2 H addr H1 H2.
  unfold disjoint in H.
  destruct H as [H | H]; omega.
Qed.

(* 
   TODO (Phase 2): 
   - Instantiate with real physical memory map from sigma_dma.c
   - Prove that DMA allocator preserves the isolation invariant after alloc/free
   - Extend to cover IPC + DMA cross-region non-interference
*)
