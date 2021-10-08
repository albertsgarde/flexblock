# [derive (Clone)] pub struct OnePoleFilter < S : Module , C : Module > { source : S , coefficient : C , prev_sample : f32 , } impl < S : Module , C : Module , RHS : crate :: modules :: Module > std :: ops :: Add < RHS > for OnePoleFilter < S , C > { type Output = crate :: modules :: Add < OnePoleFilter < S , C > , RHS > ; fn add (self , rhs : RHS) -> Self :: Output { crate :: modules :: Add :: new (self , rhs) } } impl < S : Module , C : Module , RHS : crate :: modules :: Module > std :: ops :: Sub < RHS > for OnePoleFilter < S , C > { type Output = crate :: modules :: Subtract < OnePoleFilter < S , C > , RHS > ; fn sub (self , rhs : RHS) -> Self :: Output { crate :: modules :: Subtract :: new (self , rhs) } } impl < S : Module , C : Module , RHS : crate :: modules :: Module > std :: ops :: Mul < RHS > for OnePoleFilter < S , C > { type Output = crate :: modules :: Multiply < OnePoleFilter < S , C > , RHS > ; fn mul (self , rhs : RHS) -> Self :: Output { crate :: modules :: Multiply :: new (self , rhs) } }