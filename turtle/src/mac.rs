// macro_rules! threshold {
//     // $b: AtomicBool
//     ($b:expr, $do:block) => ({
//         if $b.load(Ordering::Relaxed) {
//             $do;
//             $b.store(false, Ordering::Relaxed);
//         } else {
//             log::warn!("{} 在途请求.", stringify!($do));
//         }
//     })
// }
//
