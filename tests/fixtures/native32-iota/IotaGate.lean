import Tutorial.Meta
good_def boolGood : @Bool.rec (fun _ => Type) (Prop → Prop) Prop Bool.false :=
  fun p : Prop => p
bad_def boolBad : @Bool.rec (fun _ => Type) (Prop → Prop) Prop Bool.true :=
  unchecked (fun p : Prop => p)
good_def boolOther : @Bool.rec (fun _ => Type) (Prop → Prop) Prop Bool.true :=
  ∀ p : Prop, p
good_def fieldGood : @Prod.rec Type Type (fun _ => Type)
    (fun a b => a) (Prod.mk (Prop → Prop) Prop) := fun p : Prop => p
bad_def fieldBad : @Prod.rec Type Type (fun _ => Type)
    (fun a b => b) (Prod.mk (Prop → Prop) Prop) := unchecked (fun p : Prop => p)
good_def surplusGood : (@Bool.rec (fun _ => Type → Type) (fun A => A)
    (fun _ => Prop) Bool.false) (Prop → Prop) := fun p : Prop => p
bad_def surplusBad : (@Bool.rec (fun _ => Type → Type) (fun A => A)
    (fun _ => Prop) Bool.true) (Prop → Prop) := unchecked (fun p : Prop => p)
