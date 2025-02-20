#[macro_export]
/// The whole point.
///
/// See crate documentation for more information.
macro_rules! sealed_trait {
    (
        $v:vis sealed trait $name:ty: [$($requires:ty),*] permits $($sealed_type:ty)* => $body:tt
    ) => {
        $crate::paste::paste! {
            #[allow(unused)]
            trait [<$name Sealer>] {}
            $(
                impl [<$name Sealer>] for $sealed_type {}
            )*

            #[allow(private_bounds)]
            $v trait $name: [<$name Sealer>] $(+ $requires)* $body
        }
    };

    ($v:vis sealed trait $name:ident permits $($sealed_type:ty)* => $body:tt) => {
        $crate::paste::paste! {
            #[allow(unused)]
            trait [<$name Sealer>] {}
            $(
                impl [<$name Sealer>] for $sealed_type {}
            )*

            #[allow(private_bounds)]
            $v trait $name: [<$name Sealer>] $body
        }
    }
}
